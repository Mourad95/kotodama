# Architecture Decision Records

One file per decision, numbered, never rewritten: a decision that changes gets a new ADR that
supersedes the old one. Format: context, decision, consequences, status.

| # | Decision | Status |
|---|---|---|
| [0001](0001-english-codebase.md) | Code, docs and commits in English | accepted |
| [0002](0002-rust-core-native-shells.md) | Rust core + thin native shells via UniFFI | accepted |
| [0003](0003-typed-choice-emotion-engine.md) | "Typed choice" emotion engine on EmbeddingGemma | accepted |
| [0004](0004-onnx-runtime-q4-pruned.md) | ONNX Runtime, vocabulary pruning, 4-bit weights | accepted |
| [0005](0005-themes-as-data.md) | Themes are data interpreted by one renderer | accepted |
| [0006](0006-grammar-providers.md) | One grammar provider per language | accepted |
| [0007](0007-on-device-only.md) | All analysis on device, no message leaves the phone | accepted |
| [0008](0008-confidence-threshold.md) | Low-confidence results fall back to `calme` | accepted |
| [0009](0009-own-french-checker.md) | French checker built in-house (Grammalecte is GPL-3) | accepted |
| [0010](0010-open-licence-models-only.md) | Shipped models must carry an OSI-approved licence | accepted |
| [0011](0011-embeddinggemma-2-pruned-q4.md) | Ship EmbeddingGemma 2, pruned and Q4 | accepted |
| [0012](0012-cli-crate.md) | A small `kd-cli` crate for manual analysis and evaluation | accepted |
| [0013](0013-slim-onnx-graph.md) | Slim the ONNX graph so the session opens fast | accepted |
| [0014](0014-verify-the-bundle-once.md) | Verify the bundle checksums once, not at every launch | accepted |
| [0015](0015-scene-sampling-and-two-renderers.md) | Effects: a pure scene sampler, a CPU reference renderer, then wgpu | accepted |
