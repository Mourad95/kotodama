# iOS shell

Thin Swift shell around the Rust core (ADR-0002). Added in milestone M2 (device spike):

- app target (SwiftUI) calling `lt-ffi` through UniFFI-generated Swift bindings;
- the Rust core built as an `xcframework` (`aarch64-apple-ios`, `aarch64-apple-ios-sim`);
- later: keyboard extension (M7, needs a distilled model) and iMessage extension.

No business logic here: anything shared with Android belongs in `crates/`.
