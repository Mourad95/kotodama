# ADR-0025 — French speller on spellbook, and sharing a sticker as a GIF

**Status:** accepted · 2026-10-24 · amends [ADR-0009](0009-own-french-checker.md) (spelling part)

## Context
The owner wants to try the app with WhatsApp as soon as possible, with the writing level coming from
real spelling mistakes instead of a manual setting. Two pieces were missing: the spelling half of the
French checker (ADR-0009, point 1) and a way to send a sticker to another app.

## Decision
1. **Reader.** Spelling is checked by the `spellbook` crate (0.4, MPL-2.0, pure Rust, no network),
   which reads Hunspell `.aff`/`.dic` files. `kd-grammar::FrenchSpeller` wraps it behind
   `GrammarChecker`; issues have rule id `spelling` and no suggestions (suggesting costs tens of
   milliseconds, so `suggest` is a separate call made only for the first mistake, off the main thread).
2. **Dictionary.** The "toutes variantes" (comprehensive) French dictionary 7.7 replaces the
   "revised" one named in ADR-0009: teenagers write both classic and 1990 spellings
   ("événement", "évènement") and neither must count as a mistake. Same author, same MPL-2.0 licence,
   files unmodified, checksums in `grammar/fr/hunspell/README.md`; the licence text ships with them.
3. **What is never a mistake.** Words of `grammar/fr/allowlist.txt` (slang and abbreviations, data:
   ADR-0009 point 3), elongations whose short form is known ("troooop"), known elisions and hyphenated
   words whose parts are known, and anything containing a digit, a link, an address, a @mention or a
   #hashtag. Only the first 2000 characters are checked.
4. **Intensity.** The shell sends the number of mistakes to `intensity_level` (0 → 3, 1 → 2, 2 or
   more → 1); the tester can still force a level.
5. **FFI.** `kd_ffi::Speller` (built from the three files' text) returns `SpellingIssue { start, end,
   word }` with offsets in **UTF-16 code units**, the unit of Android and iOS strings, so the shells
   underline without converting. Plain data only (CLAUDE.md rule 1).
6. **Sharing.** `Animator::export_gif` encodes the sticker (ADR-0023) as a looping GIF (`gif` crate,
   MIT or Apache-2.0): it plays once at about 14 fps, holds the settled scene 2.5 s, at most 80
   frames, one still frame with reduced motion, 720 px at most. Messaging apps loop GIFs, which the
   hold makes readable. The Android shell writes it to `cache/shared/` (replaced at each share) and
   hands it to the system share sheet through a non-exported `FileProvider` with a per-URI read grant;
   an unanimated message (`neutre`) is shared as plain text. No permission is added.

## Consequences
- The writing level is live in the app: M4's "intensity changes live" holds for spelling; grammar
  rules (`grammar/fr/rules.json`) are still to write.
- About 1.5 MB of dictionary in the app; loading takes tens of milliseconds, checking a message is
  far below the 50 ms budget on desktop (to measure on a phone).
- The share sheet is the only way a message leaves the phone, and only when the user picks an app
  (docs/legal.md L25). The GIF holds only KotoMotion-made drawings, the bundled Noto font and the
  user's own text (L15).
- MPL-2.0 now covers a Rust dependency too (`spellbook`); `deny.toml` allows it. We do not modify its
  files, so there is nothing to publish.
- No false-positive measurement yet: the own French error corpus of ADR-0009 is still needed.
