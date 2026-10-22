# ADR-0011 — Ship EmbeddingGemma 2, pruned and Q4

**Status:** accepted · 2026-10-22 · follows [ADR-0010](0010-open-licence-models-only.md) (round 3 results)

## Context
Round 3 measured open-licence candidates on the frozen `test_v2` (301 messages, ±5 points), after
vocabulary pruning and 4-bit quantisation, run one message at a time on ONNX Runtime (container CPU):

| Model (pruned, Q4) | Size | Global | With errors | energie | suspense | Latency |
|---|---|---|---|---|---|---|
| EmbeddingGemma 2 (Apache-2.0) | 119.7 MB | 80.4 % | 81.4 % | 86.0 % | 76.7 % | 54 ms |
| bge-m3 (MIT) | 224 MB | 80.7 % | 81.4 % | 95.3 % | 83.7 % | 38 ms |
| EmbeddingGemma 2 with `prototypes_v2.json` | 119.7 MB | 82.4 % | 81.4 % | 88.4 % | 86.0 % | 42 ms |

The two models are **statistically tied** (0.3 points apart; the `energie` and `suspense` gaps are 3 to 4
messages out of 43, inside the ±5-point noise). EmbeddingGemma 2 is chosen for **size** (120 MB against
224 MB) and licence, not for accuracy. The last row uses balanced examples (20 more per emotion, written
before looking at `test_v2` errors and measured once); it is the figure closest to the bar.

Other candidates were clearly behind or unusable: Qwen3-Embedding-0.6B (79.4 %, 511 ms), multilingual-e5-base
(76.4 %), granite (72.4 % / 69.8 %), a GoEmotions mBERT classifier (51.5 %). Two suggested classifiers are
excluded by licence (CC-BY-NC; no licence declared). Generative LLMs (Qwen 2.5, Llama 3) were set aside as
too heavy for the phone. Raw results: `research/emotion-benchmark/results/round3/`.

## Decision
1. Ship **EmbeddingGemma 2, text-only, pruned to a ~100k vocabulary, Q4 (block 32)**, exported by
   `research/emotion-benchmark/export_onnx.py` (to move to `tools/model-export`).
2. **bge-m3** (MIT) stays the fallback if EmbeddingGemma 2 cannot be shipped (legal question below) or if the
   224 MB size becomes acceptable and its better `energie` / `suspense` recall is wanted.

## Consequences
- The ADR-0010 bar (85 % global, 90 % `energie` and `suspense`) is **not reached** by either model
  (80 to 82 % global, 86 to 95 % on `energie`, 77 to 86 % on `suspense`); the roadmap bar (80 / 70 / 70) is met.
  The owner decides whether to keep the stricter bar. Prototypes v2 already gained about 2 points; going
  further needs a larger development set than `dev_v1` (70 messages) so that `test_v2` stays untouched.
- Open question for the lawyer (`docs/legal.md`): the model card of EmbeddingGemma 2 refers to the Gemma
  Prohibited Use Policy although the weights are Apache-2.0.
- Still to measure: a light pruning validation, and latency and memory on a real phone.
