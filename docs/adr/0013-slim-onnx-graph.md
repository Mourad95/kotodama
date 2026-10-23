# ADR-0013 — Slim the ONNX graph so the session opens fast

**Status:** accepted · 2026-10-23

## Context
The first on-device measurement (Xiaomi M2101K6G, Android 12, arm64) showed a 10 s cold start. The
SHA-256 check is 0.7 s of it; 9.3 s is ONNX Runtime creating the session. The cost is the memory
planner, which grows faster than linearly with the node count (1.4k nodes: 0.3 s, 4k: 1.6 s, 8k: 7 s,
also on a laptop). Session options (optimisation level, arena, memory pattern, threads, fixed
dimensions) change nothing, the ORT format cannot store int4 weights, and the one flag that avoids the
planning (`enable_mem_reuse`) is not reachable through the C API used by the `ort` crate.

## Decision
`tools/model-export/slim_graph.py` reduces the Q4 graph after quantisation:
1. ONNX Runtime folds constants offline (basic level, nothing hardware specific);
2. each RMSNorm becomes one `SimplifiedLayerNormalization`, each tanh GELU one `FastGelu`
   (4011 → 2777 nodes);
3. the weights go to an external data file (`model_q4.onnx_data`), which ONNX Runtime memory-maps.

The script fails if the slim graph's embeddings differ from the source graph (cosine below 0.995).
`export_mobile.py --onnx slim/model_q4.onnx` builds the bundle from it; the bundle now holds the extra
data file, listed and hashed in the manifest like the others.

## Consequences
- Opening the session takes 1.7 s instead of 7.2 s on a laptop; dev_v1 accuracy is unchanged (88.6 %),
  the Rust/Python parity is 5e-7. The phone figures are recorded in `docs/roadmap.md` once measured.
- The slim graph differs from the source by about 1e-3 in cosine (float order of operations,
  amplified by Q4), well below the Q4 error against PyTorch (cosine 0.977).
- The bundle needs `com.microsoft` contrib operators (`FastGelu`): present in the `onnxruntime-android` package.
- Next lever if 1.7 s is still too slow: remove the shape arithmetic (about 1200 nodes) from the attention.
