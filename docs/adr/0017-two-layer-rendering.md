# ADR-0017 — Render the animation in two layers at the screen's resolution

**Status:** accepted · 2026-10-23 · amended by ADR-0019 (sharp effects move to the front layer; no message in the overlay) · display superseded by ADR-0023

## Context
Beta 2 drew each frame at most 540 px wide on the CPU and stretched it to the screen: letters and lines
looked pixelated and "cheap". The overlay also covered the whole window (title, input bar, keyboard) and
its sizes were tied to the frame width only, so it did not adapt to the screen. Rendering the whole frame at
full resolution on the CPU is too slow for 60 fps on a mid-range phone; the wgpu renderer (ADR-0015) is
still to come (M3).

## Decision
1. **Two layers.** `Renderer::render_layer(frame, text, w, h, layer)` draws one of
   `Layer::{Opaque, Over, Back, Front}`:
   - `Back` holds the soft screen effects (tint, rays, speed lines, glow). They have no sharp edges, so the
     shell renders them small (about 0.4 of the size) and upscales them with smoothing.
   - `Front` holds what must stay sharp (letters, particles, onomatopoeias) at close to the screen's resolution.
   - `Front` composited over `Back` equals `Over` (tested). The shell renders both in parallel.
2. **One size unit.** Every size is a share of `size_unit(w, h) = min(w / 0.5625, max(w, h))`, so a frame looks
   the same on narrow, tall and wide areas.
3. **Readable over any backdrop.** In overlay mode the halo contrasts with the ink (dark halo for light ink,
   white otherwise), and all halos are drawn before all inks so a halo never covers a neighbouring letter.
   This refines ADR-0016 point 3.
4. **FFI.** `Animator.render(..., layer: FrameLayer)` replaces the `overlay` boolean.
5. **The overlay covers the message area only**, not the title, input bar or keyboard, and is sized to it.
6. **Adaptive quality.** The Android shell steps down the front scale (1, 0.8, 0.65) and the back scale (0.4,
   0.33, 0.25) when the average frame time exceeds 28 ms, with a 2048 px cap.

## Consequences
- Sharp text at screen resolution at a CPU cost close to beta 2's; to be measured on the phone.
- Two buffers per frame in the shell; the wgpu renderer (M3) keeps the same layer contract and may drop it.
- Golden frames changed (text is smaller relative to the frame); the readability test checks the message band.
