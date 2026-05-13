import {
  JsonRpcProvider,
  Contract,
  Wallet,
  ZeroAddress,
  keccak256,
  toUtf8Bytes,
  parseEther,
  type ContractTransactionResponse,
  type Signer,
} from "ethers";

import RobodyneABI from "./abi/Robodyne.json";

/**
 * Canonical Robodyne contract addresses per chain.
 * Override via {@link RobodyneConfig.address} when targeting a custom deployment.
 */
export const ADDRESSES: Record<string, string> = {
  mainnet: "0x0000000000000000000000000000000000000000",
  sepolia: "0x0000000000000000000000000000000000000000",
  base: "0x0000000000000000000000000000000000000000",
} as const;

export enum VerificationMode {
  Optimistic = 0,
  ZKProof = 1,
  NofM = 2,
}

export enum TaskStatus {
  Pending = 0,
  Claimed = 1,
  Submitted = 2,
  Finalized = 3,
  Disputed = 4,
  Cancelled = 5,
}

export interface OperatorInfo {
  authority: string;
  name: string;
  stake: bigint;
  reputation: bigint;
  tasksCompleted: bigint;
  registeredAt: bigint;
  active: boolean;
  capabilities: string[];
}

export interface TaskInfo {
  submitter: string;
  assignedTo: string;
  payloadHash: string;
  resultHash: string;
  reward: bigint;
  timeoutAt: bigint;
  createdAt: bigint;
  mode: VerificationMode;
  status: TaskStatus;
}

export interface RobodyneConfig {
  /** chain key (mainnet / sepolia / base) — used to resolve the default address */
  network?: keyof typeof ADDRESSES;
  /** explicit contract address; overrides {@link network} */
  address?: string;
  /** JSON-RPC provider used for reads and to power the signer */
  provider: JsonRpcProvider;
  /** optional signer for write calls */
  signer?: Signer;
}

export interface RegisterOperatorParams {
  name: string;
  capabilities: string[];
  /** stake in wei */
  stake: bigint;
}

export interface SubmitTaskParams {
  payload: Uint8Array | string;
  /** reward in wei */
  reward: bigint;
  /** timeout in seconds */
  timeoutSec: number;
  mode?: VerificationMode;
}

/**
 * Robodyne SDK — minimal ethers-v6 client for the Robodyne execution layer.
 */
export class Robodyne {
  public readonly address: string;
  public readonly provider: JsonRpcProvider;
  public readonly signer?: Signer;
  public readonly contract: Contract;

  constructor(config: RobodyneConfig) {
    const addr = config.address ?? (config.network ? ADDRESSES[config.network] : undefined);
    if (!addr || addr === ZeroAddress) {
      throw new Error("Robodyne: no contract address provided (set `address` or `network`).");
    }
    this.address = addr;
    this.provider = config.provider;
    this.signer = config.signer;
    this.contract = new Contract(addr, RobodyneABI, config.signer ?? config.provider);
  }

  // ------------------------------------------------------------------
  // Writes
  // ------------------------------------------------------------------

  async registerOperator(p: RegisterOperatorParams): Promise<ContractTransactionResponse> {
    return this.contract.registerOperator(p.name, p.capabilities, { value: p.stake });
  }

  async withdrawStake(): Promise<ContractTransactionResponse> {
    return this.contract.withdrawStake();
  }

  async submitTask(p: SubmitTaskParams): Promise<ContractTransactionResponse> {
    const payloadHash =
      typeof p.payload === "string" ? keccak256(toUtf8Bytes(p.payload)) : keccak256(p.payload);
    return this.contract.submitTask(payloadHash, p.timeoutSec, p.mode ?? VerificationMode.Optimistic, {
      value: p.reward,
    });
  }

  async claimTask(taskId: string): Promise<ContractTransactionResponse> {
    return this.contract.claimTask(taskId);
  }

  async submitResult(
    taskId: string,
    resultHash: string,
    proof: Uint8Array | string = "0x",
  ): Promise<ContractTransactionResponse> {
    return this.contract.submitResult(taskId, resultHash, proof);
  }

  async finalizeTask(taskId: string): Promise<ContractTransactionResponse> {
    return this.contract.finalizeTask(taskId);
  }

  async slashOperator(
    operator: string,
    amount: bigint,
    reason: string,
  ): Promise<ContractTransactionResponse> {
    return this.contract.slashOperator(operator, amount, reason);
  }

  // ------------------------------------------------------------------
  // Views
  // ------------------------------------------------------------------

  async getOperator(authority: string): Promise<OperatorInfo> {
    const o = await this.contract.getOperator(authority);
    return {
      authority: o.authority,
      name: o.name,
      stake: o.stake,
      reputation: o.reputation,
      tasksCompleted: o.tasksCompleted,
      registeredAt: o.registeredAt,
      active: o.active,
      capabilities: [...o.capabilities],
    };
  }

  async getTask(taskId: string): Promise<TaskInfo> {
    const t = await this.contract.getTask(taskId);
    return {
      submitter: t.submitter,
      assignedTo: t.assignedTo,
      payloadHash: t.payloadHash,
      resultHash: t.resultHash,
      reward: t.reward,
      timeoutAt: t.timeoutAt,
      createdAt: t.createdAt,
      mode: Number(t.mode),
      status: Number(t.status),
    };
  }

  async getProtocolStats(): Promise<{
    operatorCount: bigint;
    taskCount: bigint;
    totalStaked: bigint;
  }> {
    const [operatorCount, taskCount, totalStaked] = await Promise.all([
      this.contract.operatorCount(),
      this.contract.taskCount(),
      this.contract.totalStaked(),
    ]);
    return { operatorCount, taskCount, totalStaked };
  }
}

export { RobodyneABI };
export default Robodyne;

// Re-export the common ethers helpers for ergonomics.
export { JsonRpcProvider, Wallet, parseEther, keccak256, toUtf8Bytes };
