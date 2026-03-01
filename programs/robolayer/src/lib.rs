use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("RBLYr7mRXT4oFqJzKw8PqWnLkGpMR5C3aY6hN9qFau2");

#[program]
pub mod robolayer {
    use super::*;

    /// Initialize the RoboLayer protocol state
    pub fn initialize(ctx: Context<Initialize>, config: ProtocolConfig) -> Result<()> {
        let state = &mut ctx.accounts.protocol_state;
        state.authority = ctx.accounts.authority.key();
        state.operator_count = 0;
        state.task_count = 0;
        state.total_staked = 0;
        state.min_stake = config.min_stake;
        state.slash_rate_bps = config.slash_rate_bps;
        state.reward_rate_bps = config.reward_rate_bps;
        state.bump = ctx.bumps.protocol_state;

        emit!(ProtocolInitialized {
            authority: state.authority,
            min_stake: state.min_stake,
        });

        Ok(())
    }

    /// Register a new operator with initial stake
    pub fn register_operator(
        ctx: Context<RegisterOperator>,
        name: String,
        capabilities: Vec<u8>,
    ) -> Result<()> {
        require!(name.len() <= 32, RoboLayerError::NameTooLong);
        require!(capabilities.len() <= 16, RoboLayerError::TooManyCapabilities);

        let operator = &mut ctx.accounts.operator;
        operator.authority = ctx.accounts.authority.key();
        operator.name = name.clone();
        operator.capabilities = capabilities;
        operator.stake = 0;
        operator.reputation = 0;
        operator.tasks_completed = 0;
        operator.registered_at = Clock::get()?.unix_timestamp;
        operator.is_active = true;
        operator.bump = ctx.bumps.operator;

        let state = &mut ctx.accounts.protocol_state;
        state.operator_count = state.operator_count.checked_add(1).unwrap();

        emit!(OperatorRegistered {
            operator: operator.key(),
            authority: operator.authority,
            name,
        });

        Ok(())
    }

    /// Stake tokens to increase operator weight
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        require!(amount > 0, RoboLayerError::InvalidAmount);

        let state = &ctx.accounts.protocol_state;
        let operator = &mut ctx.accounts.operator;

        require!(
            operator.stake.checked_add(amount).unwrap() >= state.min_stake,
            RoboLayerError::BelowMinStake
        );

        // Transfer tokens to vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        operator.stake = operator.stake.checked_add(amount).unwrap();

        let state = &mut ctx.accounts.protocol_state;
        state.total_staked = state.total_staked.checked_add(amount).unwrap();

        emit!(Staked {
            operator: operator.key(),
            amount,
            total_stake: operator.stake,
        });

        Ok(())
    }

    /// Submit a new task for execution
    pub fn submit_task(
        ctx: Context<SubmitTask>,
        task_type: u8,
        payload_hash: [u8; 32],
        reward: u64,
        timeout: i64,
    ) -> Result<()> {
        require!(reward > 0, RoboLayerError::InvalidAmount);
        require!(timeout > 0, RoboLayerError::InvalidTimeout);

        let task = &mut ctx.accounts.task;
        task.submitter = ctx.accounts.submitter.key();
        task.task_type = task_type;
        task.payload_hash = payload_hash;
        task.reward = reward;
        task.timeout = timeout;
        task.status = TaskStatus::Pending;
        task.assigned_operator = Pubkey::default();
        task.created_at = Clock::get()?.unix_timestamp;
        task.completed_at = 0;
        task.bump = ctx.bumps.task;

        let state = &mut ctx.accounts.protocol_state;
        state.task_count = state.task_count.checked_add(1).unwrap();

        emit!(TaskSubmitted {
            task: task.key(),
            submitter: task.submitter,
            task_type,
            reward,
        });

        Ok(())
    }

    /// Complete a task and claim reward
    pub fn complete_task(
        ctx: Context<CompleteTask>,
        result_hash: [u8; 32],
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;
        let operator = &mut ctx.accounts.operator;

        require!(task.status == TaskStatus::Assigned, RoboLayerError::InvalidTaskStatus);
        require!(
            task.assigned_operator == operator.key(),
            RoboLayerError::NotAssignedOperator
        );

        let now = Clock::get()?.unix_timestamp;
        require!(
            now <= task.created_at + task.timeout,
            RoboLayerError::TaskTimeout
        );

        task.status = TaskStatus::Completed;
        task.completed_at = now;
        task.result_hash = result_hash;

        operator.tasks_completed = operator.tasks_completed.checked_add(1).unwrap();
        operator.reputation = operator.reputation.checked_add(10).unwrap();

        emit!(TaskCompleted {
            task: task.key(),
            operator: operator.key(),
            result_hash,
        });

        Ok(())
    }
}

