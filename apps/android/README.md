# Android shell

Thin Kotlin shell around the Rust core (ADR-0002). Added in milestone M2 (device spike):

- app module (Jetpack Compose) calling `kd-ffi` through UniFFI-generated Kotlin bindings;
- the Rust core built with `cargo-ndk` for `arm64-v8a` (and `x86_64` for the emulator);
- later: IME keyboard (M6) inserting animated WebP/GIF through the commit content API.

No business logic here: anything shared with iOS belongs in `crates/`.

## The M2 spike app (`app.kotomotion.spike`)

A measurement tool, not product UI. One screen: device model + Android version + ABI + build type,
the model directory, the cold-start time of `EmotionEngine.open`, a text field with an **Analyse**
button (7 probabilities, effective emotion, latency of that call) and a **Benchmark** button.

Privacy by construction (`docs/legal.md` L1, L2, L18-L20): no `INTERNET` or other network
permission (the only `uses-permission` in the merged debug and release manifests is androidx's
non-exported-receiver signature permission `app.kotomotion.spike.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`), `allowBackup="false"` with `dataExtractionRules` / `fullBackupContent`
excluding everything, no analytics or crash SDK, message text is never logged or persisted.
`gradle verifyPrivacy` (run by `check` and after every `assemble*`) uses an allowlist: it fails on
any `uses-permission` other than that signature permission, or if `allowBackup="false"`,
`dataExtractionRules` or `fullBackupContent` is missing, in the source, debug and release merged manifests.

## Prerequisites

- JDK 17+ (built with 21), Android SDK with platform `android-36` and build-tools `36.0.0`
  (`ANDROID_HOME`, default `/opt/android-sdk`), Android NDK (any recent r2x).
- Rust with the `aarch64-linux-android` and `x86_64-linux-android` targets and
  `cargo install cargo-ndk --locked` (see `build-core.sh`).
- Network access to Google Maven / Maven Central for the first build.

## Build

```bash
cd apps/android
./gradlew assembleDebug        # runs build-core.sh, generates the bench messages, builds the APK
./gradlew verifyPrivacy lint   # privacy gate + Android lint
```

`app/build/outputs/apk/debug/app-debug.apk` is the output. The Gradle task `buildCore` runs
`build-core.sh` before every build (up to date when `crates/**` and the lockfile are unchanged), so
a Rust signature change regenerates `src/main/generated/` and breaks `compileKotlin`.
`generateBenchMessages` writes `bench_messages.txt` into the APK assets from the `text` column of
`research/emotion-benchmark/dev_v1.csv` (the dev set; **never** `test_v2.csv`).

Small disk? Put the caches and outputs elsewhere:

```bash
export GRADLE_USER_HOME=/dev/shm/gradle-home GRADLE_OPTS=-Dorg.gradle.daemon=false
export CARGO_TARGET_DIR=/dev/shm/target KD_ANDROID_BUILD_DIR=/dev/shm/kd-android-build
```

The Gradle wrapper is committed (Gradle 8.14.3, distribution SHA-256 pinned); system `gradle` 8.14.x works too.

### 16 KB page-size check (manual, not a CI dependency)

```bash
$ANDROID_HOME/build-tools/36.0.0/zipalign -P 16 -c -v 4 app/build/outputs/apk/release/app-release-unsigned.apk | grep '\.so'
```

`libkd_ffi.so` is linked with 16 KB LOAD alignment (checked by `build-core.sh`). With
`useLegacyPackaging` the libraries are stored compressed and extracted at install, so zipalign
reports them `OK - compressed` (its 16 KB check applies to uncompressed storage only).
`libonnxruntime.so` and `libjnidispatch.so` come from third-party AARs: their ELF alignment is not
re-checked here.

## Pushing the model bundle

The ~116 MB bundle is not in the APK for the spike. The app reads
`getExternalFilesDir(null)/bundle`, shown on screen. Install the app once (so the directory exists),
then:

```bash
adb install -r app/build/outputs/apk/debug/app-debug.apk
adb push bundle/ /sdcard/Android/data/app.kotomotion.spike/files/bundle
```

(`bundle/` is the output of `tools/model-export`: `manifest.json`, `model_q4.onnx`, `tokenizer.json`,
`emotion_head.json`, `parity.json`.) If `manifest.json` is missing or the bundle fails to load, the
screen says so and the app falls back to "calme" (no crash). Restart the app after pushing.
`/data/local/tmp` is not readable by apps, hence the app-specific external directory.
ONNX Runtime comes from `com.microsoft.onnxruntime:onnxruntime-android`; `useLegacyPackaging` makes
`nativeLibraryDir/libonnxruntime.so` a real file whose path is given to `EmotionEngine.open`.

## Reading the benchmark numbers

Use a **release-like** run on a cool device, screen on, not charging if possible. Press
**Benchmark** twice and keep the second result (the first includes page-cache effects).

- `cold start (open)`: wall-clock of `EmotionEngine.open` (SHA-256 of the whole bundle + model
  load). Compare with the "first launch" budget, not the per-message one.
- `n`, `p50`, `p95`, `max`: per-call latency of `analyze` over the `n` dev messages after 5
  warm-up calls (nearest-rank percentiles; one message at a time on one background thread). The
  M2 exit criterion is **p95 <= 200 ms** on a mid-range phone.
- Memory, read after the run: native heap (`Debug.getNativeHeapAllocatedSize`), Java heap
  (`Runtime`), and `VmHWM` / `VmRSS` from `/proc/self/status`. **VmHWM is the peak** to compare with
  the <= 300 MB criterion (it includes the mapped model and the Compose/ART baseline).
- Take a screenshot: the device line makes it self-explanatory. Also record app size impact from
  the release APK/AAB.

## What "verified on device" means

A number or behaviour is *verified on device* only when it was produced by this app running on a
physical phone (not an emulator, not the desktop engine), with the device model, Android version
and build type visible on the screenshot, on at least one recent and one mid-range phone. The
M2 build here was produced in a container with no device or emulator: it compiles, packages and
passes the privacy gate and lint, but is **not verified on device**. Last successful run
(2026-10-22): `./gradlew assembleDebug assembleRelease lint check`.
