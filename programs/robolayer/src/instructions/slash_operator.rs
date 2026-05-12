//! `slash_operator` — admin / challenge-triggered stake confiscation.
//!
//! Two paths reach this instruction:
//!   1. The protocol authority calls it directly with a reason code
//!      (governance slashing).
//!   2. A successful challenge (or failed N-of-M quorum) flipped the
//!      operator's `slashed` flag in `finalize_task`; an off-chain
//!      cranker then calls this ix to apply the slash ratio.
//!
//! Slashed tokens stay in the treasury but are accounted for under
//! `ProtocolState::total_slashed` so governance can later route them
//! to an insurance fund or burn.

use anchor_lang::prelude::*;

use crate::errors::RoboError;
use crate::events::OperatorSlashed;
use crate::state::{Operator, ProtocolState};
use crate::utils::{
    apply_bps, decrement_reputation, OPERATOR_SEED, PROTOCOL_SEED, REPUTATION_PENALTY,
    SLASH_BPS_MAX,
};

#[derive(Accounts)]
pub struct SlashOperator<'info> {
    /// Caller. Must be the protocol authority for direct slashes;
    /// anyone may call when the operator was already flagged.
    pub authority: Signer<'info>,

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
}

pub fn handler(
    ctx: Context<SlashOperator>,
    slash_bps: u16,
    reason_code: u32,
) -> Result<()> {
    require!(slash_bps > 0, RoboError::SlashRatioTooHigh);
    require!(slash_bps <= SLASH_BPS_MAX, RoboError::SlashRatioTooHigh);

    let operator = &mut ctx.accounts.operator;
    let protocol = &mut ctx.accounts.protocol;

    let admin_call = ctx.accounts.authority.key() == protocol.authority;
    // Non-admins may only finish a slash that the runtime already
    // marked. Admins may slash anyone.
    require!(
        admin_call || operator.slashed,
        RoboError::Unauthorized
    );

    let slash_amount = apply_bps(operator.stake, slash_bps)?;
    require!(slash_amount > 0, RoboError::SlashRatioTooHigh);
    require!(
        slash_amount <= operator.stake,
        RoboError::WithdrawalTooLarge
    );

    operator.stake = operator
        .stake
        .checked_sub(slash_amount)
        .ok_or(RoboError::ArithmeticOverflow)?;
    operator.slashed = true;
    operator.reputation = decrement_reputation(operator.reputation, REPUTATION_PENALTY);

    protocol.total_slashed = protocol
        .total_slashed
        .checked_add(slash_amount)
        .ok_or(RoboError::ArithmeticOverflow)?;
    protocol.total_staked = protocol
        .total_staked
        .checked_sub(slash_amount)
        .ok_or(RoboError::ArithmeticOverflow)?;

    let now = Clock::get()?.unix_timestamp;
    emit!(OperatorSlashed {
        operator: operator.authority,
        amount: slash_amount,
        remaining_stake: operator.stake,
        reason_code,
        ts: now,
    });

    msg!(
        "slashed operator={} amount={} reason={}",
        operator.authority,
        slash_amount,
        reason_code
    );

    Ok(())
}
