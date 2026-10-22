# Roadmap

Kotodama is a mobile app that animates messages according to the emotion they carry.
The animation only reaches full intensity when the message is well written, which turns
spelling and grammar into a game for teenagers. French first, then English.

This roadmap is ordered by **risk**: each milestone removes the biggest remaining unknown
before we invest in the next one. Every milestone has explicit exit criteria; we do not move
on until they are met or the decision to deviate is recorded in an ADR.

## Status

| Milestone | Goal | Status |
|---|---|---|
| M0 | Foundations: repo, plan, decisions, CI | in progress |
| M1 | Emotion engine running in Rust on desktop | next |
| M2 | Device spike: engine on a real iPhone and Android phone | — |
| M3 | Effects engine prototype (wgpu) | — |
| M4 | Grammar loop: errors → animation intensity | — |
| M5 | App V1 on a first platform, beta with ~10 teens | — |
| M6 | Android keyboard (IME) | — |
| M7 | iOS keyboard + iMessage extension | — |

Research tracks run in parallel (see below).

---

## M0 — Foundations

- Monorepo with Rust workspace, native shell folders, docs, ADRs, CI.
- All decisions taken during exploration recorded as ADRs (`docs/adr/`).
- Emotion benchmark and its results imported under `research/emotion-benchmark/`.

**Exit criteria:** CI green on `main` (fmt, clippy, tests); `CLAUDE.md` describes how to work in the repo.

## M1 — Emotion engine on desktop

- `tools/model-export/export_mobile.py`: build a **single ONNX graph** from the chosen model
  (transformer + mean pooling + dense layers + normalization), quantize it to Q4, train the
  few-shot head on `prototypes.json`, and write a **model bundle**:
  `model_q4.onnx`, `tokenizer.json`, `emotion_head.json`, `parity.json`.
- `kd-emotion` crate: tokenizer (`tokenizers`) + ONNX Runtime (`ort`) + head + confidence + threshold.
- CLI `kd analyze "message"` for manual testing.

**Exit criteria:**
- Parity with Python: probabilities match `parity.json` within 1e-3 on all reference messages.
- Accuracy on `test_v2.csv` reproduced in Rust within ±1 point of the Python benchmark (86.7 %).

## M2 — Device spike (highest technical risk)

Thin iOS (Swift) and Android (Kotlin) shells that call the Rust core through UniFFI and run
the emotion engine on a typed message. No UI work beyond a text field and a result label.

Measure on one recent and one mid-range phone per platform:
- latency (p50, p95), peak memory, app size impact;
- support of the Q4 operators (`MatMulNBits`, `GatherBlockQuantized`) in the mobile ONNX Runtime builds.

**Exit criteria:** p95 latency ≤ 200 ms on mid-range devices (analysis runs on typing pause,
not per keystroke), peak memory ≤ 300 MB, operators supported.
**Fallback if Q4 is unsupported:** INT8 pruned model (186 MB) and revisit the size budget.

## M3 — Effects engine prototype

- `kd-fx` with `wgpu` (Metal / Vulkan) and `cosmic-text` for glyph layout.
- Theme format v1 (`themes/*/theme.json`, JSON Schema validated).
- One theme (anime), one emotion (energy), three intensity levels, running in both shells.

**Exit criteria:** 60 fps on mid-range devices; golden-frame tests in CI for the three levels;
a new level can be added by editing JSON only.

## M4 — Grammar loop

- `kd-grammar`: provider interface; French provider = Grammalecte running in an embedded
  JavaScript engine (QuickJS); English provider = Harper (native Rust) later.
- Intensity scoring: errors per word → intensity level → effects.
- Correction UX data: error spans, suggestion, one-line explanation.

**Exit criteria:** French checks in ≤ 50 ms per message on device; intensity changes live in the M3 prototype.

## M5 — App V1 (first platform)

- Write a message, live animated preview, tap-to-fix errors, themes picker (2–3 themes).
- Export to WebP / MP4 / GIF and share sheet to WhatsApp, Messages, Instagram, TikTok.
- Points, streaks, weekly recap (local only).
- Choose the first platform from data on our target (share of iPhone vs Android among French teenagers).

**Exit criteria:** closed beta with ~10 teenagers for two weeks; retention and correction-rate metrics collected
(on device, with consent).

## M6 — Android keyboard (IME)

Android allows rich content insertion (GIF / WebP) directly into messaging apps, and the IME
process can host the full emotion model.

## M7 — iOS keyboard + iMessage extension

The iOS keyboard extension has a tight memory budget: the 116 MB model will not fit.
Depends on research track R2 (distilled small model). The iMessage extension can ship earlier.

---

## Research tracks (parallel)

| Track | Question | Output |
|---|---|---|
| R1 — Data | Better prototypes and an honest test set | `prototypes` v2 written without looking at `test_v2` errors; `test_v3` from real teen messages (with consent) |
| R2 — Distillation | A ≤ 40 MB model for the iOS keyboard | Small model fine-tuned on messages labeled by the big model; must reach the quality bar |
| R3 — Legal | Can we ship? | Gemma Terms of Use review, GDPR for minors (parental consent under 15 in France), store policies |
| R4 — Product | First platform, pricing, school/parent offers | Data on target devices, interviews |

## Quality bar (emotion engine)

From the round 2 benchmark (`research/emotion-benchmark/`):
global ≥ 80 %, messages with errors ≥ 70 %, recall on `energie` and `suspense` ≥ 70 %,
confidence threshold giving ≥ 85 % precision on ≥ 70 % of messages.
The current engine (EmbeddingGemma, pruned FR+EN, Q4, few-shot head) reaches 86.7 % / 81.4 % / 93.0 % / 90.7 %.
