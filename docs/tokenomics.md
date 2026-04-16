# $RDY Tokenomics

## Live

> **$RDY is live on Ethereum mainnet.** Fair launch via Uniswap v3, no presale, no team allocation, no advisor unlocks. 35M of team supply locked on-chain via Sablier streams.

| | |
|---|---|
| **Contract** | `0x0000000000000000000000000000000000000000` *(pinned at launch)* |
| **Uniswap** | [app.uniswap.org](https://app.uniswap.org) — RDY/ETH pool |
| **DexScreener** | [dexscreener.com/ethereum](https://dexscreener.com/ethereum) |
| **Etherscan** | [etherscan.io](https://etherscan.io) |
| **Vesting (35M)** | [Sablier stream](https://app.sablier.com) |

## Token Overview

| Parameter | Value |
|-----------|-------|
| **Name** | Robodyne |
| **Symbol** | $RDY |
| **Network** | Ethereum (ERC-20) |
| **Total Supply** | 1,000,000,000 |
| **Decimals** | 18 |

## Allocation

| Category | Tokens | Percentage | Vesting |
|----------|--------|-----------|---------|
| Operator Rewards | 400,000,000 | 40% | Emitted over 4 years (halving every 12 months) |
| Ecosystem & Grants | 200,000,000 | 20% | 12-month cliff, 36-month linear vesting |
| Team & Advisors | 150,000,000 | 15% | 12-month cliff, 24-month linear vesting |
| Liquidity Provision | 100,000,000 | 10% | Unlocked at TGE |
| Treasury | 100,000,000 | 10% | Governance-controlled, 48h timelock |
| Public Sale | 50,000,000 | 5% | Unlocked at TGE |

## Emission Schedule

Operator rewards follow a halving schedule:

| Year | Annual Emission | Daily Emission |
|------|----------------|----------------|
| Year 1 | 160,000,000 | ~438,356 |
| Year 2 | 120,000,000 | ~328,767 |
| Year 3 | 80,000,000 | ~219,178 |
| Year 4 | 40,000,000 | ~109,589 |

## Token Utility

1. **Staking**: Operators must stake $RDY to participate in task execution
2. **Task Rewards**: Operators earn $RDY for completing tasks
3. **Governance**: Token holders vote on protocol parameters
4. **Fee Payment**: Task submitters pay fees in $RDY (or ETH)
5. **Slashing Collateral**: Staked tokens are slashed for misbehavior

## Fee Structure

| Action | Fee |
|--------|-----|
| Task submission | 0.1% of reward value |
| Operator registration | 100 $RDY (burned) |
| Unstaking (early) | 1% penalty (redistributed to other stakers) |

## Deflationary Mechanisms

- Registration fees are burned
- 50% of protocol fees are burned
- Slashed tokens are partially burned (50% burned, 50% to treasury)

## Governance

Token holders can vote on:
- Minimum stake requirements
- Slash severity (basis points)
- Reward distribution parameters
- Protocol upgrades (with 48h timelock)
- Treasury spending proposals

Governance uses a Snapshot off-chain signal followed by on-chain execution via a Gnosis Safe + OpenZeppelin Timelock. A 3-day minimum lock applies to all parameter changes.
<!-- 2026-02-20 :: perf: short-circuit zero-result paths -->
<!-- 2026-03-05 :: fix: math overflow guard in SlashingLib -->
<!-- 2026-03-18 :: refactor: cleanup unused imports -->
<!-- 2026-03-29 :: fix(deploy): ensure broadcast flag is set -->
<!-- 2026-04-16 :: docs(api): add ethers v6 quickstart -->
