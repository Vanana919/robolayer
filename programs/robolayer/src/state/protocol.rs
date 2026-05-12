//! Global protocol-state account.
//!
//! Exactly one `ProtocolState` PDA exists per deployment, derived from
//! the `PROTOCOL_SEED`. It holds protocol-wide configuration and a
//! running set of counters useful for off-chain analytics.

use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ProtocolState {
    /// Multisig / governance authority allowed to call admin paths
    /// (pause, update config, slash by admin).
    pub authority: Pubkey,

    /// $ROBO SPL mint used for staking and task rewards.
    pub robo_mint: Pubkey,

    /// Token account that holds operator stakes and task escrows. PDA
    /// owned by the program; the `treasury_bump` is stored so signers
    /// can reconstruct the seed list without re-derivation.
    pub treasury: Pubkey,

    /// Bump for the `ProtocolState` PDA itself.
    pub bump: u8,

    /// Bump for the treasury PDA. Needed for token transfers out of
    /// the treasury under program authority.
    pub treasury_bump: u8,

    /// Set by `pause_protocol` (admin) to halt user-facing paths
    /// during incidents. All admin paths remain callable.
    pub paused: bool,

    /// Default basis-point ratio applied by `slash_operator` when no
    /// explicit ratio is provided.
    pub default_slash_bps: u16,

    /// Monotonically increasing counter used to derive unique task
    /// PDAs. We never decrement, even after a task is finalized, so
    /// the (requester, nonce) namespace is stable.
    pub task_nonce: u64,

    /// Aggregate counters. Useful for dashboards and circuit breakers
    /// (e.g. pause if `total_slashed > X` in a window).
    pub total_operators: u64,
    pub total_active_tasks: u64,
    pub total_completed_tasks: u64,
    pub total_staked: u64,
    pub total_slashed: u64,

    /// Reserved bytes so we can add fields without a migration.
    pub _reserved: [u8; 64],
}

impl ProtocolState {
    /// Discriminator (8) + body.
    pub const SIZE: usize = 8 + 32 + 32 + 32 + 1 + 1 + 1 + 2 + 8 + 8 + 8 + 8 + 8 + 8 + 64;
}
