# RoboLayer API Reference

## SDK Installation

```bash
npm install @robolayer/sdk
# or
yarn add @robolayer/sdk
```

## Initialization

```typescript
import { RoboLayer } from '@robolayer/sdk';
import { Keypair } from '@solana/web3.js';

const robo = new RoboLayer({
  cluster: 'mainnet-beta', // or 'devnet', 'localnet'
  wallet: walletAdapter,
  commitment: 'confirmed',
});
```

## Methods

### `registerOperator(params)`

Register a new operator on the network.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | `string` | Yes | Operator name (max 32 chars) |
| `stake` | `number` | No | Initial stake amount in $ROBO |
| `capabilities` | `string[]` | No | List of capabilities |

**Available Capabilities:**
- `inference` - AI model inference
- `vision` - Computer vision processing
- `manipulation` - Robotic manipulation
- `navigation` - Autonomous navigation
- `planning` - Task planning & scheduling
- `speech` - Speech recognition/synthesis

**Returns:** `Promise<TransactionSignature>`

**Example:**
```typescript
const tx = await robo.registerOperator({
  name: 'my-operator',
  stake: 10000,
  capabilities: ['inference', 'vision'],
});
```

---

### `stake(amount)`

Stake tokens to increase operator weight and eligibility.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `amount` | `number` | Yes | Amount of $ROBO to stake |

**Returns:** `Promise<TransactionSignature>`

---

### `submitTask(params)`

Submit a new task for operator execution.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `type` | `string` | Yes | Task type identifier |
| `payload` | `Buffer` | Yes | Encoded task payload |
| `reward` | `number` | Yes | Reward in $ROBO |
| `timeout` | `number` | Yes | Timeout in milliseconds |

**Returns:** `Promise<TransactionSignature>`

**Example:**
```typescript
const tx = await robo.submitTask({
  type: 'inference',
  payload: Buffer.from(JSON.stringify({ model: 'gpt-4', prompt: '...' })),
  reward: 50,
  timeout: 30000,
});
```

---

### `getOperator(authority)`

Fetch operator information by authority public key.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `authority` | `PublicKey` | Yes | Operator's authority key |

**Returns:** `Promise<OperatorInfo | null>`

---

### `getActiveOperators()`

Fetch all currently active operators.

**Returns:** `Promise<OperatorInfo[]>`

---

### `getTask(taskId)`

Fetch task information by ID.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| `taskId` | `BN` | Yes | Task ID (sequential counter) |

**Returns:** `Promise<TaskInfo | null>`

---

### `getProtocolStats()`

Get current protocol statistics.

**Returns:** `Promise<{ operatorCount: number, taskCount: number, totalStaked: BN }>`

---

## Types

### `OperatorInfo`

```typescript
interface OperatorInfo {
  authority: PublicKey;
  name: string;
  capabilities: number[];
  stake: BN;
  reputation: BN;
  tasksCompleted: BN;
  registeredAt: BN;
  isActive: boolean;
}
```

### `TaskInfo`

```typescript
interface TaskInfo {
  submitter: PublicKey;
  taskType: number;
  payloadHash: number[];
  resultHash: number[];
  reward: BN;
  timeout: BN;
  status: TaskStatus;
  assignedOperator: PublicKey;
  createdAt: BN;
  completedAt: BN;
}
```

### `TaskStatus`

```typescript
enum TaskStatus {
  Pending = 0,
  Assigned = 1,
  Completed = 2,
  Failed = 3,
  Cancelled = 4,
}
```

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| 6000 | NameTooLong | Name exceeds 32 characters |
| 6001 | TooManyCapabilities | More than 16 capabilities |
| 6002 | InvalidAmount | Zero or negative amount |
| 6003 | BelowMinStake | Below minimum stake requirement |
| 6004 | InvalidTimeout | Zero or negative timeout |
| 6005 | InvalidTaskStatus | Wrong status for operation |
| 6006 | NotAssignedOperator | Caller not assigned to task |
| 6007 | TaskTimeout | Task execution timed out |
| 6008 | OperatorInactive | Operator is deactivated |
| 6009 | Overflow | Arithmetic overflow |
