//! Per-task state account.
//!
//! A task moves through a small state machine:
//!
//! ```text
//!   Open ──claim──▶ Claimed ──submit_result──▶ AwaitingVerification
//!                                                       │
//!                                       finalize: accept │ reject
//!                                                       ▼
//!                                                    Finalized / Voided
//! ```
//!
//! Each transition is enforced by the corresponding instruction.

use anchor_lang::prelude::*;

use crate::verification::VerificationMode;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TaskStatus {
    /// Created and funded; awaiting an operator.
    Open,
    /// An operator claimed the task and is working on it.
    Claimed,
    /// Result hash has been submitted; in challenge / quorum window.
    AwaitingVerification,
    /// Reward paid out, escrow drained.
    Finalized,
    /// Task ended without a payout (timeout, slash, or admin cancel).
    Voided,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Open
    }
}

#[account]
#[derive(Default)]
pub struct Task {
    /// Stable id assigned at creation. Equal to the
    /// `ProtocolState::task_nonce` at the moment of creation, so off-
    /// chain indexers can deterministically derive the task PDA.
    pub id: u64,

    /// Wallet that funded the reward.
    pub requester: Pubkey,

    /// Reward (in ROBO base units) escrowed in the task's escrow PDA.
    /// Paid to the operator on accept, returned to the requester on
    /// void.
    pub reward: u64,

    /// Unix timestamp after which the task should no longer be
    /// claimable. Verification deadlines are tracked separately via
    /// `submitted_at + challenge_window`.
    pub deadline: i64,

    /// Bitmask of capabilities required to claim this task.
    pub required_capabilities: u64,

    /// Current state of the task.
    pub status: TaskStatus,

    /// Operator assigned by `claim_task`. `Pubkey::default()` until
    /// claimed.
    pub operator_assigned: Pubkey,

    /// Hash of the result payload submitted by the operator. Empty
    /// bytes until `submit_result`. The actual payload lives off-chain
    /// (IPFS / Arweave / a private blob); only its commitment is on
    /// chain.
    pub result_hash: [u8; 32],

    /// Verification strategy chosen at submission. Cannot change once
    /// the task is created.
    pub verification: VerificationMode,

    /// When the result was submitted. Combined with
    /// `verification.challenge_window` to compute the verification
    /// deadline.
    pub submitted_at: i64,

    /// Running tally of challenges accepted against this task.
    pub challenges: u32,

    /// For NofM tasks: how many operators have voted in agreement
    /// with `result_hash`.
    pub matching_votes: u8,

    /// Bump for this Task PDA.
    pub bump: u8,

    /// Created-at timestamp for analytics.
    pub created_at: i64,

    /// Reserved.
    pub _reserved: [u8; 32],
}

impl Task {
    /// Worst-case serialized size. We size `VerificationMode` for its
    /// largest variant (`NofM` is two u8s, `Optimistic` is i64, `Zk`
    /// is a Pubkey when enabled), plus a one-byte discriminant.
    pub const VERIFICATION_MAX_LEN: usize = 1 + 32;

    pub const SIZE: usize =
        // anchor discriminator
        8
        + 8                                  // id
        + 32                                 // requester
        + 8                                  // reward
        + 8                                  // deadline
        + 8                                  // required_capabilities
        + 1                                  // status (enum)
        + 32                                 // operator_assigned
        + 32                                 // result_hash
        + Self::VERIFICATION_MAX_LEN         // verification
        + 8                                  // submitted_at
        + 4                                  // challenges
        + 1                                  // matching_votes
        + 1                                  // bump
        + 8                                  // created_at
        + 32; // _reserved

    /// Helper guard used across instructions.
    pub fn assert_status(&self, expected: TaskStatus) -> Result<()> {
        require!(
            self.status == expected,
            crate::errors::RoboError::InvalidTaskState
        );
        Ok(())
    }
}
