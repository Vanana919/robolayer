//! On-chain events emitted by RoboLayer instructions.
//!
//! Indexers and off-chain matchers subscribe to these to keep their
//! view of the task book in sync without re-fetching account state.

use anchor_lang::prelude::*;

#[event]
pub struct ProtocolInitialized {
    pub authority: Pubkey,
    pub robo_mint: Pubkey,
    pub treasury: Pubkey,
    pub ts: i64,
}

#[event]
pub struct OperatorRegistered {
    pub operator: Pubkey,
    pub stake: u64,
    pub capabilities: u64,
    pub ts: i64,
}

#[event]
pub struct TaskSubmitted {
    pub task: Pubkey,
    pub requester: Pubkey,
    pub reward: u64,
    pub deadline: i64,
    pub required_capabilities: u64,
    pub ts: i64,
}

#[event]
pub struct TaskClaimed {
    pub task: Pubkey,
    pub operator: Pubkey,
    pub ts: i64,
}

#[event]
pub struct TaskResultSubmitted {
    pub task: Pubkey,
    pub operator: Pubkey,
    pub result_hash: [u8; 32],
    pub challenge_deadline: i64,
    pub ts: i64,
}

#[event]
pub struct TaskFinalized {
    pub task: Pubkey,
    pub operator: Pubkey,
    pub reward_paid: u64,
    pub ts: i64,
}

#[event]
pub struct OperatorSlashed {
    pub operator: Pubkey,
    pub amount: u64,
    pub remaining_stake: u64,
    pub reason_code: u32,
    pub ts: i64,
}

#[event]
pub struct WithdrawalRequested {
    pub operator: Pubkey,
    pub amount: u64,
    pub unlock_ts: i64,
    pub ts: i64,
}

#[event]
pub struct WithdrawalCompleted {
    pub operator: Pubkey,
    pub amount: u64,
    pub ts: i64,
}
