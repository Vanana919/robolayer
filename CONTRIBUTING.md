# Contributing to Robodyne

Thanks for your interest. We welcome contributions of all sizes — from fixing typos to adding entire modules.

## Quick Start

1. Fork the repo
2. Clone your fork
3. Create a feature branch: `git checkout -b feat/your-thing`
4. Make changes, commit
5. Push and open a PR against `main`

## Development Setup

### Requirements

- Foundry (forge / cast / anvil — install via `foundryup`)
- Solidity ^0.8.24 (managed by Foundry)
- Node 20+ (for the SDK)

### Build & test

```bash
# Install Solidity deps
forge install OpenZeppelin/openzeppelin-contracts foundry-rs/forge-std

# Contracts
forge build
forge test -vv
forge snapshot   # gas report

# SDK
cd sdk
npm install
npm run typecheck
npm test
```

## Code Style

- **Solidity**: `forge fmt` before committing. Custom errors over revert strings; natspec on every external function.
- **TypeScript**: Run `npm run lint` in `sdk/`. Prefer named exports. Target ethers v6 (`bigint`, not `BN`).
- **Commits**: Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `refactor:`, `test:`).

## Pull Requests

- Keep PRs focused — one logical change per PR.
- Include tests for new behavior. Foundry tests live in `test/`.
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
<!-- 2026-02-23 :: fix: revert message for zero stake -->
<!-- 2026-03-09 :: chore: bump TypeScript -->
<!-- 2026-03-18 :: docs(arch): redo verification mode table -->
<!-- 2026-04-15 :: ci: pin actions to commit SHAs -->
<!-- 2026-04-23 :: test: getTask returns expected fields -->
<!-- 2026-04-26 :: test: revert reason exact-match -->
<!-- 2026-05-02 :: perf: bytes32 lookups in mapping -->
