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
| [0008](0008-confidence-threshold.md) | Low-confidence results fall back to `calme` | amended by 0021 |
| [0009](0009-own-french-checker.md) | French checker built in-house (Grammalecte is GPL-3) | accepted |
| [0010](0010-open-licence-models-only.md) | Shipped models must carry an OSI-approved licence | accepted |
| [0011](0011-embeddinggemma-2-pruned-q4.md) | Ship EmbeddingGemma 2, pruned and Q4 | accepted |
| [0012](0012-cli-crate.md) | A small `kd-cli` crate for manual analysis and evaluation | accepted |
| [0013](0013-slim-onnx-graph.md) | Slim the ONNX graph so the session opens fast | accepted |
| [0014](0014-verify-the-bundle-once.md) | Verify the bundle checksums once, not at every launch | accepted |
| [0015](0015-scene-sampling-and-two-renderers.md) | Effects: a pure scene sampler, a CPU reference renderer, then wgpu | accepted |
| [0016](0016-animation-plays-over-the-conversation.md) | The animation plays over the conversation, then the message settles in its bubble | accepted |
| [0017](0017-two-layer-rendering.md) | Render the animation in two layers at the screen's resolution | accepted |
| [0018](0018-situational-manga-and-persistent-bubbles.md) | Situational manga, persistent bubbles and editable eight-category training data | accepted |
| [0019](0019-textless-overlay-and-lettered-bubble.md) | Textless overlay, lettered manga bubble, sharp front layer | accepted |
| [0020](0020-anchored-stage-and-manga-panels.md) | Effects anchored on the bubble, full-screen manga panels | accepted |
| [0021](0021-neutral-and-calm.md) | Separate `neutre` (fallback, tiny animation) from `calme` | accepted |
| [0022](0022-frozen-scene-in-settled-bubbles.md) | The settled bubble keeps a frozen scene, scaled by the writing level | accepted |
| [0023](0023-sent-messages-as-stickers.md) | Sent messages show as stickers that play, then stay on their last frame | accepted |
