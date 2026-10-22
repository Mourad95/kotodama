# Working in this repository

KotoMotion animates messages according to their emotion; the animation reaches full intensity
only when the message is well written. Mobile (iOS + Android), French first, then English.

Read before changing anything, in this order: `docs/product.md` (what we build and what we do
not), this file, `docs/agent-guide.md` (workflow, definition of done, and which skills to apply),
`docs/conventions.md` (naming, all in English), `docs/architecture.md`, `docs/roadmap.md`, `docs/adr/`, `docs/legal.md`.

## Language

Everything in the repository is in **English**: code, comments, docs, ADRs, commits, issues
(ADR-0001). Emotion identifiers in data files stay in French (`joie`, `colere`, …).

## Layout

| Path | What |
|---|---|
| `crates/kd-core` | shared types and pure logic; depends on nothing internal |
| `crates/kd-emotion` | typed-choice emotion engine (embedder + few-shot head) |
| `crates/kd-grammar` | grammar providers behind `GrammarChecker` |
| `crates/kd-fx` | theme data (+ wgpu renderer from M3) |
| `crates/kd-ffi` | the only crate the native shells link against (UniFFI) |
| `apps/ios`, `apps/android` | thin native shells |
| `themes/` | theme JSON files + `schema.json` |
| `.claude/skills/` | project skills for agents (conventions, Rust core, on-device ML, shells, effects, privacy) |
| `tools/model-export/` | Python pipeline producing the model bundle |
| `research/` | benchmarks, protocols and results — reproducible, never edited after the fact |

## Commands

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
```

## Rules

1. **Tiny FFI.** Add to `kd-ffi` only what a shell truly needs; plain data types only.
2. **Behaviour is data.** A new theme, emotion example or tip is a data change, never theme- or
   emotion-specific code. If the schema cannot express it, change the schema deliberately
   (Rust types in `kd-fx` are authoritative; mirror them in `themes/schema.json`).
3. **On device only.** No network call in the analysis path; message text never leaves the phone (ADR-0007).
4. **Benchmark hygiene.** `research/emotion-benchmark/test_v2.csv` is for measurement only: never
   tune prototypes, thresholds or pruning on it. Its checksums are verified in CI.
5. **No model weights in git** (`*.onnx`, `*.safetensors`): they are built by
   `tools/model-export` and distributed as release assets with a manifest and checksums.
6. **Every decision that changes the architecture gets an ADR** in `docs/adr/`.
7. **Tests with every change.** Pure logic gets unit tests; the engine gets parity tests against
   the Python reference; effects get golden-frame tests (from M3).
8. `missing_docs` is on: document public items.
9. **Legal register.** `docs/legal.md` is binding: respect its rules, and do not start work blocked by an open gate (for example publishing without the age-gate design) before the gate is closed there.

## Commits

Conventional commits (`feat:`, `fix:`, `docs:`, `chore:`, `test:`, `refactor:`), imperative mood,
scope when useful (`feat(emotion): …`).
