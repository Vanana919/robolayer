//! Per-operator state account.
//!
//! Derived from `[OPERATOR_SEED, authority]` so each wallet can have
//! at most one `Operator` registration.

use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Operator {
    /// Wallet that controls this operator account. Used both as the
    /// PDA seed and as the `has_one` target on every operator path.
    pub authority: Pubkey,

    /// $ROBO currently bonded to this operator. May be larger than
    /// `MIN_OPERATOR_STAKE` if the operator topped up.
    pub stake: u64,

    /// Bitmask of capabilities this operator advertises. Off-chain
    /// matchers and the `claim_task` ix both consult this. Bit
    /// assignments are protocol-defined (see roadmap appendix A).
    pub capabilities: u64,

    /// Soft reputation score used by off-chain matchers to prioritise
    /// reliable operators. Starts at `INITIAL_REPUTATION`.
    pub reputation: u32,

    /// `true` once any portion of the stake has been slashed. A
    /// slashed operator cannot claim new tasks but may still withdraw
    /// whatever remains after the unbonding period.
    pub slashed: bool,

    /// Timestamp at which the operator registered.
    pub registered_at: i64,

    /// Cumulative number of tasks the operator has been paid for.
    pub completed_tasks: u64,

    /// If non-zero, the unix timestamp at which the operator is
    /// allowed to finalize a pending withdrawal.
    pub pending_withdrawal_amount: u64,
    pub pending_withdrawal_unlock_ts: i64,

    /// Bump for this `Operator` PDA.
    pub bump: u8,

    /// Reserved for forward compatibility.
    pub _reserved: [u8; 32],
}

impl Operator {
    /// Discriminator (8) + body.
    pub const SIZE: usize =
        8 + 32 + 8 + 8 + 4 + 1 + 8 + 8 + 8 + 8 + 1 + 32;
}
