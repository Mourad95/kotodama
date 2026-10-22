# Android shell

Thin Kotlin shell around the Rust core (ADR-0002). Added in milestone M2 (device spike):

- app module (Jetpack Compose) calling `lt-ffi` through UniFFI-generated Kotlin bindings;
- the Rust core built with `cargo-ndk` for `arm64-v8a` (and `x86_64` for the emulator);
- later: IME keyboard (M6) inserting animated WebP/GIF through the commit content API.

No business logic here: anything shared with iOS belongs in `crates/`.
