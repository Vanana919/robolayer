//! `initialize` — bootstraps the protocol.
//!
//! Creates the singleton `ProtocolState` PDA and the program-owned
//! treasury token account. Must be called exactly once per deployment
//! by whatever wallet the team designates as `authority`.

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::events::ProtocolInitialized;
use crate::state::ProtocolState;
use crate::utils::{PROTOCOL_SEED, SLASH_BPS_DEFAULT, TREASURY_SEED};

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// Wallet paying for rent and becoming the protocol authority.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// Singleton protocol-state PDA.
    #[account(
        init,
        payer = authority,
        space = ProtocolState::SIZE,
        seeds = [PROTOCOL_SEED],
        bump,
    )]
    pub protocol: Account<'info, ProtocolState>,

    /// $ROBO mint. The protocol never mints — it only moves tokens
    /// in and out of the treasury — but we store the mint so all
    /// downstream instructions can assert `TokenAccount::mint ==
    /// protocol.robo_mint`.
    pub robo_mint: Account<'info, Mint>,

    /// CHECK: PDA used as the token authority. Validated via seeds.
    #[account(
        seeds = [TREASURY_SEED, robo_mint.key().as_ref(), b"authority"],
        bump,
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    /// Program-owned ATA-style account used as the treasury. Initialised
    /// with the treasury PDA as its authority so only this program can
    /// sign withdrawals.
    #[account(
        init,
        payer = authority,
        token::mint = robo_mint,
        token::authority = treasury_authority,
        seeds = [TREASURY_SEED, robo_mint.key().as_ref()],
        bump,
    )]
    pub treasury: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;
    protocol.authority = ctx.accounts.authority.key();
    protocol.robo_mint = ctx.accounts.robo_mint.key();
    protocol.treasury = ctx.accounts.treasury.key();
    protocol.bump = ctx.bumps.protocol;
    protocol.treasury_bump = ctx.bumps.treasury;
    protocol.paused = false;
    protocol.default_slash_bps = SLASH_BPS_DEFAULT;
    protocol.task_nonce = 0;
    protocol.total_operators = 0;
    protocol.total_active_tasks = 0;
    protocol.total_completed_tasks = 0;
    protocol.total_staked = 0;
    protocol.total_slashed = 0;
    protocol._reserved = [0u8; 64];

    let now = Clock::get()?.unix_timestamp;
    emit!(ProtocolInitialized {
        authority: protocol.authority,
        robo_mint: protocol.robo_mint,
        treasury: protocol.treasury,
        ts: now,
    });

    msg!(
        "robolayer initialized: authority={}, mint={}",
        protocol.authority,
        protocol.robo_mint
    );

    Ok(())
}
