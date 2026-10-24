# ADR-0009 — French checker built in-house on permissive components

**Status:** accepted · 2026-10-22 · supersedes the French provider choice of [ADR-0006](0006-grammar-providers.md) · spelling amended by [ADR-0025](0025-french-speller-and-gif-share.md) (comprehensive dictionary, `spellbook` reader)

## Context
ADR-0006 chose Grammalecte for French. Checking its licences (legal gate L7) showed that the
engine and most of its files are **GPL-3.0** (Debian copyright file, grammalecte.net). Embedding it
in a closed-source app distributed on the App Store and Google Play, or in a keyboard, is not
acceptable. Alternatives considered:

- Open-source the whole app under GPL-3: rejected (opens our code, store-terms friction).
- Ask the author for a commercial licence: uncertain, single-author project.
- Run Grammalecte on a server: rejected, message text would leave the phone (ADR-0007) and the
  users are minors.
- LanguageTool: Java, too heavy for on-device use, hard on iOS.

## Decision
The French provider is built in-house behind the existing `GrammarChecker` trait:

1. **Spelling:** a Hunspell-format reader in Rust over the French "revised" Hunspell dictionary
   (MPL-2.0). Dictionary modifications, if any, are published as MPL requires. The Rust reader's
   licence is checked when it is chosen.
2. **Grammar:** a rule set of our own, written as **data** (`grammar/fr/rules.json`) and interpreted
   by `kd-grammar`, targeting the most frequent errors of teenagers: homophones (a/à, et/est,
   ça/sa, ce/se, on/ont, ces/ses/c'est/s'est, son/sont), simple agreements, missing apostrophes.
   Rules are original work; no rule text is copied from GPL sources.
3. **Slang and abbreviations** ("mdr", "jsp", "tkt") come from an allow-list, never flagged.
4. **Grammalecte is never shipped or linked.** It may be run on a developer machine only, as an
   independent reference to measure our detection rate. Its output is not copied into our rules.
5. English stays on Harper (Apache-2.0).

## Consequences
- Milestone M4 becomes "write and test the French rule set + Hunspell reader" instead of
  "embed QuickJS".
- Coverage is smaller at first. This is acceptable: intensity only needs an error count, and the
  rule set grows as data without code changes (rule 2 of `CLAUDE.md`).
- We own the quality bar: a labelled French error corpus of our own is needed (new test set,
  written independently, same hygiene as the emotion benchmark).
- Legal gate L7 is closed; see `docs/legal.md`.
