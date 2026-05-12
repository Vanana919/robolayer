# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Optimistic verification mode with configurable challenge window
- N-of-M verification mode for consensus-based task validation
- Operator slashing logic with bps-configurable severity
- Treasury PDA escrow for task rewards
- Custom error enum covering arithmetic, auth, and state transitions

### Changed
- Separate devnet and mainnet program IDs
- README clarifies ZK roadmap status (WIP behind `zk` cargo feature)

### Fixed
- CI badge now points to the correct repository (post-rename)

## [0.1.0] - 2026-04-22
### Added
- Initial Anchor program scaffolding
- Protocol, Operator, and Task account structs
- TypeScript SDK skeleton
- Documentation and architecture overview
