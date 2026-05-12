//! `finalize_task` — apply verification, pay or void.
//!
//! Anyone can call this once the verification deadline has passed.
//! The instruction runs `verification::verify`, then either:
//!
//! * pays the operator from the treasury and credits their reputation
//!   (`Finalized`), or
//! * voids the task and refunds the requester (`Voided`); if the
//!   verification outcome demands a slash the operator is flagged so
//!   the admin's `slash_operator` ix can collect the bond.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::errors::RoboError;
use crate::events::TaskFinalized;
use crate::state::{Operator, ProtocolState, Task, TaskStatus};
use crate::utils::{
    OPERATOR_SEED, PROTOCOL_SEED, REPUTATION_REWARD, TASK_SEED, TREASURY_SEED,
};
use crate::verification::{verify, VerifyInputs};

#[derive(Accounts)]
pub struct FinalizeTask<'info> {
    /// Anyone may crank finalization once the window closes.
    pub cranker: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, ProtocolState>,

    #[account(
        mut,
        seeds = [OPERATOR_SEED, operator.authority.as_ref()],
        bump = operator.bump,
    )]
    pub operator: Account<'info, Operator>,

    #[account(
        mut,
        seeds = [TASK_SEED, &task.id.to_le_bytes()],
        bump = task.bump,
        constraint = task.operator_assigned == operator.authority
            @ RoboError::NotAssignedOperator,
    )]
    pub task: Account<'info, Task>,

    /// Where the reward goes when the task is accepted.
    #[account(
        mut,
        constraint = operator_token_account.owner == operator.authority
            @ RoboError::Unauthorized,
        constraint = operator_token_account.mint == protocol.robo_mint
            @ RoboError::CapabilityMismatch,
    )]
    pub operator_token_account: Account<'info, TokenAccount>,

    /// Where the reward goes when the task is voided.
    #[account(
        mut,
        constraint = requester_token_account.owner == task.requester
            @ RoboError::Unauthorized,
        constraint = requester_token_account.mint == protocol.robo_mint
            @ RoboError::CapabilityMismatch,
    )]
    pub requester_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = protocol.treasury,
        seeds = [TREASURY_SEED, robo_mint.key().as_ref()],
        bump = protocol.treasury_bump,
    )]
    pub treasury: Account<'info, TokenAccount>,

    /// CHECK: PDA authority used to sign the outbound transfer.
    #[account(
        seeds = [TREASURY_SEED, robo_mint.key().as_ref(), b"authority"],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    pub robo_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<FinalizeTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    task.assert_status(TaskStatus::AwaitingVerification)?;

    let now = Clock::get()?.unix_timestamp;

    let outcome = verify(VerifyInputs {
        mode: &task.verification,
        submitted_at: task.submitted_at,
        now,
        challenges: task.challenges,
        matching_votes: task.matching_votes,
    })?;

    let mint_key = ctx.accounts.robo_mint.key();
    let auth_bump = ctx.bumps.treasury_authority;
    let signer_seeds: &[&[&[u8]]] = &[&[
        TREASURY_SEED,
        mint_key.as_ref(),
        b"authority",
        std::slice::from_ref(&auth_bump),
    ]];

    let protocol = &mut ctx.accounts.protocol;
    let operator = &mut ctx.accounts.operator;

    if outcome.accepted {
        // Pay the operator.
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury.to_account_info(),
                to: ctx.accounts.operator_token_account.to_account_info(),
                authority: ctx.accounts.treasury_authority.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(cpi_ctx, task.reward)?;

        operator.completed_tasks = operator
            .completed_tasks
            .checked_add(1)
            .ok_or(RoboError::ArithmeticOverflow)?;
        operator.reputation = operator.reputation.saturating_add(REPUTATION_REWARD);

        task.status = TaskStatus::Finalized;
        protocol.total_completed_tasks = protocol
            .total_completed_tasks
            .checked_add(1)
            .ok_or(RoboError::ArithmeticOverflow)?;
    } else {
        // Refund the requester.
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.treasury.to_account_info(),
                to: ctx.accounts.requester_token_account.to_account_info(),
                authority: ctx.accounts.treasury_authority.to_account_info(),
            },
            signer_seeds,
        );
        token::transfer(cpi_ctx, task.reward)?;

        task.status = TaskStatus::Voided;

        if outcome.slash {
            // Mark the operator slashed; the actual stake confiscation
            // happens in `slash_operator` so admins can apply policy
            // (which ratio, where the slashed tokens flow).
            operator.slashed = true;
        }
    }

    protocol.total_active_tasks = protocol
        .total_active_tasks
        .checked_sub(1)
        .ok_or(RoboError::ArithmeticOverflow)?;

    let reward_paid = if outcome.accepted { task.reward } else { 0 };
    emit!(TaskFinalized {
        task: task.key(),
        operator: operator.authority,
        reward_paid,
        ts: now,
    });

    // Zero out the task reward so accounting reflects drained escrow.
    task.reward = 0;

    Ok(())
}
