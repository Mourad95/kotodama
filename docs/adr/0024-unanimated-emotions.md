# ADR-0024 — A theme can show an emotion as plain text

**Status:** accepted · 2026-10-24 (owner request) · amends ADR-0021 and ADR-0023

## Context

ADR-0021 gave `neutre` a tiny animation, and ADR-0023 turns every sent message into a sticker. The
owner wants a neutral message to look like any chat message: plain text, no sticker, no motion. An
animation that carries no emotion is noise, and it makes the animated messages less special.

Rule 2 of CLAUDE.md forbids an `if emotion == neutre` in the shell or the core: the behaviour must
be data.

## Decision

- **Schema.** `EmotionMotion` gains `animated` (boolean, default `true`, omitted when true). `false`
  means messages with this emotion show as plain text. `themes/schema.json` mirrors it.
- **Theme.** The anime theme sets `"animated": false` on `neutre` (version 0.8.0). Its three levels
  stay: `neutre` is still the fallback scene for exports and previews, and the validation still
  requires it.
- **Core.** `Theme::is_animated(emotion)` answers with the same `neutre` fallback as `Theme::level`;
  `kd-ffi` exposes `Animator::is_animated(emotion_id)`, unknown ids answering like `neutre`.
- **Android shell.** At send time the message records `is_animated`; when false it is a plain sent
  bubble with the text, like a received one but on the sender's side and colour.

## Consequences

- Any theme can choose which emotions are animated without code.
- Low-confidence messages, which fall back to `neutre`, are also plain text: the app animates only
  what it is confident about.
- The iOS shell must apply the same flag when it gets the sticker display.
