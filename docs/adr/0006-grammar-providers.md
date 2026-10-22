# ADR-0006 — One grammar provider per language

**Status:** accepted · 2026-10-22

## Context
Animation intensity depends on spelling and grammar. French first, English next, more later.
Teenagers often mix languages within a conversation.

## Decision
`lt-grammar` exposes a `GrammarChecker` trait; providers are selected per message by a language
detector:
- French: **Grammalecte** (open source, JavaScript) run inside an embedded QuickJS engine.
- English: **Harper** (open source, Rust, offline).
- Other languages: evaluated case by case (LanguageTool is too heavy for on-device use).
Pedagogical tips are content (`tips/<lang>.json`), written and reviewed by humans.
Slang and deliberate abbreviations ("mdr", "jsp") are not errors.

## Consequences
Adding a language = adding a provider and a tips file. The scoring logic is shared.
Licensing of each provider must be checked before shipping (track R3).
