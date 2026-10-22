---
name: rust-core
description: Load when writing or reviewing Rust in the KotoMotion workspace (crates kd-core, kd-emotion, kd-grammar, kd-fx, kd-ffi), adding a crate or dependency, designing a trait, handling errors, or exposing something to Swift/Kotlin through UniFFI. Covers layering, error handling, determinism, data-file parsing, the tiny-FFI rule and the verification commands.
---

# Rust core

The core is written once in Rust; Swift and Kotlin only host screens and OS integrations
(ADR-0002). Quality here is what makes the app maintainable.

## Layering (enforced by review)
- `kd-core` depends on nothing internal: types and pure logic only, no I/O.
- `kd-emotion`, `kd-grammar`, `kd-fx` depend only on `kd-core`.
- `kd-ffi` is the only crate that depends on all of them and the only one shells link against.
- Adding an internal dependency edge, or a new crate, needs an ADR.

## Boundaries are traits
Everything that touches the outside world sits behind a trait so the logic is testable without
it: `Embedder` (model runtime), `GrammarChecker` (providers). Implementations are `Send + Sync`.
Tests use tiny fakes (a keyword embedder), not mocks of the thing under test.

## Errors
- Libraries: `thiserror` enums named `<Area>Error`, with enough context to act on
  (`DimensionMismatch { expected, got }`). No `anyhow` in library crates; allowed in binaries/tools.
- No `unwrap`/`expect`/indexing panics on external input (files, models, text). `expect` only for an
  invariant already validated, with the invariant in the message (see `Theme::level`).
- A failure in analysis must degrade gracefully in the shell: fall back to `calme`, never crash a
  keyboard.

## Data files
- Parse with `serde`, `#[serde(deny_unknown_fields)]` on every data struct, then **validate**
  semantics in a `validate()` step. Reject bad data with a precise message.
- Mirror every theme-format change in `themes/schema.json` and bump `SCHEMA_VERSION` with a
  migration when breaking.

## Determinism and state
- Same input, same output: no global mutable state, no wall-clock or randomness in logic. Pass a
  seed or a clock explicitly.
- Heavy objects (model session, renderer) are created once and shared; creation is explicit and
  fallible, never lazy-inside-a-getter.

## Performance
- Analysis target ≤ ~50 ms per message on a mid-range phone; keep allocations out of per-token and
  per-frame loops. Measure before optimising; record numbers in the PR.
- Never block a UI or keyboard thread: the shell calls the core from a background context.

## `unsafe` and dependencies
- `unsafe_code = "forbid"` workspace-wide. If a UniFFI upgrade ever forces an exception, scope it to
  `kd-ffi` only, in an ADR.
- Every new dependency: check its licence against `docs/legal.md` (no copyleft without an ADR),
  maintenance, and size impact on the mobile binary. Prefer fewer, well-known crates.

## Tiny FFI (rule 1)
- Add to `kd-ffi` only what a shell truly needs; plain data types (records, enums, strings, bytes).
- Errors cross the FFI as a flat enum; no panics across the boundary.
- Avoid callbacks and async across the boundary unless the design requires it; document why.
- Anything useful to both platforms is added in Rust, never in a shell.
- Regenerate bindings in CI so a signature change fails the build, not the app.

## Verify
```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --all
```
Public items documented (`missing_docs`). Unit tests live next to the code; engine behaviour also has
parity tests against the Python reference.
