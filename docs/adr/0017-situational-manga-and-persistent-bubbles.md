# ADR-0017 — Situational manga illustrations and persistent bubble ink

**Status:** accepted · 2026-10-23 (owner-approved visual direction)

## Decision

The anime theme uses a different original manga composition for each situation: radiance,
pressure, orbit, tension, rainfall, blossom, breeze and blush. The new `genance` emotion is
serialised as the eighth label. Existing seven-label model bundles continue to load; they do
not acquire embarrassment detection until a new eight-label head is trained and installed.

`Level.manga` is an optional schema-v1 extension with a motif, highlight colour and depth.
Legacy levels still load. The scene sampler exposes illustration progress and deterministic
particle depth; the CPU renderer draws back particles, ink, offset letter outlines, foreground
ribbons and foreground particles. Motifs describe drawing primitives, not theme-specific
branches. No pretrained imagery or generated mockup is bundled in the app.

Animations last at most 4.5 seconds. Letter stagger is compressed for long messages so every
letter finishes within the cap. Long text is fitted into the central stage. Reduced motion
remains a plain fade; gentle mode softens illustration strength and removes jarring effects.
Device performance and GPU equivalence remain M3 exit criteria, not claims of this change.

Amending ADR-0016: after the overlay ends, the native conversation bubble retains static ink
contours and sparse emotion-specific marks. The shared Rust renderer exposes a decoration-only
RGBA image; the Android shell draws native text above it for accessibility and wrapping.
The decoration adapts to measured bubble dimensions and light/dark mode, renders off the UI
thread and never loops. A rendering failure falls back to a readable plain bubble.

## Training source

The owner's editable `data/emotions/dataset_emotions.xlsx` becomes the default training input
for `tools/model-export/export_mobile.py`. The eight category sheets are authoritative; the
combined snapshot is ignored. Import validates labels and duplicates without modifying the
workbook. Appended rows are read on every export. Legacy prototype training remains explicit.
The new source is separate from the frozen research corpus and historical benchmark results.
The model graph and device bundle must be available to train and install the updated head;
source integration alone does not change the installed model.
