# ADR-0020 — Effects anchored on the bubble, full-screen manga panels

**Status:** accepted · 2026-10-23 (owner request) · display superseded by ADR-0023

## Context

On a phone the overlay ink framed the middle of the screen (ADR-0019) while the message sat in its
bubble lower down, so the effects missed the message. The owner also asked for a full-screen
coloured background per emotion in the style of manga emotion panels (a vivid flat colour, bold
outlined motifs, big outlined katakana), for a more visible shame effect and for an electric
neon energy effect.

## Decision

Amending ADR-0019:

- **The shell tells the core where the bubble is.** `Animator::render` takes an optional
  `FrameStage` (plain fractions of the frame: x, y, width, height). With a stage, the overlay
  layers call `Renderer::render_layer_on`: the ink, bursts, anchored particles and onomatopoeias
  frame that rect; foreground particles fade where they cross it; the impact flash and the
  backdrop are cut around it, so the bubble stays readable. Without a stage, or for `Opaque`, the
  behaviour is unchanged (centred panel). No text crosses the FFI.
- **Each emotion has a full-screen panel.** The theme's `manga` block gains an optional
  `backdrop` (inner and outer colour, opacity, optional `rays`: 8 to 128 radial speed lines from
  the bubble outwards) drawn as a radial gradient behind everything in the soft layers, and a
  `palette` used by bold, black-outlined props scattered over the screen (`kd_fx::scatter`): few
  and large, like the motifs of a manga panel, not stickers. The bubble panel is tinted from the
  backdrop. Both are data; the schema mirrors the Rust types.
- **Manga lettering.** Onomatopoeias are lettered opaque on a scratch layer (white rim, black drop
  shadow, thick black outline, vertical gradient fill from a light tint to a dark shade of their
  colour) and faded once, so the layers still add up to the overlay. With a stage they dominate
  the panel: each word is sized to the room above or below the bubble (up to 90 % of the width,
  62 % of the room's height, four times the theme scale); when one side is too short its words
  move to the other side, stacked in two rows.
- **Energy is neon.** A `neon` motif (zigzag bolts with an accent glow) and `sparks` particles;
  any flicker stays between 85 % and 100 % brightness for photosensitivity.

## Consequences

- The Android shell reports the bubble's bounds and the overlay's origin in the window, reads
  them every frame, and passes `null` while the bubble is not laid out yet. iOS does not call
  `render` yet.
- Golden frames gain one `*-stage.png` per emotion, and a test checks the bubble stays clear while
  the corners carry the backdrop.
- Translucent effects over a translucent backdrop round to 8 bits at each pass: the Back + Front
  split may now drift from the whole overlay by up to 8 levels (was 4).
