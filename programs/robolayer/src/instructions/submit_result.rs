//! `submit_result` — operator publishes the hash of their result.
//!
//! Only the commitment (32-byte hash) lands on-chain. The actual
//! payload is exchanged out-of-band (IPFS / Arweave / requester
//! webhook). This keeps the program cheap while still letting
//! challengers prove fraud by revealing the preimage.

use anchor_lang::prelude::*;

use crate::errors::RoboError;
use crate::events::TaskResultSubmitted;
use crate::state::{Operator, ProtocolState, Task, TaskStatus};
use crate::utils::{OPERATOR_SEED, PROTOCOL_SEED, TASK_SEED};
use crate::verification::VerificationMode;

#[derive(Accounts)]
pub struct SubmitResult<'info> {
    pub authority: Signer<'info>,

    #[account(
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
        constraint = !protocol.paused @ RoboError::ProtocolPaused,
    )]
    pub protocol: Account<'info, ProtocolState>,

    #[account(
        seeds = [OPERATOR_SEED, authority.key().as_ref()],
        bump = operator.bump,
        has_one = authority @ RoboError::Unauthorized,
        constraint = !operator.slashed @ RoboError::OperatorSlashed,
    )]
    pub operator: Account<'info, Operator>,

    #[account(
        mut,
        seeds = [TASK_SEED, &task.id.to_le_bytes()],
        bump = task.bump,
        constraint = task.operator_assigned == authority.key()
            @ RoboError::NotAssignedOperator,
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<SubmitResult>, result_hash: [u8; 32]) -> Result<()> {
    require!(
        result_hash != [0u8; 32],
        RoboError::InvalidResultHash
    );

    let task = &mut ctx.accounts.task;
    task.assert_status(TaskStatus::Claimed)?;

    let now = Clock::get()?.unix_timestamp;
    require!(now <= task.deadline, RoboError::TaskExpired);

    task.result_hash = result_hash;
    task.submitted_at = now;
    task.status = TaskStatus::AwaitingVerification;
    // Pre-seed NofM matching votes with the operator's own vote so
    // `n` of `m` math is consistent across verification modes.
    if matches!(task.verification, VerificationMode::NofM { .. }) {
        task.matching_votes = 1;
    }

    let challenge_deadline = match task.verification {
        VerificationMode::Optimistic { challenge_window } => now
            .checked_add(challenge_window)
            .ok_or(RoboError::ArithmeticOverflow)?,
        // For non-optimistic modes the "deadline" is the original
        // task deadline; we emit it for indexer convenience.
        _ => task.deadline,
    };

    emit!(TaskResultSubmitted {
        task: task.key(),
        operator: ctx.accounts.authority.key(),
        result_hash,
        challenge_deadline,
        ts: now,
    });

    Ok(())
}
