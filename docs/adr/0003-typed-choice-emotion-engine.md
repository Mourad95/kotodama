# ADR-0003 — "Typed choice" emotion engine on EmbeddingGemma

**Status:** accepted · 2026-10-22

## Context
The app needs 7 emotions (`joie`, `colere`, `energie`, `suspense`, `tristesse`, `amour`, `calme`),
two of which (`energie`, `suspense`) exist in no off-the-shelf classifier, on teenage French and
English with slang, emojis and spelling errors, fully on device. A hosted API (TypeSafe Jev) was
considered and rejected for privacy and offline reasons (ADR-0007).

Two benchmark rounds were run (`research/emotion-benchmark/`). Round 2: 301 independent test
messages, 70 with errors.

| Engine | Global | With errors | `energie` | `suspense` | Size |
|---|---|---|---|---|---|
| EmbeddingGemma pruned, Q4, few-shot head | **86.7 %** | **81.4 %** | 93.0 % | 90.7 % | **116 MB** |
| MiniLM multilingual pruned, INT8 | 67.8 % | 72.9 % | 88.4 % | 60.5 % | 39 MB |
| Fixed-label classifier (tabularisai, CC-BY-NC) | 56.8 % | 50.0 % | — | — | 1.1 GB |

## Decision
A "typed choice" engine: an embedding model plus a few-shot linear head trained on
`prototypes.json` (a description and examples per emotion). It returns a choice, a probability per
emotion and a confidence. The model is `google/embeddinggemma-300m` (benchmark ran on the
`unsloth` mirror; weights to be verified against Google's repo).

## Consequences
- Emotions are defined by data; editing prototypes requires no model training.
- The Gemma Terms of Use apply (commercial use allowed, notice and prohibited-use policy must be
  passed on). Legal review is tracked in research track R3.
- 116 MB does not fit an iOS keyboard extension: research track R2 (distillation).
- Known weak spots: implicit emotions, sarcasm, subject vs sender emotion. Improve through
  prototypes v2 and an independent `test_v3` (track R1).
