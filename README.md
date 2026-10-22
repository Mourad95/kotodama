# Kotodama

Messages that come alive. Kotodama detects the emotion of a message — joy, anger, energy,
suspense, sadness, love, calm — and animates it with the user's chosen theme. The better the
message is written, the wilder the animation: spelling and grammar become a superpower for
teenagers instead of a chore.

Everything runs on the phone. French first, then English.

- **Plan:** [`docs/roadmap.md`](docs/roadmap.md)
- **Architecture:** [`docs/architecture.md`](docs/architecture.md)
- **Decisions:** [`docs/adr/`](docs/adr/README.md)
- **Emotion engine benchmark:** [`research/emotion-benchmark/`](research/emotion-benchmark/)
- **Contributor and agent guide:** [`CLAUDE.md`](CLAUDE.md)

## Status

Milestone M0 (foundations). The Rust workspace builds and is tested; the emotion engine has been
selected by benchmark (EmbeddingGemma, pruned to French + English, 4-bit, 116 MB, 86.7 % accuracy
on 301 test messages).

## Quick start

```bash
cargo test --all
```

## License

Proprietary — all rights reserved. Third-party models and libraries keep their own licenses
(see ADR-0003 for the Gemma Terms of Use).
