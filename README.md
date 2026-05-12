<div align="center">

<img src="assets/banner.png" alt="RoboLayer — The Execution Engine for Robotics" width="100%"/>

# RoboLayer

**Execution Layer for Robotics & AI Operators on Solana**

[![Solana](https://img.shields.io/badge/Solana-9945FF?logo=solana&logoColor=white)](https://solana.com)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Anchor](https://img.shields.io/badge/Anchor-0.30.1-512BD4)](https://www.anchor-lang.com)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.3-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![CI](https://github.com/robolayertech/robolayer/actions/workflows/ci.yml/badge.svg)](https://github.com/robolayertech/robolayer/actions/workflows/ci.yml)
[![Stars](https://img.shields.io/github/stars/robolayertech/robolayer?style=social)](https://github.com/robolayertech/robolayer/stargazers)
[![Discord](https://img.shields.io/badge/discord-join-5865F2?logo=discord&logoColor=white)](#)
[![Twitter Follow](https://img.shields.io/badge/follow-%40robolayer-1DA1F2?logo=twitter&logoColor=white)](#)

[Website](https://robolayer.tech) · [Docs](docs/architecture.md) · [SDK](sdk/) · [Tokenomics](docs/tokenomics.md)

</div>

---

## $ROBO — Token

> **Live on Solana mainnet.** Fair launch on pump.fun. No presale, no team allocation, no advisor unlocks.

| | |
|---|---|
| **Contract** | `Fs4w6EgrxxSLQpbrmxVhaxhpw7WNw63MqU3rvvzMpump` |
| **Network** | Solana (SPL) |
| **Pump.fun** | [pump.fun/coin/Fs4w…pump](https://pump.fun/coin/Fs4w6EgrxxSLQpbrmxVhaxhpw7WNw63MqU3rvvzMpump) |
| **DexScreener** | [dexscreener.com/solana/Fs4w…pump](https://dexscreener.com/solana/Fs4w6EgrxxSLQpbrmxVhaxhpw7WNw63MqU3rvvzMpump) |
| **Birdeye** | [birdeye.so/token/Fs4w…pump](https://birdeye.so/token/Fs4w6EgrxxSLQpbrmxVhaxhpw7WNw63MqU3rvvzMpump?chain=solana) |
| **35M Locked** | [Streamflow vesting](https://app.streamflow.finance/contract/solana/mainnet/DMQR23ogp5sGbzHmsPQvrDvezPjyJG8ZkCEfbiAxcE4b) — team supply, on-chain proof |

**Why locked?** Execution layers need operators with skin in the game, not exit liquidity. Team supply vests on-chain via Streamflow — anyone can audit the schedule. See [`docs/tokenomics.md`](docs/tokenomics.md) for the full allocation breakdown.

---

## Overview

**RoboLayer** is a Solana-native execution layer that lets autonomous agents — robotics fleets, AI operators, on-chain bots — register, get assigned work, prove they did it, and earn rewards. All settled on-chain, all economically secured by staking.

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  AI / Robotics  │───▶│   RoboLayer     │───▶│     Solana      │
│    Operators    │    │  Execution Layer│    │   Settlement    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
        ▲                       │                       │
        │                       ▼                       ▼
        │              ┌─────────────────┐    ┌─────────────────┐
        └──────────────│  Task Assigner  │    │ Reward / Slash  │
                       └─────────────────┘    └─────────────────┘
```

## Why RoboLayer

- ⚡ **Sub-second task assignment** — leverages Solana's 400ms block times
- 🔒 **Economic security** — operators stake $ROBO, slashed on faults
- 🧪 **Verifiable execution** — optimistic dispute window + N-of-M consensus (ZK proofs on roadmap)
- 💸 **Fair reward distribution** — speed bonuses, reputation multipliers
- 🛠 **Developer-first SDK** — TypeScript client, full type safety

## Quickstart

### Install SDK
```bash
npm install @robolayer/sdk
# or
yarn add @robolayer/sdk
```

### Submit a task
```typescript
import { RoboLayer } from "@robolayer/sdk";
import { Connection, Keypair } from "@solana/web3.js";

const client = new RoboLayer({
  connection: new Connection("https://api.devnet.solana.com"),
  wallet: Keypair.fromSecretKey(/* ... */),
});

const taskId = await client.submitTask({
  capability: "image-classification",
  payload: Buffer.from("..."),
  rewardLamports: 1_000_000,
  timeoutSec: 30,
});

const result = await client.awaitResult(taskId);
console.log(result);
```

### Register as an operator
```typescript
await client.registerOperator({
  name: "fleet-001",
  capabilities: ["image-classification", "navigation"],
  stakeAmount: 10_000 * 1e9, // 10,000 $ROBO
});
```

## Architecture

RoboLayer is composed of four core on-chain components:

| Component | Purpose |
|-----------|---------|
| **Operator Registry** | Lifecycle, staking, reputation, slashing |
| **Task Engine** | Submission → Assignment → Execution → Verification → Settlement |
| **Verification Layer** | Optimistic dispute window + N-of-M consensus (ZK proofs scaffolded, WIP) |
| **Reward Distribution** | Base reward + speed bonus + reputation multiplier |

Full architecture: [docs/architecture.md](docs/architecture.md)

> **Verification status:** ZK proof verification is a **roadmap item** (Q3 2026). Code is scaffolded behind the `zk` cargo feature flag and currently no-ops. Use Optimistic or N-of-M verification for production.

### Program accounts

| Account | Seeds | Purpose |
|---------|-------|---------|
| `ProtocolState` | `["protocol_state"]` | Global config |
| `Operator` | `["operator", authority]` | Per-operator state |
| `Task` | `["task", task_count]` | Per-task lifecycle |
| `Vault` | `["vault"]` | Staking token vault |
| `RewardPool` | `["reward_pool"]` | Reward distribution |

## $ROBO Tokenomics

| | |
|---|---|
| **Total supply** | 1,000,000,000 $ROBO |
| **Decimals** | 9 |
| **Network** | Solana (SPL) |
| **Operator rewards** | 40% — halving emissions over 4y |
| **Ecosystem & grants** | 20% — 12mo cliff, 36mo linear |
| **Team & advisors** | 15% — 12mo cliff, 24mo linear |
| **Liquidity** | 10% — unlocked at TGE |
| **Treasury** | 10% — governance, 48h timelock |
| **Public sale** | 5% — unlocked at TGE |

Full tokenomics: [docs/tokenomics.md](docs/tokenomics.md)

## Build from source

```bash
# Prereqs: Rust 1.75+, Solana CLI 1.18+, Anchor 0.30.1, Node 20+

git clone https://github.com/robolayertech/robolayer.git
cd robolayer

# Build the on-chain program
anchor build

# Run tests
anchor test

# Deploy to devnet
yarn deploy:devnet
```

### Program IDs

| Network | Program ID |
|---------|-----------|
| Devnet | `RBLYdev2x9NkPuMtJhAcEsW6qFp4RyLmCbGoZ3iVrK8U` |
| Mainnet | `RBLYm4inxYZ2KvHfQ3qGwT8B7nKr5ePoVtL9aXjsBd1Y` |

> Mainnet program is deployed but unverified — auditing in progress. Use devnet for integration testing.

## Roadmap

- [x] Core program: registry, tasks, staking
- [x] TypeScript SDK
- [x] Devnet deployment
- [ ] ZK proof verifier integration
- [ ] Operator dashboard
- [ ] Mainnet launch
- [ ] Cross-chain task routing (Wormhole)
- [ ] Hardware attestation (TEE)

## Contributing

PRs welcome. For larger changes, please open an issue first to discuss.

```bash
# Fork → branch → PR
git checkout -b feat/my-thing
# ... make changes
yarn test
git commit -m "feat: my thing"
git push origin feat/my-thing
```

## Security

Found a vulnerability? Email **security@robolayer.xyz** with details. Please don't open a public issue.

## License

[MIT](LICENSE) — do whatever, just don't sue us.

---

<div align="center">
<sub>Built with 🤖 on Solana</sub>
</div>
