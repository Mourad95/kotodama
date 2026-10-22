# Legal register (track R3)

This file freezes the legal constraints of KotoMotion. A **rule** is binding for the code and the
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
| L7 | Grammalecte is GPL-3.0 (confirmed): never embed, link or ship it or its files. The French checker is built in-house (ADR-0009) on the MPL-2.0 Hunspell dictionary; Grammalecte may be used on a developer machine only as a measurement reference. | Rule (gate closed 2026-10-22) | — |
| L8 | Harper (English) is Apache-2.0: allowed. | Rule | — |
| L9 | Every model shipped in the app is under an OSI-approved permissive licence (Apache-2.0, MIT, BSD) — ADR-0010. Gemma Terms of Use, CC-BY-NC and custom licences are baselines only, never shipped. Weights come only from the publisher's official repository, with SHA-256 in the manifest; licence text ships in the app. | Rule | — |
| L10 | Every dependency is checked by `cargo deny` (licence allow-list) in CI; adding a copyleft licence needs an ADR. | Rule | — |
| L11 | Fonts embedded in the app (including Japanese kana/kanji) must allow embedding and redistribution; default choice is the Noto family (OFL). Licence texts ship in the app. | Rule | — |

## 3. Trademark

| # | Rule / gate | Type | Blocks |
|---|---|---|---|
| L12 | The product name is **KotoMotion** (decided 2026-10-22, replacing "Kotodama", which same-name apps already use). The owner reports the name as free; record the INPI and EUIPO searches (classes 9 and 42) and the App Store / Google Play check here, with their date, before the first public release. | Gate | Public launch |
| L13 | Reserve the domain and the store/developer handles once the search is clean. | Gate | Public launch |

## 4. Copyright of content

| # | Rule / gate | Type | Blocks |
|---|---|---|---|
| L14 | Themes are original. They may reuse the genre's visual language (speed lines, impact frames, auras, onomatopoeia) but never a character, design, logo, scene or text of an existing work, and never name one. | Rule | — |
| L15 | Exported GIFs and stickers contain only KotoMotion-made assets. | Rule | M5 |
| L16 | Emotion examples (`prototypes.json`) are written by us; no copied lyrics, quotes or third-party datasets without a recorded licence (ADR-0003, research track R1). | Rule | — |
| L17 | Apache-2.0 weights remove licence obligations but the training data of the model is not public: ask the lawyer whether this residual provenance question matters for the product. | Gate | Public launch |

## Review process

- Every gate gets a dated note with its evidence (licence text, search result, lawyer's answer) added below.
- A rule changes only through an ADR.
- Before public launch: one review by a lawyer specialised in digital law and data protection, using the product description and this register.

## Evidence log

- **L7 closed, 2026-10-22.** Debian copyright file for Grammalecte lists GPL-3 as the default licence (MPL-2.0 only for a few files); grammalecte.net states GPL 3. The Hunspell French "revised" package lists MPL-2.0 (Arch Linux package metadata). Decision: ADR-0009.
