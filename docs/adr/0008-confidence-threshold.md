# ADR-0008 — Low-confidence results fall back to `calme`

**Status:** accepted · 2026-10-22

## Context
A wrong emotion (confetti on a sad message) is worse than a neutral animation. The engine returns
a confidence in [0, 1] (1 − normalized entropy of the probabilities).

## Decision
If confidence < 0.2, the app plays `calme`. Chosen on the development set and checked on the test
set: 91.0 % precision on 85 % of messages for the current engine.

## Consequences
The threshold is a data parameter shipped in the model bundle manifest and must be recalibrated
whenever the model or prototypes change, on a validation set larger than the current 70 messages.
