# Kotodama

Messages that come alive. Kotodama detects the emotion of a message — joy, anger, energy,
suspense, sadness, love, calm — and animates it with the user's chosen theme. The better the
message is written, the wilder the animation: spelling and grammar become a superpower for
teenagers instead of a chore.

Everything runs on the phone. French first, then English.

- **Product (start here):** [`docs/product.md`](docs/product.md)
- **Plan:** [`docs/roadmap.md`](docs/roadmap.md)
- **Architecture:** [`docs/architecture.md`](docs/architecture.md)
- **Decisions:** [`docs/adr/`](docs/adr/README.md)
- **Emotion engine benchmark:** [`research/emotion-benchmark/`](research/emotion-benchmark/)
- **Legal register:** [`docs/legal.md`](docs/legal.md)
- **Contributor and agent guide:** [`CLAUDE.md`](CLAUDE.md) and [`docs/agent-guide.md`](docs/agent-guide.md)

## Status

Milestone M0 (foundations). The Rust workspace builds and is tested. The emotion engine approach was
validated by benchmark (EmbeddingGemma 1, pruned to French + English, 4-bit, 116 MB, 86.7 % accuracy
on 301 test messages). Shipped models must be under an open licence (ADR-0010), so round 3 tests
the Apache-2.0 EmbeddingGemma 2 before the engine is built.

## Quick start

```bash
cargo test --all
```

## License

Proprietary — all rights reserved. Third-party models and libraries keep their own licenses
(see `docs/legal.md`).
