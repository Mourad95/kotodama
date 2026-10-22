# Emotion engine benchmark

How the emotion engine of Living Text was chosen (ADR-0003, ADR-0004). Everything here is a
**record**: results are never edited after the fact, and the test set is frozen (checksums in
`MD5SUMS`, verified in CI).

## Outcome

| Engine | Global | Lenient | With errors | `energie` | `suspense` | Size | Latency (M1, CPU) |
|---|---|---|---|---|---|---|---|
| **EmbeddingGemma, vocab pruned FR+EN (min count 3), Q4, few-shot head** | **86.7 %** | 87.4 % | **81.4 %** | 93.0 % | 90.7 % | **115.9 MB** | 25.5 ms |
| EmbeddingGemma, not pruned, Q4, few-shot | 87.4 % | 88.0 % | 81.4 % | 95.3 % | 90.7 % | 190.7 MB | 25.6 ms |
| EmbeddingGemma pruned, INT8, few-shot | 86.0 % | 86.7 % | 81.4 % | 93.0 % | 88.4 % | 185.8 MB | 27.8 ms |
| MiniLM multilingual pruned, INT8, prototypes | 67.8 % | 68.4 % | 72.9 % | 90.7 % | 60.5 % | 38.9 MB | 4.4 ms |
| Fixed-label classifier (tabularisai, CC-BY-NC, control) | 56.8 % | 58.5 % | 50.0 % | — | — | 1.1 GB | 8.3 ms |

Test set `test_v2.csv`: 301 messages (196 French, 105 English, 70 with spelling errors).
95 % uncertainty: about ±5 points global, ±10 points on messages with errors.
Confidence threshold 0.2 (chosen on `dev_v1.csv`): 91.0 % precision on 85 % of test messages.

## Method: "typed choice"

Each emotion is described in `prototypes.json` (a description + 16 French/English examples).
An embedding model encodes the message; either the nearest emotion wins (`proto`) or a logistic
regression trained on the examples decides (`fewshot`). The engine returns a choice, a probability
per emotion and a confidence (1 − normalized entropy).

## Files

| Path | Content |
|---|---|
| `test_v2.csv` | frozen test set — **measurement only, never tune on it** |
| `dev_v1.csv` | development set (round 1 test set), used for tuning the threshold |
| `prototypes.json` | emotion descriptions and examples used in round 2 |
| `slang_lexicon.txt` | slang / SMS / emoji lexicon added to the pruning corpus |
| `data_src/` | sources of `test_v2.csv` |
| `bench.py` | benchmark (strict and lenient accuracy, confidence, latency, size) |
| `prune_vocab.py` | vocabulary pruning to French + English with automatic validation |
| `quantize_onnx.py` | ONNX export, INT8 and Q4 quantization, size and latency |
| `protocols/` | annotation guide and protocols as given to the benchmark agent (French originals) |
| `results/round1/` | round 1 report (70 messages; superseded, kept for history) |
| `results/round3/` | round 3 (open-licence candidates, pruned Q4; summary in ADR-0011) — protocol in `protocols/round3-protocol.md` |
| `results/round2/` | round 2 synthesis, raw reports, pruning and quantization reports |

Annotation guide: [`ANNOTATION.md`](ANNOTATION.md). Protocols and reports were written in French
during exploration and are kept verbatim as historical records; new material is in English (ADR-0001).

## Reproduce

Requirements and commands: `protocols/round2-protocol.fr.md` and `protocols/round2-followup.fr.md`.
Library versions: `results/round2/versions.txt` (Apple M1 Max, macOS, CPU execution provider).

## Known limits

- The model came from the `unsloth/embeddinggemma-300m` mirror; weights still to be verified
  against `google/embeddinggemma-300m`.
- Ideas listed in the round 2 synthesis for new prototypes partly reuse `test_v2` messages:
  do not add them verbatim. Prototypes v2 must be written independently and evaluated on a new
  `test_v3` (roadmap, research track R1).
- Q4 graphs use opset 21 contrib operators: availability on mobile runtimes is checked in M2.
