# Model export

Builds the **model bundle** loaded by `kd-emotion` (see `docs/architecture.md`). Milestone M1.

## Pipeline

1. **Prune** the vocabulary to French + English —
   `research/emotion-benchmark/prune_vocab.py --wiki 20000 --wiki-min-count 3`.
2. **Export a single ONNX graph** for the full sentence embedding (transformer + mean pooling +
   dense layers + L2 normalization), so the Rust side only tokenizes and runs one session.
3. **Quantize** to 4 bits (`MatMulNBits` + `GatherBlockQuantized`).
3b. **Slim the graph** with `slim_graph.py` (fused norms and GELUs, external weights): the session
   opens in under 2 s instead of 7 s (ADR-0013).
4. **Train the few-shot head** on `prototypes.json` with embeddings from the quantized graph, and
   write `emotion_head.json` (labels, weights, bias).
5. **Write `parity.json`**: reference messages with their expected probabilities, used by the Rust
   parity test (tolerance 1e-3).
6. **Write `manifest.json`**: source model and revision, pruning settings, confidence threshold,
   checksums, benchmark metrics.

`export_mobile.py` chains steps 2-6; it reuses `research/emotion-benchmark/ort_embedder.py` and
`export_onnx.py`. By default it reuses the existing Q4 graph of the pruned model:

```bash
python tools/model-export/slim_graph.py --src research/emotion-benchmark/pruned/gemma2-m3/onnx/model_q4.onnx \
    --dst research/emotion-benchmark/pruned/gemma2-m3/slim/model_q4.onnx
python tools/model-export/export_mobile.py --onnx slim/model_q4.onnx   # pruned/gemma2-m3 -> bundle/
python tools/model-export/export_mobile.py --export         # re-run export_onnx.py first
python tools/model-export/export_mobile.py --prototypes research/emotion-benchmark/prototypes_v2.json
python -m pytest tools/model-export                         # pure-logic tests, no model needed
```

The head is trained exactly like `bench.py` "fewshot" (LogisticRegression, C=10) with the
`task: classification | query: ` prompt. `parity.json` and the manifest metrics use `dev_v1.csv`
only, never `test_v2.csv`.

## Output

```
bundle/
  model_q4.onnx        (+ model_q4.onnx_data: the weights, memory-mapped by ONNX Runtime)
  tokenizer.json
  emotion_head.json
  parity.json
  manifest.json
```

The bundle is never committed (see `.gitignore`); it is published as a release asset.
