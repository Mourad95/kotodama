---
name: minors-privacy
description: Load when a KotoMotion change touches user data, logging, analytics, crash reports, accounts, permissions, consent, onboarding, store listings or privacy labels, sharing/export, or anything that could affect teenagers' privacy. The app targets minors, so privacy is a design constraint, not a feature. Complements docs/legal.md, which is binding.
---

# Minors and privacy

KotoMotion is aimed at teenagers and reads what they type. Privacy is a design constraint.
`docs/legal.md` is the binding register; this skill is the engineering checklist that serves it.
This is not legal advice: open gates (L3 consent, L4 labels, L5 Full Access, L6 store rules) are
closed with a qualified lawyer before public launch.

## Rules the code must respect
1. **Message text never leaves the device** and is never written to disk beyond what the user
   explicitly saves. No server-side processing (ADR-0007, L1).
2. **Never log message text**: not in debug logs, not in error messages, not in crash reports, not
   in analytics events. Log lengths, durations, error kinds, never content.
3. **No advertising SDK, no third-party tracker, no fingerprinting** (L2). Any new dependency is
   checked for hidden network calls and data collection.
4. **Sensitive fields are skipped:** password, payment and similar fields are never analysed.
5. **Data minimisation:** collect nothing that the core loop does not need. No account is required
   to use it. If a feature needs data, write down what, why, how long, and where, before building.
6. **Shared output is safe:** an exported animation or magic link contains only the message the user
   chose to share; links carry no extra identifiers; expiry and deletion are defined.
7. **Secrets** (signing keys, tokens) never in git; separate keys per environment.

## Design rules for a product used by minors
- Rewards must be about the message, not about pressure: avoid streak-loss mechanics, countdowns,
  social comparison and notifications designed to bring children back. The learning angle is
  intrinsic: the animation grows when the writing is right.
- No public profiles, no contact with strangers, no leaderboards of children.
- Corrections are kind and short; never shame a mistake.
- Content shown (themes, onomatopoeias) is age-appropriate; test copy with teenagers (M5 beta).

## Legal anchors to keep in mind (verify with the lawyer)
- GDPR: in France, a child under 15 needs parental authorisation for information-society services
  (L3). The age-gate and consent flow are designed before the first external beta.
- If a data protection impact assessment is required for children's data, the lawyer decides;
  prepare the description of processing (what is processed, where, retention: none server-side).
- Stores: Apple's rules for kids apps and third-party keyboards, Google Play's Families and Data
  safety requirements. Privacy labels and the privacy manifest must match the code (L4, L6).
- Expanding to English markets brings other regimes for children (for example the US and UK);
  re-run this checklist per market.

## Review checklist for any PR
- [ ] Does it add a network call, permission, SDK, log line or stored value? If yes, justify.
- [ ] Can message text reach a log, a crash report, a file or another process?
- [ ] Are password and sensitive fields skipped?
- [ ] Do store listings and privacy labels still match the code?
- [ ] Is a gate in `docs/legal.md` touched? Update its evidence log.
- [ ] Is anything here a retention or consent question for the lawyer? Add it to the open list.
