# ADR-0026 — Confidence threshold 0.5, colour emoji in stickers, sharper GIFs

**Status:** accepted · 2026-10-24 · amends [ADR-0008](0008-confidence-threshold.md) (threshold) and
[ADR-0025](0025-french-speller-and-gif-share.md) (GIF settings)

## Context
After trying the app with WhatsApp, the owner reported three problems: the emotion is often wrong,
an emoji typed in a message shows as an empty box in the sticker, and the shared GIF looks poor.

## Decision
1. **Threshold 0.5.** Below a confidence of 0.5 (1 − normalized entropy) the result is `neutre`,
   which is not animated (ADR-0021, ADR-0024). The value lives in the bundle manifest (`threshold`,
   written by `tools/model-export/export_mobile.py`) and in `EmotionEngine::DEFAULT_THRESHOLD`.
   Measured on the dev set only (`dev_v1.csv`, 70 messages, current bundle):

   | Threshold | Precision of animated results | Messages animated |
   |---|---|---|
   | 0.2 | 83.3 % | 94 % |
   | 0.4 | 83.1 % | 84 % |
   | 0.5 | 86.3 % | 73 % |
   | 0.55 | 88.6 % | 63 % |

   Most remaining "errors" above 0.5 are `calme` predicted `neutre` (the dev set predates `neutre`;
   neither is animated). We prefer a missed animation to a wrong one. The test set was not used.
2. **Colour emoji.** `kd-fx` embeds Noto Color Emoji 2.047 (OFL 1.1), reduced to the emoji of its
   character map (3 MB, bitmaps unchanged, see `crates/kd-fx/assets/fonts/README.md`). A character
   missing from Noto Sans JP is drawn from that font's PNG bitmap (`skrifa` bitmap strikes, decoded
   by `tiny-skia`, cached per glyph), with the same motion as the letters. Without shaping, skin-tone
   modifiers, ZWJ, variation selectors and the keycap mark are hidden (zero width): "👍🏽" shows as
   "👍", a ZWJ family as its members side by side. Flags are not supported.
3. **GIF settings.** 600 px (was 480), 20 fps (was about 14), at most 90 frames, quantization speed
   10. A pixel equal to the previous frame's is written transparent and the frame keeps what was
   below, so only what moves is stored. On desktop, a 25-character joy sticker is about 4 MB and
   takes about 4 s to encode (13.9 MB and 19 s with per-frame full images at 720 px and 25 fps).

## Consequences
- About a quarter of messages are no longer animated; the dev measurement must be redone when the
  head or the examples change, and on the test set only at milestone measurements.
- The app grows by 3 MB. Combined emoji (skin tones, ZWJ sequences, flags) need a shaper; to
  revisit with the GPU renderer (M3).
- Encoding is CPU work of several seconds on a phone, off the main thread; to measure on a real
  device. MP4 would be smaller but is not needed yet.
- The shared GIF may contain emoji glyphs of the bundled Noto font, typed by the user (L15 evidence
  in docs/legal.md).
