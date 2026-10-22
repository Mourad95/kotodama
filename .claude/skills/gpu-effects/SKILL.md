---
name: gpu-effects
description: Load when working on the Kotodama effects engine (kd-fx) — the wgpu renderer, themes and their JSON schema, particles, screen effects, text rendering and CJK fonts, golden-frame tests, export to GIF/WebP/MP4, frame-rate budgets, or accessibility of motion (reduced motion, photosensitivity). Covers how to keep themes as data and animations legible and safe.
---

# Effects engine

`kd-fx` turns (theme, emotion, intensity) into animation. A theme is **data** interpreted by one
renderer (ADR-0005); the renderer never contains theme-specific code.

## Design rules
- **Adding a theme or a level is a data change.** If the schema cannot express it, extend the schema
  deliberately: Rust types are authoritative, mirror them in `themes/schema.json`, bump
  `SCHEMA_VERSION` with a migration if breaking.
- **Pure function of (theme level, time, seed) → frame.** No wall-clock, no hidden randomness.
  The same inputs give the same pixels; that is what makes golden-frame tests possible.
- **Legibility first.** The message text is always drawn on top, with enough contrast against any
  background effect, at every intensity. An effect that hides the words is a bug.
- Timeline model: a level is a set of effects with `at_ms` / `duration_ms`; the renderer samples it
  at time `t`. Keep durations in milliseconds and name units in fields.

## Rendering
- `wgpu` on Metal (iOS) and Vulkan (Android). Offscreen rendering for export uses the same code
  path as on-screen rendering.
- Text: shape with a text-layout library, render through a glyph atlas. Bundle a **subset** of a
  CJK-capable font for the onomatopoeias (kana, a few kanji) with a licence allowing embedding
  (Noto under OFL is the default, gate L11). Never depend on system fonts for theme text.
- Particles: bounded counts per level, set in the theme; the shell lowers them in low-power mode.
- Custom shaders are referenced by name from the theme, never inlined from user data.

## Budgets
- 60 fps on a mid-range phone; frame time well under 16 ms with headroom for heat. Measure on
  devices (M3 exit criteria), not only on a desktop GPU.
- Export size: messaging apps cap attachments; a full-intensity GIF/WebP must stay small (set a
  target in M5 from real apps). Use palette quantization and a capped frame count for GIF; prefer
  animated WebP or short video when the host supports it.

## Tests
- **Golden frames:** render fixed (theme, emotion, intensity, t, seed) tuples and compare to
  reference images with a small tolerance. Run on a software adapter in CI so results do not depend
  on a GPU; update goldens only on purpose, in their own commit.
- Schema tests: a valid theme passes, wrong level order / missing `calme` / bad colour / unknown
  field are rejected (see `kd-fx` tests).
- Property: no effect exceeds its declared duration; all colours are `#RRGGBB`.

## Accessibility and safety (not optional)
- **Photosensitivity:** `impact_flash`, rapid shake and strobing speed lines can trigger seizures.
  Keep flashes below the accepted threshold (no more than three flashes per second, and limit the
  flashed area and contrast), cap `strength`, and test every theme against that rule. Teenagers are
  the audience; err on the safe side.
- **Reduced motion:** honour the OS setting (Reduce Motion on iOS, animator-duration scale on
  Android): show the message with a minimal fade and no shake, flash or full-screen effect.
- Provide a setting to lower effect intensity, and never make the learning feedback depend on motion.

## Legal
Themes are original (rule L14): inspired by the genre's visual language only; never a character,
design, logo, scene, text or name from an existing work. Exported assets contain only Kotodama-made
elements (L15).
