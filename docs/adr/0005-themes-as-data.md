# ADR-0005 — Themes are data interpreted by one renderer

**Status:** accepted · 2026-10-22

## Context
Users pick a theme (anime, minimal, ink…); within a theme, the animation is chosen by emotion and
intensity. Themes must be fully custom ("custom animations to the max") yet cheap to add, identical
on both platforms, and shippable without an app update.

## Decision
A theme is a versioned JSON document validated by `themes/schema.json`. For each emotion and each
intensity level (1–3) it declares letter entrances, particles, screen effects, onomatopoeias,
palette and optional named WGSL shaders. `lt-fx` interprets themes; no theme-specific logic in code.
Themes may evoke a genre (e.g. Japanese animation codes: speed lines, impact frames,
onomatopoeias) but never reproduce characters, logos or recognizable assets of existing works.

## Consequences
New themes are content work, downloadable and sellable as packs. The renderer must stay generic:
any need that cannot be expressed in the schema is a schema change reviewed as such.
