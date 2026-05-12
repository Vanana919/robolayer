//! # RoboLayer
//!
//! Solana execution layer for AI agents and robotics operators.
//! Operators stake $ROBO, claim tasks that match their capability
//! bitmask, submit result commitments, and either get paid out of
//! escrow or slashed if they misbehave.
//!
//! ## Modules
//!
//! * [`state`] – on-chain account definitions
//! * [`instructions`] – one file per ix handler
//! * [`verification`] – Optimistic / NofM / (feature-gated) Zk modes
//! * [`errors`] – `RoboError` variants returned to clients
//! * [`events`] – emitted for off-chain indexers
//! * [`utils`] – constants and small helpers
//!
//! ## Verification modes
//!
//! `Optimistic` and `NofM` are wired up end-to-end. `Zk` is feature-
//! flagged (`zk`) and gated behind `todo!()` until the Groth16
//! verifier program is live (see roadmap §3.4). Default builds never
//! reach the `todo!()`.

use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;
pub mod verification;

use instructions::*;
use verification::VerificationMode;

// -----------------------------------------------------------------------------
// Program IDs
//
// We ship distinct IDs for devnet and mainnet so the same binary, with
// the appropriate feature flag, deploys to either cluster without
// touching source. Both are valid 32-byte base58 program IDs prefixed
// with `RBLY` for grep-ability.
// -----------------------------------------------------------------------------

#[cfg(feature = "mainnet")]
declare_id!("RBLYm4inxYZ2KvHfQ3qGwT8B7nKr5ePoVtL9aXjsBd1Y");

#[cfg(not(feature = "mainnet"))]
declare_id!("RBLYdev2x9NkPuMtJhAcEsW6qFp4RyLmCbGoZ3iVrK8U");

#[program]
pub mod robolayer {
    use super::*;

    /// Bootstrap the singleton protocol state and treasury.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Stake $ROBO and register an operator with a capability mask.
    pub fn register_operator(
        ctx: Context<RegisterOperator>,
        capabilities: u64,
        stake_amount: u64,
    ) -> Result<()> {
        instructions::register_operator::handler(ctx, capabilities, stake_amount)
    }

    /// Create and fund a new task.
    pub fn submit_task(
        ctx: Context<SubmitTask>,
        reward: u64,
        deadline: i64,
        required_capabilities: u64,
        verification: VerificationMode,
    ) -> Result<()> {
        instructions::submit_task::handler(
            ctx,
            reward,
            deadline,
            required_capabilities,
            verification,
        )
    }

    /// Operator picks up an Open task that matches their capabilities.
    pub fn claim_task(ctx: Context<ClaimTask>) -> Result<()> {
        instructions::claim_task::handler(ctx)
    }

    /// Operator publishes the commitment to their result.
    pub fn submit_result(ctx: Context<SubmitResult>, result_hash: [u8; 32]) -> Result<()> {
        instructions::submit_result::handler(ctx, result_hash)
    }

    /// Crank verification after the challenge window closes.
    pub fn finalize_task(ctx: Context<FinalizeTask>) -> Result<()> {
        instructions::finalize_task::handler(ctx)
    }

    /// Apply a slash to an operator's bond.
    pub fn slash_operator(
        ctx: Context<SlashOperator>,
        slash_bps: u16,
        reason_code: u32,
    ) -> Result<()> {
        instructions::slash_operator::handler(ctx, slash_bps, reason_code)
    }

    /// Two-phase stake withdrawal (initiate or finalize via `finalize`).
    pub fn withdraw_stake(
        ctx: Context<WithdrawStake>,
        amount: u64,
        finalize: bool,
    ) -> Result<()> {
        instructions::withdraw_stake::handler(ctx, amount, finalize)
    }
}
