# Agent guide — how to work on KotoMotion

For any AI agent (Claude Code or other) or new developer. Read in this order:

1. `docs/product.md` — what we are building and what we are not.
2. `CLAUDE.md` — binding repository rules.
3. `docs/architecture.md`, `docs/roadmap.md`, `docs/adr/` — how and in what order.
4. `docs/legal.md` — binding legal rules and gates.

Where a skill and a repository rule disagree, **the repository wins** (`CLAUDE.md`, ADRs,
`docs/legal.md`). Everything written in this repository is in English (ADR-0001), even though the
skills themselves are written in French.

## Personal skills

The owner maintains a skills repository: `Mourad95/claude-skills` (private, 24 skills).

Install on a machine:

```bash
git clone git@github.com:Mourad95/claude-skills.git ~/dev/claude-skills
cd ~/dev/claude-skills && ./install.sh     # symlinks each skill into ~/.claude/skills/
```

In a cloud session, attach the repository (read access) and install the same way. If the skills
are not available, apply the principles below from this file; do not stop working.

### Skills that apply here, and when

| Skill | Use it for, in this project |
|---|---|
| `brainstorming` | Any vague or large request ("add a theme system", "support Spanish", "make it faster"). Frame with questions, compare two approaches, get the design validated by sections, then write an ADR if it changes the architecture. Skip it for precise, bounded tasks. |
| `architecture` | Boundaries between crates (`kd-core` depends on nothing; shells only link `kd-ffi`), SOLID, keeping domain logic pure and I/O behind traits (`Embedder`, `GrammarChecker`). |
| `clean-code` | DRY/KISS/YAGNI. The FFI stays tiny; no abstraction before the third use; no code for hypothetical themes or languages. |
| `tdd` | Strict Red → Green → Refactor for pure logic: intensity rules, grammar rules, theme validation, confidence computation, parsing. A bug fix starts with a failing test that reproduces it. |
| `testing` | Choosing the level of test. Pure logic → unit tests; engine → parity tests against the Python reference; effects → golden-frame tests (from M3); mocks only at boundaries (the embedder, the OS). |
| `systematic-debugging` | Any unexplained behaviour: wrong emotion, parity mismatch, ONNX operator unsupported on a device, animation glitch. Reproduce, observe, hypothesise, fix the cause. |
| `verification-before-completion` | Before saying "done": show what was actually run and its output; say what could not be verified (for example anything that needs a real phone or a Hugging Face download). |
| `security` | Input validation on theme files and model bundles, secrets (never in git), dependency hygiene, least privilege on the keyboard ("Allow Full Access" is a legal gate L5), anything involving minors' data. |
| `git-workflow` | Conventional commits and branch names (`feat/<scope>-<desc>`), small PRs (< 400 lines), self-review, no secrets. |
| `python-ai` | `tools/model-export/` and `research/emotion-benchmark/` scripts: strict typing, `uv`/`pyproject`, reproducible runs, no hidden state. |
| `ai-llm` | Evaluation discipline and AI-safety thinking only. We use embeddings and a linear head, not RAG, agents or prompts. |
| `devops` | The GitHub Actions workflow in `.github/workflows/` only. Docker and AWS parts do not apply (on device only). |

### Skills that do not apply (stack mismatch)

`typescript-strict`, `frontend-react`, `frontend-nextjs`, `react-native`, `angular`,
`backend-nestjs`, `backend-node`, `database-mongo`, `database-postgres`, `redis`, `api-design`.
The shells are native (Swift, Kotlin) around a Rust core; there is no backend. `naming` is
TypeScript/Python-oriented: use its Git and Python parts, and Rust conventions for Rust.

### Language conventions that stand in for the missing skills

- **Naming and language:** everything is English, with a fixed vocabulary: see
  `docs/conventions.md` and the `kotomotion-conventions` skill.

### Project skills (in this repository)

The stack-specific skills missing from the personal repository live in `.claude/skills/` and are
loaded automatically by Claude Code when working here. They are versioned with the code they
describe; copy them to the personal skills repository if you want them in other projects.

| Skill | Use it for |
|---|---|
| `kotomotion-conventions` | Naming and language rules (`docs/conventions.md`): English everywhere, fixed vocabulary. |
| `rust-core` | Crate layering, traits at boundaries, errors, data-file parsing, tiny FFI, verification. |
| `on-device-ml` | Embedding model, pruning, Q4, ONNX Runtime on mobile, parity, budgets, benchmark hygiene. |
| `ios-swift-shell` | SwiftUI app, keyboard and iMessage extensions, XCFramework, memory limits, App Store. |
| `android-kotlin-shell` | Compose app, IME service, commit-content API, cargo-ndk, Play requirements. |
| `gpu-effects` | wgpu renderer, themes, golden frames, export, reduced motion and photosensitivity. |
| `minors-privacy` | Privacy engineering checklist for a product used by teenagers. |

Skills in this folder describe constraints that were researched but not yet proven on devices.
Statements about platform limits are marked "verify": check current documentation and measure on a
real phone before relying on them, then update the skill with what you learned.

## Working loop for any task

1. **Orient.** Read the relevant ADRs, roadmap milestone and its exit criteria, and the legal
   register. Check no open gate blocks the task.
2. **Frame** (if vague): `brainstorming`. State the reading you took.
3. **Test first** for pure logic: `tdd`.
4. **Implement** the smallest change. Behaviour goes in data when it can (rule 2).
5. **Verify** (`verification-before-completion`):

   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets -- -D warnings
   cargo test --all
   (cd research/emotion-benchmark && md5sum -c MD5SUMS)      # if research/ was touched
   ```

6. **Document:** update the roadmap status, add an ADR for any architecture-changing decision,
   record legal evidence in `docs/legal.md` when a gate closes.
7. **Commit** with a conventional message; open a small PR; self-review.

## Definition of done

- [ ] Formatting, lints and tests pass; the commands above were really run.
- [ ] New logic has tests; bug fixes have a regression test.
- [ ] No message text leaves the device; no network call in the analysis path (ADR-0007).
- [ ] No model weights, secrets or generated bundles in git.
- [ ] Legal rules respected; no open gate bypassed.
- [ ] Docs and ADRs updated; the change is in English.
- [ ] The report says what was verified and what was not.

## Hard "do not"

- Do not tune prototypes, thresholds or pruning on `test_v2.csv` (measurement only).
- Do not ship or link Grammalecte (GPL-3.0) or any model that is not under an OSI-approved licence.
- Do not reproduce or name an existing anime/manga work, character or text in a theme.
- Do not add a tracker, an ad SDK or any server-side processing of messages.
- Do not claim something works on a phone unless it was run on a phone.
