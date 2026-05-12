//! `submit_task` — requester escrows reward and creates a task.
//!
//! Task IDs come from `ProtocolState::task_nonce` so the PDA is
//! deterministic for any (mint, nonce) pair. The reward is moved from
//! the requester's ROBO ATA into the protocol treasury; we keep the
//! escrow virtually accounted-for via `total_active_tasks` and the
//! task's own `reward` field rather than spinning up a dedicated
//! escrow account per task (cheaper rent for high-volume task books).

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::errors::RoboError;
use crate::events::TaskSubmitted;
use crate::state::{ProtocolState, Task, TaskStatus};
use crate::utils::{MAX_TASK_REWARD, PROTOCOL_SEED, TASK_SEED, TREASURY_SEED};
use crate::verification::VerificationMode;

#[derive(Accounts)]
#[instruction(reward: u64, deadline: i64, required_capabilities: u64, verification: VerificationMode)]
pub struct SubmitTask<'info> {
    /// Wallet creating and funding the task.
    #[account(mut)]
    pub requester: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
        constraint = !protocol.paused @ RoboError::ProtocolPaused,
    )]
    pub protocol: Account<'info, ProtocolState>,

    /// Task PDA derived from the current protocol nonce.
    #[account(
        init,
        payer = requester,
        space = Task::SIZE,
        seeds = [TASK_SEED, &protocol.task_nonce.to_le_bytes()],
        bump,
    )]
    pub task: Account<'info, Task>,

    /// Requester's ROBO ATA. Reward is debited from here.
    #[account(
        mut,
        constraint = requester_token_account.mint == protocol.robo_mint
            @ RoboError::CapabilityMismatch,
        constraint = requester_token_account.owner == requester.key()
            @ RoboError::Unauthorized,
    )]
    pub requester_token_account: Account<'info, TokenAccount>,

    /// Protocol treasury (shared with stake pool).
    #[account(
        mut,
        address = protocol.treasury,
        seeds = [TREASURY_SEED, robo_mint.key().as_ref()],
        bump = protocol.treasury_bump,
    )]
    pub treasury: Account<'info, TokenAccount>,

    pub robo_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SubmitTask>,
    reward: u64,
    deadline: i64,
    required_capabilities: u64,
    verification: VerificationMode,
) -> Result<()> {
    require!(reward > 0, RoboError::InvalidTaskReward);
    require!(reward <= MAX_TASK_REWARD, RoboError::InvalidTaskReward);
    require!(
        required_capabilities != 0,
        RoboError::CapabilityMismatch
    );

    let now = Clock::get()?.unix_timestamp;
    require!(deadline > now, RoboError::InvalidTaskDeadline);

    verification.validate()?;

    // Move reward into treasury escrow.
    let cpi_accounts = Transfer {
        from: ctx.accounts.requester_token_account.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.requester.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    token::transfer(cpi_ctx, reward)?;

    let protocol = &mut ctx.accounts.protocol;
    let task_id = protocol.task_nonce;
    protocol.task_nonce = protocol
        .task_nonce
        .checked_add(1)
        .ok_or(RoboError::ArithmeticOverflow)?;
    protocol.total_active_tasks = protocol
        .total_active_tasks
        .checked_add(1)
        .ok_or(RoboError::ArithmeticOverflow)?;

    let task = &mut ctx.accounts.task;
    task.id = task_id;
    task.requester = ctx.accounts.requester.key();
    task.reward = reward;
    task.deadline = deadline;
    task.required_capabilities = required_capabilities;
    task.status = TaskStatus::Open;
    task.operator_assigned = Pubkey::default();
    task.result_hash = [0u8; 32];
    task.verification = verification;
    task.submitted_at = 0;
    task.challenges = 0;
    task.matching_votes = 0;
    task.bump = ctx.bumps.task;
    task.created_at = now;
    task._reserved = [0u8; 32];

    emit!(TaskSubmitted {
        task: task.key(),
        requester: task.requester,
        reward,
        deadline,
        required_capabilities,
        ts: now,
    });

    Ok(())
}
