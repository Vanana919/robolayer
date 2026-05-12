//! Protocol-wide constants, PDA seed prefixes and small math helpers.
//!
//! Keeping these in a single module makes it easy to audit economic
//! parameters (stake floors, slash ratios, timing windows) without
//! grepping the whole crate.

use anchor_lang::prelude::*;

use crate::errors::RoboError;

// -----------------------------------------------------------------------------
// Economic constants
// -----------------------------------------------------------------------------

/// ROBO has 9 decimals (matches SOL / typical SPL convention).
pub const ROBO_DECIMALS: u8 = 9;

/// 1 ROBO expressed in base units.
pub const ROBO_UNIT: u64 = 1_000_000_000;

/// Minimum stake required to register as an operator: 10,000 ROBO.
pub const MIN_OPERATOR_STAKE: u64 = 10_000 * ROBO_UNIT;

/// Default slash applied when an operator misbehaves (10.00%).
pub const SLASH_BPS_DEFAULT: u16 = 1_000;

/// Hard cap on slash ratio to prevent governance from confiscating
/// 100% of stake in a single instruction (50.00%).
pub const SLASH_BPS_MAX: u16 = 5_000;

/// Basis-point denominator.
pub const BPS_DENOMINATOR: u64 = 10_000;

/// Initial reputation given to a freshly registered operator. The
/// reputation is a soft signal off-chain matchers can use to rank
/// operators; it does not gate stake math directly.
pub const INITIAL_REPUTATION: u32 = 1_000;

/// Reputation gained for each successfully finalized task.
pub const REPUTATION_REWARD: u32 = 5;

/// Reputation lost when an operator is slashed.
pub const REPUTATION_PENALTY: u32 = 250;

// -----------------------------------------------------------------------------
// Timing constants (seconds)
// -----------------------------------------------------------------------------

/// How long an operator has, after claiming, to submit a result before
/// the task can be reassigned.
pub const TASK_TIMEOUT_SECS: i64 = 3_600; // 1h

/// Optimistic challenge window. After a result is submitted, anyone
/// may submit a challenge until this window expires.
pub const CHALLENGE_WINDOW_SECS: i64 = 7_200; // 2h

/// Operators must wait this long after `withdraw_stake` is initiated
/// before funds can be moved out of the treasury.
pub const UNBONDING_PERIOD_SECS: i64 = 14 * 86_400; // 14 days

/// Maximum reward a single task may carry. Keeps individual task
/// griefing bounded.
pub const MAX_TASK_REWARD: u64 = 1_000_000 * ROBO_UNIT;

// -----------------------------------------------------------------------------
// PDA seed prefixes
// -----------------------------------------------------------------------------

pub const PROTOCOL_SEED: &[u8] = b"protocol";
pub const OPERATOR_SEED: &[u8] = b"operator";
pub const TASK_SEED: &[u8] = b"task";
pub const TREASURY_SEED: &[u8] = b"treasury";
pub const ESCROW_SEED: &[u8] = b"task_escrow";

// -----------------------------------------------------------------------------
// Math helpers
// -----------------------------------------------------------------------------

/// `amount * bps / 10_000` with overflow check. Returns an anchor
/// `Result<u64>` so call-sites can use `?` directly.
pub fn apply_bps(amount: u64, bps: u16) -> Result<u64> {
    let product = (amount as u128)
        .checked_mul(bps as u128)
        .ok_or(RoboError::ArithmeticOverflow)?;
    let result = product
        .checked_div(BPS_DENOMINATOR as u128)
        .ok_or(RoboError::ArithmeticOverflow)?;
    u64::try_from(result).map_err(|_| error!(RoboError::ArithmeticOverflow))
}

/// Returns `true` if every bit set in `required` is also set in
/// `supported`. Used to match task capability requirements against
/// operator capability bitmasks.
pub fn capability_match(supported: u64, required: u64) -> bool {
    (supported & required) == required
}

/// Saturating reputation decrement. Reputation cannot go below zero;
/// using saturating math avoids one extra branch in the slashing path.
pub fn decrement_reputation(current: u32, penalty: u32) -> u32 {
    current.saturating_sub(penalty)
}
