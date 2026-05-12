//! `claim_task` — operator picks up an Open task.
//!
//! We check three things:
//!   1. The operator is registered and not slashed.
//!   2. The task is `Open` and not past its deadline.
//!   3. The operator's capability bitmask covers the task's
//!      requirements.
//!
//! The first operator to land a transaction wins the task; everyone
//! else will see `InvalidTaskState` because the status will have
//! moved.

use anchor_lang::prelude::*;

use crate::errors::RoboError;
use crate::events::TaskClaimed;
use crate::state::{Operator, ProtocolState, Task, TaskStatus};
use crate::utils::{capability_match, OPERATOR_SEED, PROTOCOL_SEED, TASK_SEED};

#[derive(Accounts)]
pub struct ClaimTask<'info> {
    /// Operator wallet claiming the task.
    pub authority: Signer<'info>,

    #[account(
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
        constraint = !protocol.paused @ RoboError::ProtocolPaused,
    )]
    pub protocol: Account<'info, ProtocolState>,

    #[account(
        mut,
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
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<ClaimTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let operator = &ctx.accounts.operator;

    task.assert_status(TaskStatus::Open)?;

    let now = Clock::get()?.unix_timestamp;
    require!(now < task.deadline, RoboError::TaskExpired);

    require!(
        capability_match(operator.capabilities, task.required_capabilities),
        RoboError::CapabilityMismatch
    );

    task.status = TaskStatus::Claimed;
    task.operator_assigned = ctx.accounts.authority.key();

    emit!(TaskClaimed {
        task: task.key(),
        operator: task.operator_assigned,
        ts: now,
    });

    Ok(())
}
