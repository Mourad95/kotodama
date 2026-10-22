# ADR-0004 — ONNX Runtime, vocabulary pruning, 4-bit weights

**Status:** accepted · 2026-10-22

## Context
EmbeddingGemma is 1.27 GB in FP32. Most of a multilingual model is its vocabulary embedding
table (262k pieces for 100+ languages) while we need French and English only.

## Decision
- **Vocabulary pruning** to pieces used by a French + English corpus (Wikipedia pieces kept from
  3 occurrences, all pieces from short teen-style messages, Latin characters and emojis as fallback):
  262,145 → 106,233 pieces, 307.6 M → 187.8 M parameters, embeddings unchanged on short messages
  (mean cosine 0.99992).
- **4-bit block quantization** of MatMul and embedding Gather (ONNX Runtime `MatMulNBits` +
  `GatherBlockQuantized`): 116 MB, faster and as accurate as dynamic INT8 (186 MB).
- **ONNX Runtime** as the inference runtime on both platforms, via the `ort` crate.

## Consequences
- The model graph uses opset 21 / IR 10 contrib operators: their availability in mobile ONNX
  Runtime builds must be verified in M2. Fallback: INT8 pruned model.
- Pruning and quantization scripts live in `tools/model-export/` and are reproducible.
