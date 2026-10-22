---
name: on-device-ml
description: Load when working on the KotoMotion emotion model — exporting or pruning the embedding model, quantization, ONNX Runtime on mobile, the few-shot head, the model bundle and manifest, parity tests, latency or memory budgets, or any benchmark. Covers the budgets, the licence rule, benchmark hygiene and what must be measured on a real phone.
---

# On-device ML

The emotion engine is "typed choice" (ADR-0003): a sentence-embedding model plus a few-shot linear
head trained on `prototypes.json`. It runs fully on the phone (ADR-0007).

## Non-negotiables
- **Licence:** a shipped model must be under an OSI-approved permissive licence (ADR-0010). Gemma
  Terms of Use and non-commercial models are baselines only. Weights come from the publisher's
  official repository; the SHA-256 of each file goes in `manifest.json`.
- **No weights in git.** Bundles are built by `tools/model-export/` and published as release assets.
- **No network** in the analysis path, and no message text in any log, metric or crash report.

## Budgets (revisit in M2 with real devices)
| Budget | Target |
|---|---|
| Bundle size | ≤ 150 MB (iOS keyboard needs far less: distillation, track R2) |
| Latency | analysis ≤ ~50 ms per message on a mid-range phone (25 ms measured on an M1 CPU) |
| Memory | the model must fit the host's limit; the iOS keyboard limit is small and must be measured |
| Quality | on frozen `test_v2`: ≥ 85 % global, ≥ 80 % with spelling errors, `energie`/`suspense` ≥ 90 % |

## Pipeline (see `tools/model-export/README.md`)
prune vocabulary (FR+EN) → single ONNX graph (transformer + pooling + dense + normalize) → 4-bit
quantization → train the head → write `parity.json` → write `manifest.json`.
- The tokenizer must be the pruned one shipped in the bundle; Rust and Python must tokenize
  identically. Test with edge cases: emojis, accents, apostrophes, very long text, empty text.
- Pruning is valid only if held-out French/English text embeds almost identically (cosine mean
  ≥ 0.999, min ≥ 0.99 on FR/EN lines).
- Changing prototypes re-trains the head only (seconds); the model is untouched.

## Runtime facts to verify, not assume
- 4-bit graphs use contrib operators (`MatMulNBits`, `GatherBlockQuantized`, opset 21). Their
  availability in the **mobile build** of ONNX Runtime, and the accelerator providers (CoreML on
  iOS, NNAPI/XNNPACK on Android), must be checked on devices in M2. In benchmarking, CoreML failed
  on quantized graphs and CPU was forced: do not assume acceleration works.
- Load the session off the UI thread, once, and reuse it. Measure cold start and warm latency
  separately; keep thread counts conservative (battery, heat).
- If the model fails to load or run, the shell shows `calme` and keeps working.

## Parity and evaluation
- Rust output must match the Python reference within 1e-3 on `parity.json` (CI test).
- **Benchmark hygiene:** `test_v2.csv` is measurement only; never tune prototypes, thresholds or
  pruning on it. Tune on `dev_v1.csv` or a new set. New prototypes are written independently and
  evaluated on a new `test_v3`. Verify `md5sum -c MD5SUMS` before and after any run.
- Report sample size and uncertainty (about ±5 points global on 301 messages, ±10 on the 70 with
  errors). Do not claim differences smaller than that.
- Confidence = 1 − normalised entropy; below 0.2 the animation uses `calme` (ADR-0008). Changing the
  threshold is tuned on dev data and recorded in the manifest.

## What only a real phone can prove
Latency, memory peak, thermal behaviour, operator support, cold start. Say "not verified on device"
until it was.
