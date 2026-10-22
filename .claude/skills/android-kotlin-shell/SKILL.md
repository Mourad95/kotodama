---
name: android-kotlin-shell
description: Load when working in apps/android — the Jetpack Compose app, the IME keyboard service, inserting animated WebP/GIF through the commit-content API, building the Rust core with cargo-ndk, UniFFI Kotlin bindings, model asset delivery, permissions or Google Play requirements for a kids-oriented keyboard.
---

# Android shell

A thin Kotlin shell around the Rust core (ADR-0002). No business logic here. Google Play and Android
requirements change; **check current documentation before relying on any detail below.**

## Targets
1. App (Jetpack Compose): onboarding, composer, theme picker, settings, learning feedback.
2. IME keyboard (M6): an `InputMethodService`, the first keyboard we ship.

## Building the core
- Build `kd-ffi` with `cargo-ndk` for `arm64-v8a` (plus `x86_64` for the emulator).
- UniFFI Kotlin bindings load the library through JNA: package the JNA dependency correctly and
  keep the library name stable.
- Native libraries must satisfy Google Play's page-size requirement (16 KB alignment for recent
  target API levels): verify with the current policy and fix in the linker flags.
- Regenerate bindings in CI; a Rust signature change must fail the Kotlin build.

## IME keyboard
- Extends `InputMethodService`; keep the service light, call the core on a background dispatcher,
  debounce analysis on typing pauses (~300 ms), and never block the input thread.
- Compose inside an IME needs the lifecycle/saved-state owners set up on the host view; test
  rotation, split screen and dark mode.
- **Detect password and sensitive fields** from `EditorInfo.inputType` and disable analysis, learning
  feedback and any recording there. Never log or persist typed text.
- **Inserting animations:** use the commit-content API (`InputConnectionCompat.commitContent`) with
  a `FileProvider` content URI and read-permission grant. Only send a MIME type the target field
  declares in `EditorInfoCompat.getContentMimeTypes`; otherwise fall back to a share intent or the
  link. Not every app accepts images: test the apps teenagers actually use.

## Privacy by construction
- **Do not declare the `INTERNET` permission** in the keyboard app if the design allows it: it is a
  verifiable proof that messages cannot leave the phone (ADR-0007). If any feature needs the
  network, isolate it and document why (gate L4).
- Data safety form and Families-related policies must match the code (gates L4, L6). No ads or
  analytics SDK.

## Model delivery
The bundle is ~116 MB: ship it as an install-time asset pack or a verified download of a signed,
checksummed bundle. Verify the SHA-256 from `manifest.json` before loading; fall back to `calme`
when the model is unavailable.

## Accessibility and localisation
String resources with English keys; French first. TalkBack labels on every control; honour the
system animator-duration scale and reduce-motion (see the `gpu-effects` skill).

## Verify
Test on at least one low-end and one recent device; record latency, memory and battery. Say "not
verified on device" when that is the case.
