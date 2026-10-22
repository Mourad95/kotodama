# Legal register (track R3)

This file freezes the legal constraints of Kotodama. A **rule** is binding for the code and the
content from now on; a **gate** must be closed (with evidence) before the milestone it blocks.
None of this is legal advice: every gate ends with a review by a qualified lawyer before public launch.

## 1. Minors and personal data

| # | Rule / gate | Type | Blocks |
|---|---|---|---|
| L1 | Message text is analysed on device only; it is never sent, logged or stored off device (ADR-0007). No analytics or crash SDK may receive message content. | Rule | — |
| L2 | No advertising SDK and no third-party tracker in any shell. | Rule | — |
| L3 | Age gate and parental consent under 15 (France, GDPR art. 8) designed with the lawyer before the first external beta. | Gate | M5 beta |
| L4 | Privacy policy and store privacy labels written from what the code really does, and kept in sync by review at each release. | Gate | M5 beta |
| L5 | iOS keyboard "Allow Full Access" is requested only if a feature truly needs it; the justification is written before M7. | Gate | M7 |
| L6 | Apple and Google rules for apps aimed at children and for third-party keyboards checked against the final design. | Gate | M5, M6, M7 |

## 2. Third-party licences

| # | Rule / gate | Type | Blocks |
|---|---|---|---|
| L7 | Grammalecte is believed to be GPL-3.0: do not embed it until the licence of the engine and of the dictionaries is confirmed compatible with store distribution. Otherwise choose another French provider (ADR-0006). | Gate | M4 |
| L8 | Harper (English) is Apache-2.0: allowed. | Rule | — |
| L9 | Gemma: ship the required notice and pass on the prohibited-use policy; fetch the weights only from Google's official repository and record the checksum in the bundle manifest. The Unsloth mirror was used for benchmarking only. | Gate | M2 release bundle |
| L10 | Every dependency is checked by `cargo deny` (licence allow-list) in CI; adding a copyleft licence needs an ADR. | Rule | — |
| L11 | Fonts embedded in the app (including Japanese kana/kanji) must allow embedding and redistribution; default choice is the Noto family (OFL). Licence texts ship in the app. | Rule | — |

## 3. Trademark

| # | Rule / gate | Type | Blocks |
|---|---|---|---|
| L12 | The product name is **Kotodama** (decided). Search INPI and EUIPO (classes 9 and 42) and the App Store / Google Play before the first public release; same-name apps already exist (diary, voice changer), so the store title is "Kotodama – <descriptor>". | Gate | Public launch |
| L13 | Reserve the domain and the store/developer handles once the search is clean. | Gate | Public launch |

## 4. Copyright of content

| # | Rule / gate | Type | Blocks |
|---|---|---|---|
| L14 | Themes are original. They may reuse the genre's visual language (speed lines, impact frames, auras, onomatopoeia) but never a character, design, logo, scene or text of an existing work, and never name one. | Rule | — |
| L15 | Exported GIFs and stickers contain only Kotodama-made assets. | Rule | M5 |
| L16 | Emotion examples (`prototypes.json`) are written by us; no copied lyrics, quotes or third-party datasets without a recorded licence (ADR-0003, research track R1). | Rule | — |

## Review process

- Every gate gets a dated note with its evidence (licence text, search result, lawyer's answer) added below.
- A rule changes only through an ADR.
- Before public launch: one review by a lawyer specialised in digital law and data protection, using the product description and this register.

## Evidence log

_No gate closed yet._