// ============================================================================
// Accounts
// ============================================================================

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProtocolState::INIT_SPACE,
        seeds = [b"protocol_state"],
        bump
    )]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterOperator<'info> {
    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump
    )]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(
        init,
        payer = authority,
        space = 8 + Operator::INIT_SPACE,
        seeds = [b"operator", authority.key().as_ref()],
        bump
    )]
    pub operator: Account<'info, Operator>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump
    )]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(
        mut,
        seeds = [b"operator", authority.key().as_ref()],
        bump = operator.bump,
        has_one = authority
    )]
    pub operator: Account<'info, Operator>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SubmitTask<'info> {
    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump
    )]
    pub protocol_state: Account<'info, ProtocolState>,
    #[account(
        init,
        payer = submitter,
        space = 8 + Task::INIT_SPACE,
        seeds = [b"task", protocol_state.task_count.to_le_bytes().as_ref()],
        bump
    )]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub submitter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(
        mut,
        has_one = assigned_operator @ RoboLayerError::NotAssignedOperator
    )]
    pub task: Account<'info, Task>,
    #[account(
        mut,
        constraint = operator.key() == task.assigned_operator
    )]
    pub operator: Account<'info, Operator>,
    pub assigned_operator: Signer<'info>,
}

// ============================================================================
// State
// ============================================================================

#[account]
#[derive(InitSpace)]
pub struct ProtocolState {
    pub authority: Pubkey,
    pub operator_count: u64,
    pub task_count: u64,
    pub total_staked: u64,
    pub min_stake: u64,
    pub slash_rate_bps: u16,
    pub reward_rate_bps: u16,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Operator {
    pub authority: Pubkey,
    #[max_len(32)]
    pub name: String,
    #[max_len(16)]
    pub capabilities: Vec<u8>,
    pub stake: u64,
    pub reputation: u64,
    pub tasks_completed: u64,
    pub registered_at: i64,
    pub is_active: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Task {
    pub submitter: Pubkey,
    pub task_type: u8,
    pub payload_hash: [u8; 32],
    pub result_hash: [u8; 32],
    pub reward: u64,
    pub timeout: i64,
    pub status: TaskStatus,
    pub assigned_operator: Pubkey,
    pub created_at: i64,
    pub completed_at: i64,
    pub bump: u8,
}

// ============================================================================
// Types & Enums
// ============================================================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum TaskStatus {
    Pending,
    Assigned,
    Completed,
    Failed,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProtocolConfig {
    pub min_stake: u64,
    pub slash_rate_bps: u16,
    pub reward_rate_bps: u16,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct ProtocolInitialized {
    pub authority: Pubkey,
    pub min_stake: u64,
}

#[event]
pub struct OperatorRegistered {
    pub operator: Pubkey,
    pub authority: Pubkey,
    pub name: String,
}

#[event]
pub struct Staked {
    pub operator: Pubkey,
    pub amount: u64,
    pub total_stake: u64,
}

#[event]
pub struct TaskSubmitted {
    pub task: Pubkey,
    pub submitter: Pubkey,
    pub task_type: u8,
    pub reward: u64,
}

#[event]
pub struct TaskCompleted {
    pub task: Pubkey,
    pub operator: Pubkey,
    pub result_hash: [u8; 32],
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum RoboLayerError {
    #[msg("Name exceeds maximum length of 32 characters")]
    NameTooLong,
    #[msg("Too many capabilities (max 16)")]
    TooManyCapabilities,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Stake below minimum required")]
    BelowMinStake,
    #[msg("Invalid timeout value")]
    InvalidTimeout,
    #[msg("Invalid task status for this operation")]
    InvalidTaskStatus,
    #[msg("Caller is not the assigned operator")]
    NotAssignedOperator,
    #[msg("Task has timed out")]
    TaskTimeout,
    #[msg("Operator is not active")]
    OperatorInactive,
    #[msg("Arithmetic overflow")]
    Overflow,
}
