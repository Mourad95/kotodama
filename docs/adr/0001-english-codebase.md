# ADR-0001 — Code, docs and commits in English

**Status:** accepted · 2026-10-22

## Context
The founder works in French, the first market is French, but the project is meant to be
maintained by humans and AI agents of any language and to go international (English next).

## Decision
Identifiers, comments, documentation, ADRs, commit messages and issues are written in English.
User-facing strings are localized data (Project Fluent), French first. Emotion labels keep their
historical French identifiers in data files (`joie`, `colere`, `energie`, `suspense`, `tristesse`,
`amour`, `calme`) for compatibility with the benchmark; code maps them to an English enum.

## Consequences
Contributors and tools work with a single language. Benchmark files stay readable as-is.
