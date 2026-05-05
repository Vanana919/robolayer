# Contributing to RoboLayer

Thanks for your interest. We welcome contributions of all sizes — from fixing typos to adding entire modules.

## Quick Start

1. Fork the repo
2. Clone your fork
3. Create a feature branch: `git checkout -b feat/your-thing`
4. Make changes, commit
5. Push and open a PR against `main`

## Development Setup

### Requirements

- Rust 1.75+
- Solana CLI 1.18+
- Anchor 0.30.1
- Node 20+

### Build & test

```bash
# On-chain program
anchor build
anchor test

# SDK
cd sdk
npm install
npm run typecheck
npm test
```

## Code Style

- **Rust**: `cargo fmt` before committing. We follow the default Anchor style.
- **TypeScript**: Run `npm run lint` in `sdk/`. Prefer named exports.
- **Commits**: Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`).

## Pull Requests

- Keep PRs focused — one logical change per PR.
- Include tests for new behavior.
- Update docs if you change public APIs.
- The CI must be green before review.
- Expect review feedback. PRs are typically merged within 3–5 business days.

## Reporting Issues

- Use the issue templates.
- Include version info, repro steps, expected vs actual.
- For security vulnerabilities, see [SECURITY.md](SECURITY.md) — **do not open public issues**.

## RFCs / Larger Changes

For changes that affect protocol design, tokenomics, or public APIs, open an issue with the `rfc` label first to discuss before implementing.

## License

By contributing you agree that your work will be licensed under the [MIT License](LICENSE).
