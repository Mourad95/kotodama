# Bundled font

`NotoSansJP-Bold-subset.ttf` is **Noto Sans JP Bold** (SIL Open Font License 1.1, text in `OFL.txt`),
taken from the `@fontsource/noto-sans-jp` 5.3.1 package and reduced with `fonttools` to the glyphs the
app draws: Basic Latin, Latin-1, `œ`/`Œ`, typographic punctuation, `€`, hiragana and katakana
(609 glyphs, 101 KB). No outline was changed. The licence text must ship with the app (docs/legal.md L11): the Android build copies the font and
`OFL.txt` into the app assets, where the bubble lettering uses the font (ADR-0019).
