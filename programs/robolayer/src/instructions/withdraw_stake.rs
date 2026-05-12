//! `withdraw_stake` — two-phase unbonding.
//!
//! Phase 1: operator records `pending_withdrawal_amount` and a
//!          `pending_withdrawal_unlock_ts = now + UNBONDING_PERIOD_SECS`.
//!          The stake stays in the treasury but is logically reserved.
//!
//! Phase 2: after the unlock timestamp, the operator calls the same
//!          ix with `finalize = true`. The treasury transfers tokens
//!          out using its PDA authority.
//!
//! Operators that have been slashed can still withdraw whatever bond
//! survived the slash, but they cannot claim new tasks.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::errors::RoboError;
use crate::events::{WithdrawalCompleted, WithdrawalRequested};
use crate::state::{Operator, ProtocolState};
use crate::utils::{OPERATOR_SEED, PROTOCOL_SEED, TREASURY_SEED, UNBONDING_PERIOD_SECS};

#[derive(Accounts)]
pub struct WithdrawStake<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, ProtocolState>,

    #[account(
        mut,
        seeds = [OPERATOR_SEED, authority.key().as_ref()],
        bump = operator.bump,
        has_one = authority @ RoboError::Unauthorized,
    )]
    pub operator: Account<'info, Operator>,

    #[account(
        mut,
        constraint = operator_token_account.owner == authority.key()
            @ RoboError::Unauthorized,
        constraint = operator_token_account.mint == protocol.robo_mint
            @ RoboError::CapabilityMismatch,
    )]
    pub operator_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = protocol.treasury,
        seeds = [TREASURY_SEED, robo_mint.key().as_ref()],
        bump = protocol.treasury_bump,
    )]
    pub treasury: Account<'info, TokenAccount>,

    /// CHECK: PDA used to sign the outbound transfer.
    #[account(
        seeds = [TREASURY_SEED, robo_mint.key().as_ref(), b"authority"],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    pub robo_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawStake>, amount: u64, finalize: bool) -> Result<()> {
    require!(amount > 0, RoboError::WithdrawalTooLarge);

    let now = Clock::get()?.unix_timestamp;
    let operator = &mut ctx.accounts.operator;
    let protocol = &mut ctx.accounts.protocol;

    if !finalize {
        // Phase 1 — initiate.
        require!(
            amount <= operator.stake,
            RoboError::WithdrawalTooLarge
        );
        require!(
            operator.pending_withdrawal_amount == 0,
            RoboError::WithdrawalTooLarge
        );

        operator.pending_withdrawal_amount = amount;
        operator.pending_withdrawal_unlock_ts = now
            .checked_add(UNBONDING_PERIOD_SECS)
            .ok_or(RoboError::ArithmeticOverflow)?;

        emit!(WithdrawalRequested {
            operator: operator.authority,
            amount,
            unlock_ts: operator.pending_withdrawal_unlock_ts,
            ts: now,
        });
        return Ok(());
    }

    // Phase 2 — finalize.
    require!(
        operator.pending_withdrawal_amount == amount,
        RoboError::WithdrawalTooLarge
    );
    require!(
        now >= operator.pending_withdrawal_unlock_ts,
        RoboError::UnbondingNotComplete
    );
    require!(
        amount <= operator.stake,
        RoboError::WithdrawalTooLarge
    );

    let mint_key = ctx.accounts.robo_mint.key();
    let auth_bump = ctx.bumps.treasury_authority;
    let signer_seeds: &[&[&[u8]]] = &[&[
        TREASURY_SEED,
        mint_key.as_ref(),
        b"authority",
        std::slice::from_ref(&auth_bump),
    ]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.treasury.to_account_info(),
            to: ctx.accounts.operator_token_account.to_account_info(),
            authority: ctx.accounts.treasury_authority.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(cpi_ctx, amount)?;

    operator.stake = operator
        .stake
        .checked_sub(amount)
        .ok_or(RoboError::ArithmeticOverflow)?;
    operator.pending_withdrawal_amount = 0;
    operator.pending_withdrawal_unlock_ts = 0;

    protocol.total_staked = protocol
        .total_staked
        .checked_sub(amount)
        .ok_or(RoboError::ArithmeticOverflow)?;

    emit!(WithdrawalCompleted {
        operator: operator.authority,
        amount,
        ts: now,
    });

    Ok(())
}
