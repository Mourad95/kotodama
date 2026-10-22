# ADR-0006 — One grammar provider per language

**Status:** accepted · 2026-10-22 · French provider superseded by [ADR-0009](0009-own-french-checker.md)

## Context
Animation intensity depends on spelling and grammar. French first, English next, more later.
Teenagers often mix languages within a conversation.

## Decision
`kd-grammar` exposes a `GrammarChecker` trait; providers are selected per message by a language
detector:
- French: **Grammalecte** (open source, JavaScript) run inside an embedded QuickJS engine.
- English: **Harper** (open source, Rust, offline).
- Other languages: evaluated case by case (LanguageTool is too heavy for on-device use).
Pedagogical tips are content (`tips/<lang>.json`), written and reviewed by humans.
Slang and deliberate abbreviations ("mdr", "jsp") are not errors.

## Consequences
Adding a language = adding a provider and a tips file. The scoring logic is shared.
Licensing of each provider must be checked before shipping (track R3). Open point: Grammalecte is
believed to be GPL-3.0, which may rule out embedding it in a closed-source store app; Harper is
Apache-2.0. The French provider choice is provisional until this is settled.
