# ADR-0016 — The animation plays over the conversation, then the message settles in its bubble

**Status:** accepted · 2026-10-23 (decided by the owner) · amended by ADR-0019 (the message sits in its bubble from the start; the overlay draws no text)

## Context
The first betas drew the animation on an opaque full-screen background. KotoMotion is meant to be a
keyboard used in messaging apps (WhatsApp, the system messages app), and our own app has a conversation
screen. Android and iOS do not let a keyboard draw over another app.

## Decision
1. **In our app:** sending a message plays its animation full screen on a **transparent** layer over the
   conversation, for a short moment; then the layer disappears and the message sits, still, in its bubble.
2. **From the keyboard, in other apps (M6, M7):** the same renderer produces an animated WebP (transparent
   where the host app supports it) inserted as a message or sticker, so it plays in the host's bubble, not full
   screen.
3. `Renderer::render_over` draws on a transparent background and gives the letters a white halo, so the message
   stays readable over any conversation, light or dark. `Animator.render(..., overlay)` exposes it to the shells (refined by ADR-0017: two layers, contrasting halo).
4. The conversation shown in the app lives in memory only and is never stored (docs/legal.md L18).

## Consequences
- One renderer, two outputs (live overlay, exported animated asset).
- Readability over arbitrary backdrops relies on the halo and, from level 3, on the pale burst; both are tested.
- The animation length is whatever the theme declares, capped later by the "ephemeral animation" decision
  (about 4 to 5 seconds, see the roadmap).
