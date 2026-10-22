# ADR-0012 — A small `kd-cli` crate for manual analysis and evaluation

**Status:** accepted · 2026-10-22

## Context
Milestone M1 needs a way to run the emotion engine on a desktop: try one message, and measure accuracy on
a labelled CSV (the Rust counterpart of the Python benchmark). The engine crate must stay free of
command-line and CSV dependencies, and `kd-ffi` stays tiny (CLAUDE.md rule 1).

## Decision
1. New workspace crate `crates/kd-cli`, binary `kd`: `kd analyze "message" --bundle <dir>` and
   `kd eval <csv> --bundle <dir>` (accuracy and per-emotion recall).
2. It depends on `kd-emotion` only (plus `csv`, MIT/Unlicense). It is never linked by the native shells.
3. ONNX Runtime is loaded dynamically (`ort` with `load-dynamic`, `ORT_DYLIB_PATH`): nothing is downloaded
   at build time and the mobile shells choose their own runtime build (checked in M2).

## Consequences
- The parity test (`kd-emotion/tests/parity.rs`) and `kd eval` are the Rust side of the M1 exit criteria.
- The parity test skips when the bundle or the runtime library is absent, so CI stays green without weights.
