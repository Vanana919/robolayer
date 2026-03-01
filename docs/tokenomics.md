# $ROBO Tokenomics

## Token Overview

| Parameter | Value |
|-----------|-------|
| **Name** | RoboLayer |
| **Symbol** | $ROBO |
| **Network** | Solana (SPL Token) |
| **Total Supply** | 1,000,000,000 |
| **Decimals** | 9 |

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

1. **Staking**: Operators must stake $ROBO to participate in task execution
2. **Task Rewards**: Operators earn $ROBO for completing tasks
3. **Governance**: Token holders vote on protocol parameters
4. **Fee Payment**: Task submitters pay fees in $ROBO
5. **Slashing Collateral**: Staked tokens are slashed for misbehavior

## Fee Structure

| Action | Fee |
|--------|-----|
| Task submission | 0.1% of reward value |
| Operator registration | 100 $ROBO (burned) |
| Unstaking (early) | 1% penalty (redistributed to other stakers) |

## Deflationary Mechanisms

- Registration fees are burned
- 50% of protocol fees are burned
- Slashed tokens are partially burned (50% burned, 50% to reporters)

## Governance

Token holders can vote on:
- Minimum stake requirements
- Slash rates
- Reward distribution parameters
- Protocol upgrades (with timelock)
- Treasury spending proposals

Governance uses conviction voting with a 3-day minimum lock period.
