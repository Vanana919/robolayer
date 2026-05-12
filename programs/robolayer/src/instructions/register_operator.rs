//! `register_operator` — stake $ROBO and announce capabilities.
//!
//! Transfers `stake_amount` from the caller's ROBO ATA into the
//! protocol treasury and creates an `Operator` PDA keyed by their
//! wallet. The capability bitmask describes what kinds of tasks they
//! can serve (image classification, agent inference, robotics motion
//! planning, etc.). Bit assignments are documented in the public
//! roadmap appendix A.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::errors::RoboError;
use crate::events::OperatorRegistered;
use crate::state::{Operator, ProtocolState};
use crate::utils::{
    INITIAL_REPUTATION, MIN_OPERATOR_STAKE, OPERATOR_SEED, PROTOCOL_SEED, TREASURY_SEED,
};

#[derive(Accounts)]
pub struct RegisterOperator<'info> {
    /// Wallet registering as an operator. Pays rent for the PDA.
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
        constraint = !protocol.paused @ RoboError::ProtocolPaused,
    )]
    pub protocol: Account<'info, ProtocolState>,

    /// Operator PDA. `init` guarantees the same wallet cannot
    /// double-register.
    #[account(
        init,
        payer = authority,
        space = Operator::SIZE,
        seeds = [OPERATOR_SEED, authority.key().as_ref()],
        bump,
    )]
    pub operator: Account<'info, Operator>,

    /// Operator's ROBO token account. Must match the protocol mint.
    #[account(
        mut,
        constraint = operator_token_account.mint == protocol.robo_mint
            @ RoboError::CapabilityMismatch,
        constraint = operator_token_account.owner == authority.key()
            @ RoboError::Unauthorized,
    )]
    pub operator_token_account: Account<'info, TokenAccount>,

    /// Protocol treasury. Receives the stake.
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
    ctx: Context<RegisterOperator>,
    capabilities: u64,
    stake_amount: u64,
) -> Result<()> {
    require!(
        stake_amount >= MIN_OPERATOR_STAKE,
        RoboError::StakeTooLow
    );
    require!(capabilities != 0, RoboError::CapabilityMismatch);

    // CPI: move stake from operator's ATA into the treasury.
    let cpi_accounts = Transfer {
        from: ctx.accounts.operator_token_account.to_account_info(),
        to: ctx.accounts.treasury.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
    );
    token::transfer(cpi_ctx, stake_amount)?;

    let now = Clock::get()?.unix_timestamp;
    let op = &mut ctx.accounts.operator;
    op.authority = ctx.accounts.authority.key();
    op.stake = stake_amount;
    op.capabilities = capabilities;
    op.reputation = INITIAL_REPUTATION;
    op.slashed = false;
    op.registered_at = now;
    op.completed_tasks = 0;
    op.pending_withdrawal_amount = 0;
    op.pending_withdrawal_unlock_ts = 0;
    op.bump = ctx.bumps.operator;
    op._reserved = [0u8; 32];

    let protocol = &mut ctx.accounts.protocol;
    protocol.total_operators = protocol
        .total_operators
        .checked_add(1)
        .ok_or(RoboError::ArithmeticOverflow)?;
    protocol.total_staked = protocol
        .total_staked
        .checked_add(stake_amount)
        .ok_or(RoboError::ArithmeticOverflow)?;

    emit!(OperatorRegistered {
        operator: op.authority,
        stake: stake_amount,
        capabilities,
        ts: now,
    });

    Ok(())
}
