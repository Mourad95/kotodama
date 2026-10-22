---
name: ios-swift-shell
description: Load when working in apps/ios — the SwiftUI app, the custom keyboard extension, the iMessage extension, building the Rust core as an XCFramework, UniFFI Swift bindings, memory limits, privacy manifest or App Store review for a kids-oriented keyboard. Covers what the shell may and may not do and the traps to verify on device.
---

# iOS shell

A thin Swift shell around the Rust core (ADR-0002). No business logic here: anything shared with
Android belongs in `crates/`. Apple's rules change; **check current Apple documentation before
relying on any limit below.**

## Targets
1. App (SwiftUI): onboarding, composer, theme picker, settings, learning feedback.
2. iMessage extension: send animated stickers/messages inside Messages.
3. Keyboard extension (M7): type anywhere. The hardest target.

## Building the core
- Build `kd-ffi` for `aarch64-apple-ios` and `aarch64-apple-ios-sim` (and the Mac if needed), pack
  an XCFramework, expose it as a Swift package with the UniFFI-generated Swift file.
- Rebuild bindings in CI; a Rust signature change must fail the Swift build.
- Never rename generated Swift names; fix them in Rust.

## Keyboard extension constraints
- **Memory is small and strictly enforced** (historically tens of MB): the 116 MB model does not fit.
  The keyboard needs a distilled model or a different split (research track R2). Treat the exact
  limit as unknown until measured on a device, with the extension's own memory profile.
- Do analysis on a typing pause (debounce ~300 ms), off the main thread; the keyboard UI must stay
  responsive.
- **Full Access** (`RequestsOpenAccess`) is a legal gate (L5): request it only if a feature truly
  needs it, and write the justification first. The default design must work without it. Without
  it, the extension cannot use the network and has limited shared storage: plan the bundle sharing
  with the app (App Group) accordingly, and verify what works.
- Ignore and do not analyse secure text fields (passwords) and never log or store typed text.
- A keyboard cannot freely insert an animated image into every host app; the supported route is
  the pasteboard or the share/sticker mechanisms. Verify which are available without Full Access.

## Privacy and App Store
- Include a privacy manifest (`PrivacyInfo.xcprivacy`); declare no tracking and no collected data
  unless that is true. The privacy labels must match what the code does (gate L4).
- The app targets minors: review Apple's rules for kids apps and for third-party keyboards before
  choosing the age rating and category (gate L6). No third-party analytics or advertising SDK.
- Localise user-facing strings with String Catalogs; keys in English (`docs/conventions.md`).
- Accessibility: Dynamic Type, VoiceOver labels on every control, honour Reduce Motion (see the
  `gpu-effects` skill).

## Rendering
The core renders with Metal via wgpu; host it in a `CAMetalLayer`-backed view. Pause rendering when
the app is backgrounded; respect Low Power Mode by lowering particle counts.

## Verify
Build on a device, not only the simulator. Record memory (Instruments), latency and thermals. Say
"not verified on device" when that is the case.
