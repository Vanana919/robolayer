//! Account state for the RoboLayer program.
//!
//! Each submodule defines one account type. They are re-exported here
//! so callers can `use crate::state::{ProtocolState, Operator, Task};`.

pub mod operator;
pub mod protocol;
pub mod task;

pub use operator::*;
pub use protocol::*;
pub use task::*;
