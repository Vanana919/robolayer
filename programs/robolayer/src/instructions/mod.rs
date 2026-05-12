//! Instruction handlers grouped one-per-file.
//!
//! Each file exports a `pub fn handler(...)` and an `Accounts` struct.
//! `lib.rs` re-exports them so the `#[program]` module can stay short.

pub mod claim_task;
pub mod finalize_task;
pub mod initialize;
pub mod register_operator;
pub mod slash_operator;
pub mod submit_result;
pub mod submit_task;
pub mod withdraw_stake;

pub use claim_task::*;
pub use finalize_task::*;
pub use initialize::*;
pub use register_operator::*;
pub use slash_operator::*;
pub use submit_result::*;
pub use submit_task::*;
pub use withdraw_stake::*;
