# Robodyne API Reference

## SDK Installation

```bash
npm install @robodyne/sdk ethers
# or
yarn add @robodyne/sdk ethers
```

## Initialization

```typescript
import { Robodyne } from "@robodyne/sdk";
import { JsonRpcProvider, Wallet } from "ethers";

const provider = new JsonRpcProvider(process.env.SEPOLIA_RPC_URL);
const signer = new Wallet(process.env.PRIVATE_KEY!, provider);

const robo = new Robodyne({
  network: "sepolia", // or "mainnet" / "base"
  provider,
  signer,
});
```

## Methods

### `registerOperator(params)`

Register a new operator on the network.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | Operator name (max 32 chars) |
| `stake` | `bigint` | Yes | Initial stake in wei |
| `capabilities` | `string[]` | Yes | List of capabilities |

**Available Capabilities (recommended labels):**
- `inference` — AI model inference
- `vision` — computer vision processing
- `manipulation` — robotic manipulation
- `navigation` — autonomous navigation
- `planning` — task planning & scheduling
- `speech` — speech recognition/synthesis

**Returns:** `Promise<ContractTransactionResponse>`

**Example:**
```typescript
import { parseEther } from "ethers";

const tx = await robo.registerOperator({
  name: "my-operator",
  stake: parseEther("1"),
  capabilities: ["inference", "vision"],
});
```

---

### `submitTask(params)`

Submit a new task for operator execution. The reward is escrowed as `msg.value`.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `payload` | `Uint8Array \| string` | Yes | Task payload; hashed before submission |
| `reward` | `bigint` | Yes | Reward in wei |
| `timeoutSec` | `number` | Yes | Timeout in seconds |
| `mode` | `VerificationMode` | No | Default: `Optimistic` |

**Returns:** `Promise<ContractTransactionResponse>`

**Example:**
```typescript
import { parseEther } from "ethers";

const tx = await robo.submitTask({
  payload: JSON.stringify({ model: "gpt-4", prompt: "..." }),
  reward: parseEther("0.05"),
  timeoutSec: 30,
});
```

---

### `claimTask(taskId)`

Claim an unassigned task. The caller must be an active operator with stake above the minimum.

**Returns:** `Promise<ContractTransactionResponse>`

---

### `submitResult(taskId, resultHash, proof?)`

Post the result hash and optional proof bytes for a claimed task.

**Returns:** `Promise<ContractTransactionResponse>`

---

### `finalizeTask(taskId)`

Anyone can call after the challenge window elapses. Pays out the operator.

**Returns:** `Promise<ContractTransactionResponse>`

---

### `slashOperator(operator, amount, reason)`

Owner-only. Slash an operator's stake. Pass `amount = 0n` to use the default severity (basis points configured at deploy).

**Returns:** `Promise<ContractTransactionResponse>`

---

### `withdrawStake()`

Operator exits and withdraws remaining stake.

**Returns:** `Promise<ContractTransactionResponse>`

---

### `getOperator(authority)`

Fetch operator information by address.

**Returns:** `Promise<OperatorInfo>`

---

### `getTask(taskId)`

Fetch task information by ID.

**Returns:** `Promise<TaskInfo>`

---

### `getProtocolStats()`

Get current protocol statistics.

**Returns:** `Promise<{ operatorCount: bigint; taskCount: bigint; totalStaked: bigint }>`

---

## Types

### `OperatorInfo`

```typescript
interface OperatorInfo {
  authority: string;
  name: string;
  stake: bigint;
  reputation: bigint;
  tasksCompleted: bigint;
  registeredAt: bigint;
  active: boolean;
  capabilities: string[];
}
```

### `TaskInfo`

```typescript
interface TaskInfo {
  submitter: string;
  assignedTo: string;
  payloadHash: string;     // 0x-prefixed bytes32
  resultHash: string;      // 0x-prefixed bytes32
  reward: bigint;          // wei
  timeoutAt: bigint;       // unix seconds
  createdAt: bigint;       // unix seconds
  mode: VerificationMode;
  status: TaskStatus;
}
```

### `VerificationMode`

```typescript
enum VerificationMode {
  Optimistic = 0,
  ZKProof = 1,
  NofM = 2,
}
```

### `TaskStatus`

```typescript
enum TaskStatus {
  Pending = 0,
  Claimed = 1,
  Submitted = 2,
  Finalized = 3,
  Disputed = 4,
  Cancelled = 5,
}
```

## Custom Errors

The contract uses custom errors instead of revert strings.

| Selector name | Meaning |
|---|---|
| `NameTooLong` | Operator name longer than 32 chars |
| `BelowMinStake` | Stake below `minStake` |
| `OperatorNotFound` | Address not registered |
| `OperatorInactive` | Operator has been deactivated |
| `NotAssignedOperator` | Caller is not the task's assignee |
| `InvalidTimeout` | Timeout was zero |
| `InvalidAmount` | Zero / negative amount, or too many capabilities |
| `InvalidStatus` | Task is in the wrong status for the call |
| `TaskExpired` | Past the task's `timeoutAt` |
| `AlreadyRegistered` | Operator address already registered |
| `NotAuthorized` | Caller lacks the required role |
<!-- 2026-02-17 :: docs: clarify task lifecycle in README -->
<!-- 2026-03-03 :: docs(arch): add data flow diagram -->
<!-- 2026-03-13 :: test: cooldown boundary case -->
