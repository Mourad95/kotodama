# ADR-0007 — All analysis on device

**Status:** accepted · 2026-10-22

## Context
Users are mostly minors; messages are private. GDPR requires parental consent under 15 in France
for consent-based processing. Keyboards must work offline.

## Decision
Emotion detection, grammar checking, scoring and rendering run on device. No message text is sent
to any server. Hosted models may only be used offline, on synthetic or consented data, to produce
training material (subject to their terms).

## Consequences
No per-user inference cost; privacy is a product argument. Model size and latency budgets are hard
constraints (see roadmap M2). Analytics, if any, are aggregated and contain no message content.
