# ADR-0022 — The settled bubble keeps a frozen scene, scaled by the writing level

**Status:** accepted · 2026-10-24 (owner request) · extends ADR-0018 and ADR-0020 · display superseded by ADR-0023

## Context

When the animation ends, the message settles in its bubble (ADR-0016). Until now the bubble kept
only a tint and a few accents, so the panel's world disappeared. The owner wants the animation's
background to stay, frozen, inside the bubble. The writing level (1 to 3, from spelling and
grammar mistakes, M4) must stay visible there too: a well-written message earns the full scene.

## Decision

- **A frozen scene behind a reading card.** When a level has a backdrop, `draw_bubble` paints the
  backdrop (gradient, rays) and, at level 3, the motif's props, all at one fixed moment, clipped
  to the bubble's shape. A pale rounded card with a soft halo then covers the middle, where the
  native text sits. The text's safe area stays pale and opaque in light mode and dark in dark mode
  (tests in `tests/manga.rs`).
- **Scaled by the writing level.** Level 1 has a faint gradient and no rays, level 2 adds the rays
  at a stronger opacity, and level 3 adds the frozen props. The scene's strength comes from the
  level number, not from per-emotion code. A test checks that the frame's ink grows strictly from
  level 1 to level 3 for every emotion with a backdrop.
- **A wider gap between levels.** The panel's strength is 0.3, 0.65 and 1.0 for levels 1 to 3
  (it was 0.57 to 1.0): level 1 shows few, smaller props and thin smoke, so a well-written
  message is clearly richer, in the animation and in the bubble.
- **No backdrop, no scene.** `neutre` keeps its plain bubble (ADR-0021).
- **Same drawing code.** The frozen scene reuses the overlay's backdrop and scatter drawing, so a
  theme change shows in both places and stays data.

## Consequences

- Every bubble golden with a backdrop changes; the bubble costs one extra off-screen layer per
  render. A bubble is drawn once per message, not per frame.
- Until the grammar engine (M4) provides the level, the app's level buttons choose it.
