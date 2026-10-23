# ADR-0015 — Effects: a pure scene sampler, a CPU reference renderer, then wgpu

**Status:** accepted · 2026-10-23

## Context
M3 needs animation running at 60 fps on phones, the same result on iOS and Android, golden-frame
tests in CI, and later export to GIF/WebP. A renderer that mixes timing logic with GPU calls cannot be
tested without a GPU, and CI machines have none.

## Decision
1. `kd_fx::scene::sample(level, glyph_count, t_ms, seed) -> Frame` is a **pure function**: no clock,
   no font, no GPU. It returns what to draw (letter transforms, particles, screen effects,
   onomatopoeias, shake) in resolution-independent units. Particles are computed in closed form from
   `t` and a hash of `(seed, emitter, index)`, so any instant can be sampled without stepping.
2. A **CPU reference renderer** (`tiny-skia` for drawing, `skrifa` for glyph outlines, a bundled subset of Noto Sans JP Bold; behind the `cpu-render` feature so the shells do not link it by default; licences in `docs/legal.md`) draws a `Frame` to
   pixels. It is the golden-frame reference in CI and the future GIF/WebP exporter.
3. The on-screen renderer is **`wgpu`** (planned) (Metal/Vulkan) drawing the same `Frame`; it is checked against
   the CPU renderer on a few frames with a tolerance.
4. Photosensitivity limits are enforced when a theme loads, and the sampler has a reduced-motion mode (docs/legal.md L26, which lists what is still open).

## Consequences
- Behaviour lives in one place (the sampler) and is unit-tested without graphics.
- Two renderers must agree; the tolerance test is the guard, and a divergence is fixed in the wgpu one.
- Adding a level or tuning a timing is a JSON change; a new effect *kind* needs the sampler and both renderers.
