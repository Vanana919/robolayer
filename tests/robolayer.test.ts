import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { assert, expect } from "chai";

describe("robolayer", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Robolayer as Program;
  const authority = provider.wallet;

  let protocolStatePDA: PublicKey;
  let protocolStateBump: number;
  let operatorPDA: PublicKey;
  let operatorBump: number;

  before(async () => {
    [protocolStatePDA, protocolStateBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("protocol_state")],
      program.programId
    );

    [operatorPDA, operatorBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("operator"), authority.publicKey.toBuffer()],
      program.programId
    );
  });

  describe("initialize", () => {
    it("initializes the protocol state", async () => {
      const config = {
        minStake: new anchor.BN(1000_000_000_000), // 1000 ROBO
        slashRateBps: 500, // 5%
        rewardRateBps: 1000, // 10%
      };

      const tx = await program.methods
        .initialize(config)
        .accounts({
          protocolState: protocolStatePDA,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Initialize tx:", tx);

      const state = await program.account.protocolState.fetch(protocolStatePDA);
      assert.equal(state.authority.toBase58(), authority.publicKey.toBase58());
      assert.equal(state.operatorCount.toNumber(), 0);
      assert.equal(state.taskCount.toNumber(), 0);
      assert.equal(state.totalStaked.toNumber(), 0);
      assert.equal(state.minStake.toNumber(), 1000_000_000_000);
      assert.equal(state.slashRateBps, 500);
      assert.equal(state.rewardRateBps, 1000);
    });

    it("fails to initialize twice", async () => {
      const config = {
        minStake: new anchor.BN(1000_000_000_000),
        slashRateBps: 500,
        rewardRateBps: 1000,
      };

      try {
        await program.methods
          .initialize(config)
          .accounts({
            protocolState: protocolStatePDA,
            authority: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        assert.fail("Should have thrown");
      } catch (err) {
        // Expected: account already initialized
        expect(err).to.exist;
      }
    });
  });

  describe("register_operator", () => {
    it("registers a new operator", async () => {
      const tx = await program.methods
        .registerOperator("test-operator-1", Buffer.from([1, 2, 3]))
        .accounts({
          protocolState: protocolStatePDA,
          operator: operatorPDA,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Register operator tx:", tx);

      const operator = await program.account.operator.fetch(operatorPDA);
      assert.equal(operator.name, "test-operator-1");
      assert.equal(operator.authority.toBase58(), authority.publicKey.toBase58());
      assert.deepEqual(operator.capabilities, [1, 2, 3]);
      assert.equal(operator.stake.toNumber(), 0);
      assert.equal(operator.reputation.toNumber(), 0);
      assert.equal(operator.tasksCompleted.toNumber(), 0);
      assert.equal(operator.isActive, true);

      const state = await program.account.protocolState.fetch(protocolStatePDA);
      assert.equal(state.operatorCount.toNumber(), 1);
    });

    it("fails with name too long", async () => {
      const longName = "a".repeat(33);
      const newAuthority = Keypair.generate();

      // Airdrop to new authority
      const sig = await provider.connection.requestAirdrop(
        newAuthority.publicKey,
        2_000_000_000
      );
      await provider.connection.confirmTransaction(sig);

      const [newOperatorPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("operator"), newAuthority.publicKey.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .registerOperator(longName, Buffer.from([1]))
          .accounts({
            protocolState: protocolStatePDA,
            operator: newOperatorPDA,
            authority: newAuthority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([newAuthority])
          .rpc();
        assert.fail("Should have thrown");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("NameTooLong");
      }
    });
  });

  describe("submit_task", () => {
    it("submits a task successfully", async () => {
      const state = await program.account.protocolState.fetch(protocolStatePDA);
      const taskCount = state.taskCount;

      const [taskPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("task"), taskCount.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      const payloadHash = Buffer.alloc(32);
      payloadHash.fill(0xab);

      const tx = await program.methods
        .submitTask(
          1, // task_type: inference
          Array.from(payloadHash),
          new anchor.BN(50_000_000_000), // 50 ROBO reward
          new anchor.BN(30) // 30 second timeout
        )
        .accounts({
          protocolState: protocolStatePDA,
          task: taskPDA,
          submitter: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Submit task tx:", tx);

      const task = await program.account.task.fetch(taskPDA);
      assert.equal(task.submitter.toBase58(), authority.publicKey.toBase58());
      assert.equal(task.taskType, 1);
      assert.equal(task.reward.toNumber(), 50_000_000_000);
      assert.deepEqual(task.status, { pending: {} });
    });
  });

  describe("edge cases", () => {
    it("fails to submit task with zero reward", async () => {
      const state = await program.account.protocolState.fetch(protocolStatePDA);
      const taskCount = state.taskCount;

      const [taskPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("task"), taskCount.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      try {
        await program.methods
          .submitTask(1, Array.from(Buffer.alloc(32)), new anchor.BN(0), new anchor.BN(30))
          .accounts({
            protocolState: protocolStatePDA,
            task: taskPDA,
            submitter: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        assert.fail("Should have thrown");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("InvalidAmount");
      }
    });

    it("fails to submit task with zero timeout", async () => {
      const state = await program.account.protocolState.fetch(protocolStatePDA);
      const taskCount = state.taskCount;

      const [taskPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("task"), taskCount.toArrayLike(Buffer, "le", 8)],
        program.programId
      );

      try {
        await program.methods
          .submitTask(1, Array.from(Buffer.alloc(32)), new anchor.BN(100), new anchor.BN(0))
          .accounts({
            protocolState: protocolStatePDA,
            task: taskPDA,
            submitter: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();
        assert.fail("Should have thrown");
      } catch (err: any) {
        expect(err.error.errorCode.code).to.equal("InvalidTimeout");
      }
    });
  });
});
