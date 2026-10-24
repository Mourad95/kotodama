#!/usr/bin/env bash
# Builds the Rust core (kd-ffi) for Android and generates the UniFFI Kotlin bindings.
#
#   apps/android/build-core.sh [--debug]
#
# Outputs (both gitignored, regenerated on every run):
#   app/src/main/jniLibs/<abi>/libkd_ffi.so          stripped, 16 KB page aligned
#   app/src/main/generated/uniffi/kd_ffi/kd_ffi.kt    Kotlin bindings (add as a Kotlin source dir)
#
# Because the bindings are generated from the compiled library, a Rust signature change
# changes the Kotlin API and breaks the Kotlin build of any caller that is not updated.
#
# Requirements: Rust with the aarch64-linux-android and x86_64-linux-android targets,
# `cargo install cargo-ndk --locked`, and an Android NDK (ANDROID_NDK_HOME, or
# $ANDROID_SDK_ROOT/ndk/<version>, default SDK root /opt/android-sdk).
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
app="$root/apps/android/app/src/main"
profile=release
profile_flag=--release
if [[ "${1:-}" == "--debug" ]]; then profile=debug; profile_flag=; fi

sdk="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-/opt/android-sdk}}"
if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
  ANDROID_NDK_HOME="$(ls -d "$sdk"/ndk/*/ 2>/dev/null | sort -V | tail -1 || true)"
  ANDROID_NDK_HOME="${ANDROID_NDK_HOME%/}"
fi
[[ -d "$ANDROID_NDK_HOME" ]] || { echo "Android NDK not found (set ANDROID_NDK_HOME)" >&2; exit 1; }
export ANDROID_NDK_HOME
command -v cargo-ndk >/dev/null || { echo "cargo-ndk missing: cargo install cargo-ndk --locked" >&2; exit 1; }

target_dir="${CARGO_TARGET_DIR:-$root/target}"
strip_tool="$(ls "$ANDROID_NDK_HOME"/toolchains/llvm/prebuilt/*/bin/llvm-strip 2>/dev/null | head -1 || true)"
readelf_tool="$(ls "$ANDROID_NDK_HOME"/toolchains/llvm/prebuilt/*/bin/llvm-readelf 2>/dev/null | head -1 || true)"
[[ -x "$strip_tool" ]] || { echo "llvm-strip not found under $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/*/bin (incomplete NDK install?)" >&2; exit 1; }
[[ -x "$readelf_tool" ]] || { echo "llvm-readelf not found under $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/*/bin (incomplete NDK install?)" >&2; exit 1; }

# Google Play requires 16 KB page size support for apps targeting Android 15+.
android_rustflags="${RUSTFLAGS:-} -C link-arg=-Wl,-z,max-page-size=16384"

cd "$root"
rm -rf "$app/jniLibs"
RUSTFLAGS="$android_rustflags" cargo ndk -t arm64-v8a -t x86_64 --platform 26 -o "$app/jniLibs" \
  build -p kd-ffi --lib $profile_flag

# Bindings are generated from the UNSTRIPPED library (UniFFI reads its metadata symbols).
rm -rf "$app/generated/uniffi"
cargo run -q -p kd-ffi --features bindgen --bin uniffi-bindgen -- generate \
  --library "$target_dir/aarch64-linux-android/$profile/libkd_ffi.so" \
  --language kotlin --no-format --out-dir "$app/generated"

for abi in arm64-v8a x86_64; do
  so="$app/jniLibs/$abi/libkd_ffi.so"
  "$strip_tool" --strip-unneeded "$so"
  # Every LOAD segment must be aligned to 0x4000 (16 KB).
  bad="$("$readelf_tool" -lW "$so" | awk '$1=="LOAD" && $NF!="0x4000" {print $NF}')"
  [[ -z "$bad" ]] || { echo "$abi: LOAD segment not 16 KB aligned ($bad)" >&2; exit 1; }
  echo "$abi: $(du -h "$so" | cut -f1), LOAD segments aligned to 0x4000"
done
echo "Kotlin bindings: $app/generated/uniffi/kd_ffi/kd_ffi.kt"
