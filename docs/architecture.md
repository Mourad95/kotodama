# Architecture

## Principles

1. **One Rust core, thin native shells.** Everything that matters (emotion, grammar, scoring,
   rendering, export) lives in Rust and is written once. Swift and Kotlin only host the screens,
   the keyboard extensions and the OS integrations.
2. **A tiny FFI surface.** Shells see a handful of functions. Everything behind them can be
   refactored without touching Swift or Kotlin.
3. **Behaviour as data.** Themes, emotion prototypes and pedagogical tips are versioned data files,
   not code. Adding a theme or an emotion example never requires a code change.
4. **On-device only.** Messages never leave the phone. No network call in the analysis path.
5. **Every claim is tested.** Model parity with Python, golden frames for effects, benchmark
   accuracy in CI where possible.

## Layers

```
┌───────────────────────────── native shells ─────────────────────────────┐
│ apps/ios (Swift)                      apps/android (Kotlin)              │
│  app · keyboard ext · iMessage ext     app · IME keyboard                │
└──────────────────────────────┬──────────────────────────────────────────┘
                               │ UniFFI (generated bindings)
┌──────────────────────────────▼──────────────────────────────────────────┐
│ kd-ffi        public facade: Engine::analyze, render, export            │
├─────────────────────────────────────────────────────────────────────────┤
│ kd-emotion    tokenizer + ONNX model + few-shot head → EmotionScores     │
│ kd-grammar    GrammarChecker providers (FR: in-house, EN: Harper)     │
│ kd-fx         theme data + wgpu renderer + exporters (WebP/MP4/GIF)      │
│ kd-core       shared types and pure logic (emotions, intensity, scoring) │
└─────────────────────────────────────────────────────────────────────────┘
        ▲ data files: model bundle · themes/*.json · tips/<lang>.json
```

Dependency rule: `kd-core` depends on nothing internal. `kd-emotion`, `kd-grammar` and `kd-fx`
depend only on `kd-core`. `kd-ffi` is the only crate that depends on all of them and the only one
the shells link against.

## Data flow for one message

```
text ──► kd-grammar.check(text, lang) ──► issues (spans, suggestions)
  │                                            │
  └──► kd-emotion.analyze(text) ──► scores ────┤
                                               ▼
                           kd-core.intensity(word_count, issues)
                                               ▼
               kd-fx.play(theme, scores.choice, intensity) ──► frames / export
```

- Analysis runs on a **typing pause** (debounce ~300 ms), not on each keystroke.
- If `scores.confidence < threshold` (0.2, see ADR-0008), the app uses `neutre` (ADR-0021).

## Emotion engine (ADR-0003, ADR-0004)

"Typed choice": the engine is trained on labelled examples (nine categories in Dataset emotions, ADR-0021; seven or eight in legacy bundles, whose `calme` is read as `neutre`), and returns a choice, a probability per emotion and a confidence in [0, 1].

The **model bundle** produced by `tools/model-export/` contains:

| File | Content |
|---|---|
| `model_q4.onnx` | EmbeddingGemma, vocabulary pruned to FR+EN, full sentence-embedding graph (transformer + pooling + dense + normalize), 4-bit weights |
| `tokenizer.json` | pruned tokenizer, Hugging Face `tokenizers` format |
| `emotion_head.json` | few-shot linear head trained on Dataset emotions (or explicit legacy prototypes): labels, weights, bias |
| `parity.json` | reference messages with expected probabilities, for the Rust parity test |
| `manifest.json` | versions, source model, checksums, quality metrics |

Enriching Dataset emotions means rerunning the head export, reusing the existing embedding graph.
The editable workbook is the default training source (ADR-0018); see `data/emotions/README.md`.

## Effects engine (ADR-0005)

A theme is a **motion language**: for each emotion and each intensity level it describes the
letters' entrance, particles, screen effects (speed lines, impact flash, shockwave, vignette,
shake), onomatopoeias and palette. The renderer interprets this data; it never contains
theme-specific code. Custom WGSL shaders are referenced by name from the theme.

## Grammar (ADR-0006)

`GrammarChecker` trait with one provider per language, selected by a language detector per
message (teenagers mix French and English). Pedagogical one-line tips live in `tips/<lang>.json`.

## Repository layout

```
crates/            Rust workspace (kd-core, kd-emotion, kd-grammar, kd-fx, kd-ffi)
apps/ios/          Swift shell (Xcode project added in M2)
apps/android/      Kotlin shell (Gradle project added in M2)
themes/            theme data + JSON Schema
tools/model-export Python pipeline producing the model bundle
research/          benchmarks and experiments, with their protocols and results
docs/              roadmap, architecture, ADRs
```
