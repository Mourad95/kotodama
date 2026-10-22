# ADR-0010 — Shipped models must carry an OSI-approved licence

**Status:** accepted · 2026-10-22 · amends [ADR-0003](0003-typed-choice-emotion-engine.md) (model choice)

## Context
ADR-0003 selected `google/embeddinggemma-300m`, released under the custom **Gemma Terms of Use**
(not an OSI-approved licence: notice obligations, a prohibited-use policy passed on to end users,
terms that Google may change for future versions). The product is aimed at minors and is meant to
be commercial; the owner wants no ownership or licensing ambiguity on the model.

On 2026-10-06 Google released **EmbeddingGemma 2** under **Apache-2.0**, ungated, on
`google/embeddinggemma-2` (per Google's announcement and the Hugging Face record; to be confirmed
on the model card when downloading). It has a 270M-parameter text backbone with optional vision and
audio encoders, 768 dimensions with Matryoshka truncation, 8K context. Version 1 stays under the
Gemma terms.

## Decision
1. **Rule:** every model weight file shipped in the app must be under an OSI-approved permissive
   licence (Apache-2.0, MIT, BSD). Gemma Terms of Use, non-commercial (CC-BY-NC) and custom
   licences are excluded from the shipped bundle. The benchmark may still use them as baselines.
2. **Candidate:** EmbeddingGemma 2, text-only, replaces EmbeddingGemma 1 *if* it meets the same
   quality bar on the frozen `test_v2` (≥ 85 % global, ≥ 80 % on messages with errors, `energie`
   and `suspense` ≥ 90 %) after pruning and Q4 within the 150 MB budget (benchmark round 3).
3. **Fallbacks** if it fails: `paraphrase-multilingual-MiniLM-L12-v2` (Apache-2.0, measured 67.8 %,
   so it needs a better head or distillation) or a model we distil ourselves from an Apache-2.0
   teacher (research track R2).
4. The weights are downloaded only from the publisher's official repository; the SHA-256 goes in the
   bundle manifest. The licence text and any required notice ship in the app.

## Consequences
- Legal gate L9 is replaced by rule L9 (open licence only); the Gemma notice/prohibited-use
  obligations no longer apply to the shipped app.
- "Apache-2.0 weights" removes licence obligations; it does not make the training data public.
  The residual question of training-data provenance is listed in `docs/legal.md` (L17) for the lawyer.
- A benchmark round 3 is required before M1 starts exporting the bundle (roadmap).
