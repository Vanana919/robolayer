# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] — Migrated to Ethereum
### Added
- New Solidity implementation in `contracts/Robodyne.sol` targeting `^0.8.24`
- `IRobodyne` interface in `contracts/interfaces/`
- `SlashingLib` pure-math library for slash and reward arithmetic
- Foundry test suite (`test/Robodyne.t.sol`) with fuzzing
- Foundry deploy script (`script/Deploy.s.sol`)
- ethers v6 SDK rewrite with `bigint` math, `JsonRpcProvider`, `Contract`
- `foundry.toml` profile + `remappings.txt`
- Sepolia / Mainnet / Base RPC endpoint configuration

### Changed
- Project renamed from RoboLayer to Robodyne
- Token ticker shortened to `$RDY` (was `$ROBO`)
- TypeScript SDK now consumes `@robodyne/sdk` and depends on `ethers ^6.10.0`
- Verification mode now lives on the task struct (`Optimistic`, `ZKProof`, `NofM`)
- Slashing payouts split 50/50 between burn sink and treasury

### Removed
- Anchor 0.30.1 program (`programs/robolayer/`) — replaced by `contracts/`
- `@coral-xyz/anchor`, `@solana/web3.js`, `@solana/spl-token` dependencies
- `Cargo.toml` workspace + `Anchor.toml` — replaced by `foundry.toml`
- Mocha/chai test setup — replaced by Foundry's `forge test`

## [0.1.0] - 2026-04-22
### Added
- Initial protocol scaffolding
- Protocol, Operator, and Task account structs
- TypeScript SDK skeleton
- Documentation and architecture overview
<!-- 2026-03-02 :: chore: bump openzeppelin patch -->
<!-- 2026-03-12 :: feat(deploy): support BASE rpc out of the box -->
