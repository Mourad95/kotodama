# ADR-0021 — Separate `neutre` from `calme`

**Status:** accepted · 2026-10-24 (owner request) · display amended by ADR-0024

## Context

`calme` meant two things: neutral or practical messages ("je suis dans le bus") and the fallback
when the engine is unsure (ADR-0008). The owner wants a tiny animation for neutral messages and a
real calm animation when the message itself is peaceful ("dimanche matin, pas de réveil, je
profite").

## Decision

Amending ADR-0008 and ADR-0018:

- **A ninth emotion, `neutre`** (`Emotion::Neutral`, added last to `Emotion::ALL`): neutral,
  practical or informative messages. `calme` now means peace, rest and serenity.
- **`neutre` is the fallback.** Below the confidence threshold, for unknown ids, and when a theme
  lacks an emotion, the app plays `neutre`. Themes must define `neutre` instead of `calme`.
- **A tiny animation.** The anime theme's `neutre` lets the letters rise briefly, draws a faint
  grey `breeze` frame around the bubble, at most three small grey stars, and **no backdrop** and
  no onomatopoeia: the screen stays as it is.
- **Training data.** In `dataset_emotions.xlsx` the former `calme` sheet is renamed `neutre`
  (its 100 messages are practical ones) and a new `calme` sheet holds 100 peaceful messages written
  by Claude for the owner on 2026-10-24 (provenance in `data/emotions/README.md` and
  `docs/legal.md` L28). `dev_v1.csv` predates this split: when it measures a nine-label head its
  `calme` rows are read as `neutre`; the frozen research files are unchanged.
- **Legacy bundles keep working.** A head without a `neutre` label (seven- and eight-label
  bundles) reads its `calme` scores as `neutre`, since that is what they were trained on; the
  manifest check still compares the head's own ids.

## Consequences

- A nine-label head must be trained and shipped for real calm animations to appear; older bundles
  never produce the new `calme`.
- The development set does not measure calm recall: a separately labelled evaluation set is needed,
  as for `genance`. The threshold (0.2) is kept and must be recalibrated with it.
- Goldens gain `neutre-*` frames; the full-screen backdrop test skips levels without a backdrop.
