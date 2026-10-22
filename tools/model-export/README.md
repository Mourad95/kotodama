# Model export

Builds the **model bundle** loaded by `lt-emotion` (see `docs/architecture.md`). Milestone M1.

## Pipeline

1. **Prune** the vocabulary to French + English —
   `research/emotion-benchmark/prune_vocab.py --wiki 20000 --wiki-min-count 3`.
2. **Export a single ONNX graph** for the full sentence embedding (transformer + mean pooling +
   dense layers + L2 normalization), so the Rust side only tokenizes and runs one session.
3. **Quantize** to 4 bits (`MatMulNBits` + `GatherBlockQuantized`).
4. **Train the few-shot head** on `prototypes.json` with embeddings from the quantized graph, and
   write `emotion_head.json` (labels, weights, bias).
5. **Write `parity.json`**: reference messages with their expected probabilities, used by the Rust
   parity test (tolerance 1e-3).
6. **Write `manifest.json`**: source model and revision, pruning settings, confidence threshold,
   checksums, benchmark metrics.

`export_mobile.py` (to be written in M1) chains these steps; steps 1 and 3 reuse the benchmark
scripts, which become importable modules here.

## Output

```
bundle/
  model_q4.onnx
  tokenizer.json
  emotion_head.json
  parity.json
  manifest.json
```

The bundle is never committed (see `.gitignore`); it is published as a release asset.
