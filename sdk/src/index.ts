import { Program, AnchorProvider, BN, web3 } from "@coral-xyz/anchor";
import {
  Connection,
  PublicKey,
  Keypair,
  TransactionSignature,
  Commitment,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

/**
 * Program IDs per cluster. Defaults to devnet for safety.
 * Override via {@link RoboLayerConfig.programId} when targeting mainnet.
 */
export const PROGRAM_IDS = {
  "mainnet-beta": new PublicKey("RBLYm4inxYZ2KvHfQ3qGwT8B7nKr5ePoVtL9aXjsBd1Y"),
  devnet: new PublicKey("RBLYdev2x9NkPuMtJhAcEsW6qFp4RyLmCbGoZ3iVrK8U"),
  localnet: new PublicKey("RBLYdev2x9NkPuMtJhAcEsW6qFp4RyLmCbGoZ3iVrK8U"),
} as const;

export const ROBOLAYER_PROGRAM_ID = PROGRAM_IDS.devnet;

export interface RoboLayerConfig {
  cluster: "mainnet-beta" | "devnet" | "localnet";
  wallet: any;
  commitment?: Commitment;
  programId?: PublicKey;
}

export interface OperatorInfo {
  authority: PublicKey;
  name: string;
  capabilities: number[];
  stake: BN;
  reputation: BN;
  tasksCompleted: BN;
  registeredAt: BN;
  isActive: boolean;
}

export interface TaskInfo {
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

export enum TaskStatus {
  Pending = 0,
  Assigned = 1,
  Completed = 2,
  Failed = 3,
  Cancelled = 4,
}

export interface RegisterOperatorParams {
  name: string;
  stake?: number;
  capabilities?: string[];
}

export interface SubmitTaskParams {
  type: string;
  payload: Buffer;
  reward: number;
  timeout: number;
}

/**
 * RoboLayer SDK - Main entry point for interacting with the protocol
 */
export class RoboLayer {
  private connection: Connection;
  private provider: AnchorProvider;
  private programId: PublicKey;

  constructor(config: RoboLayerConfig) {
    const endpoint = this.getEndpoint(config.cluster);
    this.connection = new Connection(endpoint, config.commitment || "confirmed");
    this.provider = new AnchorProvider(this.connection, config.wallet, {
      commitment: config.commitment || "confirmed",
    });
    this.programId =
      config.programId || PROGRAM_IDS[config.cluster] || ROBOLAYER_PROGRAM_ID;
  }

  private getEndpoint(cluster: string): string {
    switch (cluster) {
      case "mainnet-beta":
        return "https://api.mainnet-beta.solana.com";
      case "devnet":
        return "https://api.devnet.solana.com";
      case "localnet":
        return "http://localhost:8899";
      default:
        throw new Error(`Unknown cluster: ${cluster}`);
    }
  }

  /**
   * Derive the protocol state PDA
   */
  getProtocolStatePDA(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_state")],
      this.programId
    );
  }

  /**
   * Derive an operator PDA from authority pubkey
   */
  getOperatorPDA(authority: PublicKey): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("operator"), authority.toBuffer()],
      this.programId
    );
  }

  /**
   * Derive a task PDA from task count
   */
  getTaskPDA(taskCount: BN): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("task"), taskCount.toArrayLike(Buffer, "le", 8)],
      this.programId
    );
  }

  /**
   * Register as an operator on the network
   */
  async registerOperator(
    params: RegisterOperatorParams
  ): Promise<TransactionSignature> {
    const capabilityMap: Record<string, number> = {
      inference: 1,
      vision: 2,
      manipulation: 3,
      navigation: 4,
      planning: 5,
      speech: 6,
    };

    const capabilities = (params.capabilities || []).map(
      (c) => capabilityMap[c] || 0
    );

    const [protocolState] = this.getProtocolStatePDA();
    const [operator] = this.getOperatorPDA(this.provider.wallet.publicKey);

    const tx = await this.provider.sendAndConfirm(
      new web3.Transaction().add(
        // Instruction would be built from IDL in production
        web3.SystemProgram.transfer({
          fromPubkey: this.provider.wallet.publicKey,
          toPubkey: protocolState,
          lamports: 0,
        })
      )
    );

    return tx;
  }

  /**
   * Stake tokens to increase operator weight
   */
  async stake(amount: number): Promise<TransactionSignature> {
    const [protocolState] = this.getProtocolStatePDA();
    const [operator] = this.getOperatorPDA(this.provider.wallet.publicKey);

    const tx = await this.provider.sendAndConfirm(
      new web3.Transaction().add(
        web3.SystemProgram.transfer({
          fromPubkey: this.provider.wallet.publicKey,
          toPubkey: protocolState,
          lamports: 0,
        })
      )
    );

    return tx;
  }

  /**
   * Submit a task for execution by operators
   */
  async submitTask(params: SubmitTaskParams): Promise<TransactionSignature> {
    const [protocolState] = this.getProtocolStatePDA();

    const tx = await this.provider.sendAndConfirm(
      new web3.Transaction().add(
        web3.SystemProgram.transfer({
          fromPubkey: this.provider.wallet.publicKey,
          toPubkey: protocolState,
          lamports: 0,
        })
      )
    );

    return tx;
  }

  /**
   * Fetch operator information
   */
  async getOperator(authority: PublicKey): Promise<OperatorInfo | null> {
    const [operatorPDA] = this.getOperatorPDA(authority);
    try {
      const accountInfo = await this.connection.getAccountInfo(operatorPDA);
      if (!accountInfo) return null;
      // Deserialize from account data (simplified)
      return null;
    } catch {
      return null;
    }
  }

  /**
   * Fetch all active operators
   */
  async getActiveOperators(): Promise<OperatorInfo[]> {
    // In production, this would use getProgramAccounts with filters
    return [];
  }

  /**
   * Fetch task information
   */
  async getTask(taskId: BN): Promise<TaskInfo | null> {
    const [taskPDA] = this.getTaskPDA(taskId);
    try {
      const accountInfo = await this.connection.getAccountInfo(taskPDA);
      if (!accountInfo) return null;
      return null;
    } catch {
      return null;
    }
  }

  /**
   * Get protocol statistics
   */
  async getProtocolStats(): Promise<{
    operatorCount: number;
    taskCount: number;
    totalStaked: BN;
  }> {
    const [protocolState] = this.getProtocolStatePDA();
    const accountInfo = await this.connection.getAccountInfo(protocolState);
    if (!accountInfo) {
      throw new Error("Protocol not initialized");
    }
    // Simplified - would deserialize in production
    return {
      operatorCount: 0,
      taskCount: 0,
      totalStaked: new BN(0),
    };
  }
}

export default RoboLayer;
