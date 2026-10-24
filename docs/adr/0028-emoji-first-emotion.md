# ADR-0028 — Emoji reveal the main emotion

**Status:** accepted · 2026-10-24 · amends [ADR-0008](0008-confidence-threshold.md) and
[ADR-0026](0026-threshold-emoji-and-gif-quality.md) (the threshold no longer applies when a known
emoji is present)

## Context
The owner observed that the model often misreads a sentence, while an emoji says plainly what the
writer feels: 🔥 and 💪 are energy, 😭 is sadness, ❤️ is love. They asked that, when a message
contains an emoji, the emoji decide the main emotion that drives the animation, and that the model
keep handling messages without emoji.

## Decision
1. **The emoji map is data.** `data/emotions/emoji.json` maps each animated emotion id to its
   emoji. `kd_emotion::EmojiMap` embeds and validates it (known ids, no empty emoji, no emoji
   under two emotions). `neutre` has no emoji.
2. **Matching.** U+FE0F is ignored; the longest listed emoji wins at each position, so skin tones
   and other modifiers that follow an emoji do not hide it. The emotion with the most emoji in the
   message wins; a tie goes to the emotion whose emoji comes first.
3. **Emoji first, model second.** `EmotionEngine::analyze` always runs the model (its scores stay
   visible for debugging and measurement), then sets `effective` to the emoji's emotion when there
   is one, whatever the confidence; otherwise `effective` is the thresholded model choice as
   before. `EmotionAnalysis::from_emoji` and the FFI field `EmotionResult.from_emoji` say which
   path decided; the shells may show it.
4. **One vocabulary.** A kd-ffi test checks that every emoji of the anime theme's `emoji_art`
   maps to the emotion its image is named after, so the drawing and the animation agree.

## Consequences
- Messages with emoji are animated with the emotion the writer chose, even when the model is
  unsure. Benchmarks of the model (`research/`) measure `choice`, which this decision does not
  change.
- An emoji used ironically (😂 on a sad message) wins over the text. This is accepted: the writer
  picked it.
- The map was written by Claude at the owner's request. It is a lookup table, not training data,
  so gates L28/L29 of docs/legal.md do not apply to it. Adding an emoji is a data change.
