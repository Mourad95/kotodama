# ADR-0011 — Ship EmbeddingGemma 2, pruned and Q4

**Status:** accepted · 2026-10-22 · follows [ADR-0010](0010-open-licence-models-only.md) (round 3 results)

## Context
Round 3 measured open-licence candidates on the frozen `test_v2` (301 messages, ±5 points), after
vocabulary pruning and 4-bit quantisation, run one message at a time on ONNX Runtime (container CPU):

| Model (pruned, Q4) | Size | Global | With errors | energie | suspense | Latency |
|---|---|---|---|---|---|---|
| EmbeddingGemma 2 (Apache-2.0) | 119.7 MB | 80.4 % | 81.4 % | 86.0 % | 76.7 % | 54 ms |
| bge-m3 (MIT) | 224 MB | 80.7 % | 81.4 % | 95.3 % | 83.7 % | 38 ms |

Other candidates were clearly behind or unusable: Qwen3-Embedding-0.6B (79.4 %, 511 ms), multilingual-e5-base
(76.4 %), granite (72.4 % / 69.8 %), a GoEmotions mBERT classifier (51.5 %). Two suggested classifiers are
excluded by licence (CC-BY-NC; no licence declared). Generative LLMs (Qwen 2.5, Llama 3) were set aside as
too heavy for the phone. Raw results: `research/emotion-benchmark/results/round3/`.

## Decision
1. Ship **EmbeddingGemma 2, text-only, pruned to a ~100k vocabulary, Q4 (block 32)**, exported by
   `research/emotion-benchmark/export_onnx.py` (to move to `tools/model-export`).
2. **bge-m3** (MIT) stays the fallback if EmbeddingGemma 2 cannot be shipped (legal gate below) or if its
   `energie` / `suspense` recall cannot be raised.

## Consequences
- The ADR-0010 bar (85 % global, 90 % `energie` and `suspense`) is **not reached** by either model
  (about 80 %); the roadmap bar (80 / 70 / 70) is met. The owner decides whether to keep the stricter bar;
  the lever is more few-shot examples (data), not the model.
- Open question for the lawyer (`docs/legal.md`): the model card of EmbeddingGemma 2 refers to the Gemma
  Prohibited Use Policy although the weights are Apache-2.0.
- Still to measure: a light pruning validation, and latency and memory on a real phone.
