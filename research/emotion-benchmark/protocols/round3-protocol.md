# Round 3 protocol — Apache-2.0 model (ADR-0010)

Goal: decide whether **EmbeddingGemma 2** (`google/embeddinggemma-2`, Apache-2.0) can replace
EmbeddingGemma 1 with the same quality, size and speed. Same hygiene as round 2: `test_v2.csv` is
for measurement only, verify `md5sum -c MD5SUMS` before and after, never tune on it.

## 0. Setup
- Python env as in `results/round2/versions.txt`; record new versions in `results/round3/versions.txt`.
- Download `google/embeddinggemma-2` from the **official repo**. Record the commit hash, the
  `LICENSE` file content and the SHA-256 of every weight file. Confirm the model card says
  Apache-2.0 and the repo is not gated; if not, stop and report.
- Use the **text-only** configuration (270M backbone). If the loader pulls vision/audio encoders
  into the exported graph, export the text tower only. Report the parameter count actually shipped.
- Check the tokenizer: if it is the Gemma 262k SentencePiece vocabulary, `prune_vocab.py` applies
  unchanged; otherwise report the format and adapt the script (do not guess).

## 1. Unpruned baseline
```bash
python bench.py --only gemma2-proto gemma2-fewshot --out results/round3/r3-origin
```
Also run Matryoshka truncation at 768 / 512 / 256 dims for `gemma2-fewshot` and report accuracy.

## 2. Pruning (FR + EN) and Q4
```bash
python prune_vocab.py --model google/embeddinggemma-2 --out pruned/gemma2-m3 --wiki 20000 --wiki-min-count 3
python quantize_onnx.py --model pruned/gemma2-m3 --q4
python bench.py --only none --out results/round3/r3-pruned-q4 --local gemma2-m3-q4=pruned/gemma2-m3@onnx/<q4 file>.onnx
```
Pruning validity criterion: as in round 2 (`pruning.json`, FR/EN held-out lines).

## 3. Acceptance (ADR-0010)
Pass if, for the pruned Q4 model on `test_v2.csv` with the few-shot head: global ≥ 85 %, messages
with errors ≥ 80 %, `energie` ≥ 90 %, `suspense` ≥ 90 %, bundle ≤ 150 MB. Report latency on CPU
(median of 200 runs) and peak memory. Compare with the round 2 table in `README.md`.

## 4. Deliverables
`results/round3/` with raw reports (`.md` + `.json`), `versions.txt`, `pruning.json`, the weight
file checksums and a `SYNTHESIS.md` in English: table vs round 2, pass/fail per criterion,
recommendation (adopt / fall back to ADR-0010 option 3), risks. Report anything surprising.
