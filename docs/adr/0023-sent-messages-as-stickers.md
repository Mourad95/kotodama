# ADR-0023 — Sent messages show as stickers that play, then stay on their last frame

**Status:** accepted · 2026-10-24 (owner request) · amended by ADR-0024 · supersedes, for the app's display, the
full-screen overlay of ADR-0016, ADR-0017, ADR-0019 and ADR-0020, and the decorated bubble of ADR-0022

## Context

KotoMotion will reach other people through the chat apps they already use, where an animated message
travels as a sticker (an animated WebP or GIF): a square card that plays, then stays on its last
frame. No chat app lets a sticker play over the whole conversation or decorate a text bubble. The
owner asked the beta app to look exactly like that, so what the testers see is what a recipient
will see, and to stop working on the bubble's background.

The last frame of the scene was not a good still: `scene::sample` fades the manga panel out over the
last 450 ms, because the overlay was meant to give the screen back to the bubble. Its last frame is
the message alone on an empty background.

## Decision

- **The sticker sampler.** `kd_fx::scene::sample_sticker` samples the same scene as `sample`, but the
  panel only fades in: it stays to the end, so the last frame is the whole scene with the message.
  With reduced motion every instant is that settled last frame of the full scene, so the sticker is
  a still image (docs/legal.md L26). Golden frames `*-sticker.png` cover every emotion's last frame.
- **A sticker layer.** `FrameLayer::Sticker` renders that scene opaque, square or not, and ignores the
  stage. The shell asks for frames of the sticker's own size, not of the screen.
- **The Android shell.** A sent message is a 220 dp square card, drawn at its pixel size (720 px at
  most). It plays once when sent, then stays on its last frame; a tap plays it again; when it scrolls
  back into view it shows the last frame. TalkBack reads the message text. In the minimal mode it
  shows the still at once. Received messages stay plain bubbles. The overlay, its two-layer
  rendering and the decorated bubble are removed from the shell.
- **The FFI keeps what is now unused.** `render_bubble`, `bubble_look` and the overlay layers stay in
  `kd-ffi` for now: the iOS shell and the export to WebP still have to be decided against this
  display. They will be removed when nothing needs them, in their own change.

## Consequences

- What the app shows is what an exported sticker will be: the same frames, the same last frame.
- The text is part of the image. Accessibility relies on the content description in the app, and
  on the alt text of the sticker once exported.
- The writing level still scales the scene (ADR-0022): a well-written message earns the full scene.
- The screen-sized rendering budget of ADR-0017 no longer applies to the app; a sticker frame is
  much smaller, which leaves room on slower phones.
