# Robodyne Architecture

## Overview

Robodyne is a modular execution layer that sits between off-chain robotics / AI infrastructure and Ethereum (mainnet + L2s such as Base). The architecture prioritizes verifiable execution, economic security through staking, and a clean SDK surface for operators and submitters.

## System Components

### 1. Operator Registry

The Operator Registry is the core on-chain contract (`Robodyne.sol`) that manages the lifecycle of operators:

- **Registration**: Operators register with a name, capability list, and initial ETH stake (will migrate to `$RDY` after TGE).
- **Staking**: Operators must maintain stake above `minStake` to remain eligible for task assignment.
- **Reputation**: On-chain reputation score based on task completion rate.
- **Slashing**: Misbehaving operators lose stake proportional to severity (basis points). 50% burned, 50% to the treasury.

### 2. Task Execution Engine

The Task Execution Engine handles the full lifecycle of tasks:

```
Submission -> Claim -> Execution -> Verification -> Finalization
```

**Task assignment:**
- Operators self-claim tasks they're eligible for (capability + stake check).
- A future version will introduce on-chain weighted selection based on stake × reputation.
- Submitters can require N-of-M consensus for high-value workloads.

### 3. Verification Layer

Tasks are verified through one of the following modes:
- **Optimistic verification** *(shipped)*: Results accepted unless challenged within a configurable dispute window.
- **N-of-M consensus** *(shipped)*: Critical tasks require N-of-M operator agreement.
- **ZK proofs** *(roadmap, Q3 2026)*: The `submitResult` call accepts a `proof` bytes argument; the verifier currently no-ops and must not be relied on in production. Tracking issue in the roadmap.

### 4. Reward Distribution

Rewards are escrowed in the contract at task submission time:
- Base reward per completion
- Speed bonus when finalized well under the timeout
- Reputation multiplier for high-reputation operators (see `SlashingLib.rewardFor`)

## Data Flow

```
[External Client] --> [Robodyne SDK (ethers v6)] --> [Ethereum RPC]
                                                          |
                                              [Robodyne contract]
                                                          |
                              +---------------+-----------+-----------+
                              |               |                       |
                       [Operator Mgmt]  [Task Engine]          [Reward / Slash]
                              |               |                       |
                      [Stake / Withdraw] [Optimistic/N-of-M]   [ETH transfers]
```

## Security Model

1. **Economic security**: Operators risk slashing of staked ETH/$RDY.
2. **Cryptographic security**: ZK proofs for verifiable computation *(roadmap — see Verification Layer above)*.
3. **Social security**: Owner-controlled slashing (will move to a Gnosis Safe multisig + timelock).
4. **Redundancy**: Critical tasks can be assigned to multiple operators with N-of-M agreement.

## Contract Surface

| Function | Mutability | Description |
|---|---|---|
| `registerOperator(name, capabilities)` | payable | Stake + join the registry |
| `withdrawStake()` | nonpayable | Exit and reclaim stake |
| `submitTask(payloadHash, timeoutSec, mode)` | payable | Submit work with escrowed reward |
| `claimTask(taskId)` | nonpayable | Operator self-assigns |
| `submitResult(taskId, resultHash, proof)` | nonpayable | Post result hash (+ optional proof) |
| `finalizeTask(taskId)` | nonpayable | Settle after the challenge window |
| `slashOperator(op, amount, reason)` | onlyOwner | Burn part of stake on fault |
| `getOperator(addr)` / `getTask(id)` | view | Read accessors |
| `operatorCount` / `taskCount` / `totalStaked` | view | Aggregate counters |

## Upgrade Path

The current `Robodyne.sol` is non-upgradable for auditability. The v0.3 mainnet contract will sit behind an OpenZeppelin transparent proxy with a 48h timelock on the admin. The proxy address will be pinned at TGE and announced in `docs/tokenomics.md`.
