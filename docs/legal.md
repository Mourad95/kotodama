# Legal register (track R3)

This file freezes the legal constraints of KotoMotion. A **rule** is binding for the code and the
content from now on; a **gate** must be closed (with evidence) before the milestone it blocks.
None of this is legal advice: every gate ends with a review by a qualified lawyer before public launch.

## 1. Minors and personal data

| # | Rule / gate | Type | Blocks |
|---|---|---|---|
| L1 | Message text is analysed on device only; it is never sent, logged or stored off device (ADR-0007). No analytics or crash SDK may receive message content. | Rule | — |
| L2 | No advertising SDK and no third-party tracker in any shell. | Rule | — |
| L3 | Age gate and parental consent under 15 (France, GDPR art. 8): **to be confirmed with the lawyer** whether they are needed at all, since the app processes no personal data (L18, L24: nothing leaves the phone, nothing is kept, no account). Until confirmed, design for the stricter case; the store rules for apps aimed at children (L6) may require something regardless of the GDPR. | Gate | M5 beta |
| L4 | Privacy policy and store privacy labels written from what the code really does, and kept in sync by review at each release. | Gate | M5 beta |
| L5 | iOS keyboard "Allow Full Access" is requested only if a feature truly needs it; the justification is written before M7. | Gate | M7 |
| L6 | Apple and Google rules for apps aimed at children and for third-party keyboards checked against the final design. | Gate | M5, M6, M7 |
| L18 | **No message content is kept.** No text, word, fragment, correction, emotion result or keystroke history is stored beyond the screen session, logged, or sent anywhere. Nothing derived from a message (embedding, score, error list) is persisted either. | Rule | — |
| L19 | **Monitoring is technical and anonymous only:** crash type and stack, app version, OS version, device class, model load and inference latency, memory. No message content, no account, no advertising or device identifier. Use the store's built-in reporting (Play Console vitals, App Store crash reports) first; any monitoring SDK needs an ADR and must be configured to collect nothing else. | Rule | — |
| L20 | Android: auto-backup is disabled, so nothing app-side is copied to the cloud. | Rule | — |
| L21 | The model is shipped with the app through the store (APK or Play Asset Delivery; App Store equivalent), not downloaded from our own server. | Rule | — |
| L22 | Beta feedback and support data are kept apart from the app, collected only when the user chooses to send it, and never include message content unless the user pastes it. | Rule | M5 beta |
| L23 | No new test set (test_v3) is collected from real teenagers without parental consent. Until then test data is written by us. | Gate | test_v3 |
| L24 | No magic-link or email sign-in in v1: the app works without any account. | Rule | — |
| L25 | Export of an animation goes through the system share sheet, which the user controls; the app has no upload feature of its own. | Rule | — |

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
- **L18-L25 recorded, 2026-10-22.** Decided by the owner: nothing about a message is kept; only anonymous technical monitoring; MPL-2.0 for the spelling dictionary accepted (ADR-0009). The owner does not want to pursue the EmbeddingGemma 2 model-card question (Prohibited Use Policy) with a lawyer for now; the weights are Apache-2.0 (ADR-0010, ADR-0011).
