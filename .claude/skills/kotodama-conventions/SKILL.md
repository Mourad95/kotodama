---
name: kotodama-conventions
description: Load when naming anything in the Kotodama repository (crates, types, functions, files, branches, data keys, localisation keys, commits), when writing or reviewing code, docs or commit messages, or when unsure whether text should be English. Everything is English; one fixed vocabulary; French only in user-facing strings and in the historical emotion ids.
---

# Kotodama conventions

The full rules are in `docs/conventions.md`. Read it when naming something new. The essentials:

## Language
- Everything is **English**: identifiers, comments, docs, ADRs, commits, branches, issues, log and
  error messages, file names, localisation keys.
- Exceptions: user-facing strings in the localisation catalogs (French first), the French emotion
  ids in data files (`joie`, `colere`, `energie`, `suspense`, `tristesse`, `amour`, `calme`), and
  archived research reports.

## One vocabulary
`emotion`, `intensity`, `theme`, `level`, `issue`, `checker`, `embedder`, `head`, `bundle`, `shell`.
Never mood/feeling, tier, skin, corrector, encoder, pack, frontend. A spelling or grammar finding is
an **issue**; "error" means a Rust `Error` type.

## Shapes
- Rust: `snake_case` / `PascalCase` / `SCREAMING_SNAKE_CASE`; crates `kd-*`; errors `<Area>Error`;
  no `I` prefix on traits; units in names (`duration_ms`).
- JSON keys and enum values: `snake_case`. Theme ids: `kebab-case`.
- Branches `<type>/<scope>-<description>`; commits Conventional, imperative, ≤ 72 chars.
- Never rename generated bindings in a shell; rename in Rust.

## Review checklist
- [ ] Any French word in code or docs outside the allowed exceptions?
- [ ] A synonym of the fixed vocabulary?
- [ ] Unit missing from a duration, size or count?
- [ ] Commit and branch names conventional?
