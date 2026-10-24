# Bundled fonts

`NotoSansJP-Bold-subset.ttf` is **Noto Sans JP Bold** (SIL Open Font License 1.1, text in `OFL.txt`),
taken from the `@fontsource/noto-sans-jp` 5.3.1 package and reduced with `fonttools` to the glyphs the
app draws: Basic Latin, Latin-1, `œ`/`Œ`, typographic punctuation, `€`, hiragana and katakana
(609 glyphs, 101 KB). No outline was changed. The licence text must ship with the app (docs/legal.md L11): the Android build copies the font and
`OFL.txt` into the app assets, where the bubble lettering uses the font (ADR-0019).

`NotoColorEmoji-base.ttf` is **Noto Color Emoji** 2.047 (SIL Open Font License 1.1, text in
`OFL-NotoColorEmoji.txt`), taken from the Ubuntu `fonts-noto-color-emoji` 2.047 package and reduced to
the emoji reachable through its character map, without the `GSUB` sequences (skin tones, flags, ZWJ
families) that the renderer cannot shape (1 494 glyphs, 3 MB; ADR-0026). No bitmap was changed:

```bash
pyftsubset NotoColorEmoji.ttf --unicodes='*' --layout-features='' --no-layout-closure \
  --drop-tables+=GSUB,vhea,vmtx --output-file=NotoColorEmoji-base.ttf
```

The renderer draws a character with Noto Sans JP when it has it, else with the emoji font. The Android
build copies `OFL-NotoColorEmoji.txt` into the app assets with the other licence texts.
