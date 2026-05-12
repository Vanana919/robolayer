//! Custom error codes returned by RoboLayer instructions.
//!
//! Anchor maps `RoboError` variants to numeric codes starting at
//! `6000`. We keep the names short but descriptive so client SDKs can
//! surface them directly to end users.

use anchor_lang::prelude::*;

#[error_code]
pub enum RoboError {
    #[msg("Arithmetic overflow.")]
    ArithmeticOverflow,

    #[msg("Stake is below the protocol minimum.")]
    StakeTooLow,

    #[msg("Operator has already been registered.")]
    OperatorAlreadyRegistered,

    #[msg("Operator account has been slashed and cannot perform actions.")]
    OperatorSlashed,

    #[msg("Operator capabilities do not match the task requirements.")]
    CapabilityMismatch,

    #[msg("Task is not in the expected state for this transition.")]
    InvalidTaskState,

    #[msg("Task reward is outside the permitted bounds.")]
    InvalidTaskReward,

    #[msg("Task deadline must be in the future.")]
    InvalidTaskDeadline,

    #[msg("Task has already expired.")]
    TaskExpired,

    #[msg("Task has already been claimed by another operator.")]
    TaskAlreadyClaimed,

    #[msg("Only the assigned operator may perform this action.")]
    NotAssignedOperator,

    #[msg("Result hash is empty or malformed.")]
    InvalidResultHash,

    #[msg("Challenge window has not yet elapsed.")]
    ChallengeWindowOpen,

    #[msg("Challenge window has already closed.")]
    ChallengeWindowClosed,

    #[msg("Slash ratio exceeds the protocol maximum.")]
    SlashRatioTooHigh,

    #[msg("Unbonding period has not elapsed yet.")]
    UnbondingNotComplete,

    #[msg("Withdrawal amount exceeds available stake.")]
    WithdrawalTooLarge,

    #[msg("Caller is not the protocol authority.")]
    Unauthorized,

    #[msg("Verification mode is not supported in this build.")]
    UnsupportedVerificationMode,

    #[msg("N-of-M verification quorum was not reached.")]
    QuorumNotReached,

    #[msg("Protocol is paused.")]
    ProtocolPaused,

    #[msg("Insufficient escrow balance to pay reward.")]
    InsufficientEscrow,
}
