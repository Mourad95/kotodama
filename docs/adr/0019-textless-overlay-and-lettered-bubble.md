# ADR-0019 — Textless overlay, lettered bubble, sharp front layer

**Status:** accepted · 2026-10-23 (owner request)

## Decision

Amending ADR-0016, ADR-0017 and ADR-0018:

- **The overlay never draws the message.** The `Over`, `Back` and `Front` layers draw only the
  effects, particles, manga ink and onomatopoeias, never the letters or the central panel. The
  words live in the conversation bubble from the first frame. `Opaque` (export, golden frames)
  still draws the whole card with the message.
- **The bubble is lettered like a manga effect.** `render_bubble` stamps the level's
  onomatopoeias on the bubble's corners, and the shell draws the native text with the bundled
  Noto Sans JP Bold font, an accent extrusion and a slant scaled by the level's manga depth, and a
  halo outline. `Animator::bubble_look` exposes only colours and the slant and extrusion numbers the
  renderer itself uses (`kd_fx::letter_slant`, `extrusion_steps`, `EXTRUSION_STEP`), so the rule has
  one source; `accent_color` is removed in its favour (plain data); text stays native for wrapping and TalkBack, and no message text crosses the FFI.
  The font and its OFL licence are packaged as app assets (docs/legal.md L11).
- **Sharp edges stay sharp.** Screen effects are split by `ScreenKind::is_soft`: impact flash,
  tint and spotlight are soft and drawn in `Back`, which the shell may render smaller; speed
  lines and shockwaves have hard edges and move to `Front`, which is always drawn at screen
  resolution. `Over` keeps the same stacking (soft, then sharp). The bubble decoration renders at
  its laid-out size (capped at 2048 px per side) with high-quality filtering.

## Consequences

- The overlay no longer shows the text being written; the bubble's lettering and its stamps carry
  the message's style from the start. The overlay ink frames the middle of the screen, not the
  bubble's position in the conversation.
- Each bubble keeps its own decoration bitmap, bounded to 4 MP (16 MB) and 2048 px per side, so
  large accessibility fonts cannot grow it without limit.
- The front layer costs more to draw now that it is never downscaled; performance on a real
  phone remains an M3 exit criterion.
- Gentle mode and reduced motion are unchanged.
