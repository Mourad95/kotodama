# Product — what KotoMotion is

Read this first. It defines the product so that any agent or developer builds the right thing.

## One sentence

KotoMotion is a messaging companion that **animates each message according to its emotion** — in a
theme chosen by the user (anime-style first) — and **unlocks the full animation only when the
message is correctly written**, so that teenagers want to spell and write well.

The name joins *koto* (言, "word", from *kotodama* 言霊, the Japanese idea that words carry spirit and
power) and *motion*: the better the words, the more power the message has.

## The core loop

1. The user types a message (in a keyboard, or in the app).
2. On device, the **emotion engine** picks one of 8 supported emotions: `joie`, `colere`, `energie`,
   `suspense`, `tristesse`, `amour`, `calme`, `genance` (`calme` is also the fallback when the engine is unsure).
3. On device, the **grammar checker** finds spelling and grammar issues.
4. The number of issues gives an **intensity** (3 levels): no error → full animation, one error →
   medium, two or more → low (faulty words glitch).
5. The **effects engine** plays the theme's animation for (emotion, intensity).
6. The user can tap an issue, see a one-line explanation, fix it, and watch the animation grow.
   This is the learning mechanic: *correct spelling is the unlock.*
7. The message is shared as an animated asset (GIF/WebP/video) or a magic link that plays it.

Success looks like: a teenager rewrites "sa va a 18h" into "ça va à 18h" on their own, because the
animation gets crazier.

## Who it is for

- **Primary:** teenagers who text a lot and write quickly and badly. They want style and
  self-expression, not school. The app must never feel like a correction tool.
- **Secondary:** their parents (trust, privacy), and any adult who likes expressive messages.
- **Market order:** French first, then English, then other languages.

## What makes it different

- Animations are **fully custom per theme, emotion and intensity**, not generic reactions. A theme
  is data (JSON), interpreted by one renderer (ADR-0005).
- The reward is tied to **writing quality**, which is the educational angle.
- Everything runs **on device**: no message leaves the phone (ADR-0007). This is a product
  promise, a legal requirement for minors, and an engineering constraint.

## Distribution surfaces (in order)

| Order | Surface | Notes |
|---|---|---|
| 1 | Standalone app on one platform (M5) | Beta with ~10 teens. |
| 2 | Android keyboard (IME, M6) | Can insert GIF/WebP via the commit-content API. |
| 3 | iOS keyboard + iMessage extension (M7) | Keyboard extension has a tight memory limit: needs a small model (research track R2). |
| later | Magic web link, sticker/GIF export | The receiver does not need the app to see the animation. |

TikTok/Instagram plugins are **not possible** (Meta Spark is discontinued); the keyboard avoids
leaving the host app.

## The emotions, defined

Authoritative definitions and examples live in `research/emotion-benchmark/prototypes.json`.
Short version: `joie` happiness/pride, `colere` anger/blame, `energie` motivation/hype,
`suspense` curiosity/secret/rumour, `tristesse` sadness/failure, `amour` affection/warm thanks,
`calme` neutral/practical/unsure. `genance` covers embarrassment and awkwardness (ADR-0017).
The legacy prototypes describe seven labels; Dataset emotions adds the eighth. Existing model
bundles keep their original categories until a new head is trained and installed.

## Themes

A theme gives, for every emotion and each of 3 intensity levels: letter entrance, particles,
screen effects (speed lines, impact flash, shockwave, shake…), onomatopoeia and colours.

- The first theme is **anime-style**: inspired by the *genre's* visual language only.
- **Never** reproduce or name an existing work, character, logo, scene or text (legal rule L14).
- Adding a theme is a data change, not code (rule 2 of `CLAUDE.md`).

## Non-goals (do not build these)

- No server-side analysis of messages, no accounts required to use the core loop, no ads, no trackers.
- No social network, no feed, no public profiles.
- No general chatbot or LLM text generation.
- No grading or scoring of the child as a person: the app rewards a message, it does not rank users.
- No copy of any existing anime/manga work.

## Quality bar ("a clean app")

- **Readable first:** animation never hides the text; the message stays legible at every intensity.
- **Fast:** analysis ≤ ~50 ms per message on a mid-range phone; animation starts without visible lag.
- **Small:** model bundle ≤ 150 MB (≤ iOS keyboard limits via distillation later).
- **Honest:** low-confidence emotion falls back to `calme` rather than a wrong animation (ADR-0008).
- **Private by construction:** see `docs/legal.md`, which is binding.
- **Maintainable:** behaviour is data; one Rust core; thin native shells; every architecture
  decision has an ADR.

## Glossary

| Term | Meaning |
|---|---|
| Typed choice | Emotion engine: embedding model + few-shot linear head (ADR-0003). |
| Intensity | 1 low, 2 medium, 3 full; derived from the number of writing issues. |
| Theme | JSON file describing all animations; validated by `themes/schema.json` and `kd-fx`. |
| Bundle | Model files + head + manifest, built by `tools/model-export`, not stored in git. |
| Gate / rule | Legal items in `docs/legal.md`; a gate blocks a milestone until closed. |
| Shell | The thin iOS/Android app around the Rust core (`apps/ios`, `apps/android`). |
