# ADR-0014 — Verify the bundle checksums once, not at every launch

**Status:** accepted · 2026-10-23 (decided by the owner)

## Context
Hashing the 120 MB model costs 0.7–1.1 s of every cold start on the test phone (Xiaomi M2101K6G).
The checksums protect against a corrupt or incomplete download or update, not against an attacker
who can already write in the app's private storage (that attacker can replace the app's code).

## Decision
`Bundle::open_with_receipt(dir, receipt)` hashes every file on the first open and writes a receipt
(manifest digest + size of each file) to a path chosen by the shell in the app's **private** storage.
Later opens skip the hashing while the manifest and every file size still match the receipt. A
missing, unreadable or stale receipt means a full verification, and a failed verification never
writes a receipt. `Bundle::open` (always hash) stays for tools and tests.

## Consequences
- Cold start is about a second shorter. A model update changes the manifest, so it is verified again.
- Same-size corruption after the first verification is not detected on later launches; accepted
  because the bundle sits in storage only the app can write (shells must not use shared storage in
  production; the spike keeps it there only to push the bundle with `adb`).
- `EmotionEngine::open` takes the receipt path as an optional third argument (`None` = always verify).
