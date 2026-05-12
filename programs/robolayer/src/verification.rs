//! Verification strategies for task results.
//!
//! RoboLayer supports a small set of verification modes, all expressed
//! by the same `VerificationMode` enum so the runtime can dispatch on
//! it uniformly:
//!
//! * **Optimistic** — accept the operator's result by default, but
//!   anyone may submit a challenge during the challenge window. If a
//!   challenge succeeds the operator is slashed and the task is voided.
//!   This is the cheapest mode and the only one suitable for tasks
//!   whose result is hard to verify on-chain (e.g. image
//!   classification, agent inference).
//!
//! * **NofM** — a quorum of `n` honest votes from `m` assigned
//!   operators is required for the result to be accepted. Used for
//!   high-value tasks where the cost of running the workload multiple
//!   times is acceptable.
//!
//! * **Zk** — verify a Groth16 / SP1 / Risc0 proof on-chain via CPI to
//!   a dedicated verifier program. Behind the `zk` feature flag because
//!   the verifier program is not yet deployed; the public roadmap
//!   tracks this work in section 3.4. Enabling the feature will let
//!   downstream code branch on `Zk`, but the actual proof check is
//!   stubbed with `todo!()` until the verifier is live.

use anchor_lang::prelude::*;

use crate::errors::RoboError;

/// Maximum value of `n` and `m` for `NofM` verification. Keeps the
/// vote-accounting account size constant at the IDL level.
pub const MAX_QUORUM_PARTICIPANTS: u8 = 16;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum VerificationMode {
    /// Single-operator submission with a challenge window in seconds.
    Optimistic { challenge_window: i64 },

    /// Require `n` out of `m` operators to vote the same result hash.
    NofM { n: u8, m: u8 },

    /// Verify a succinct proof via CPI into `verifier_program`.
    /// Roadmap-only — see module docs and `verify_zk` for details.
    #[cfg(feature = "zk")]
    Zk { verifier_program: Pubkey },
}

impl Default for VerificationMode {
    /// Sensible default used when an account is zero-initialized via
    /// `#[account]`. Defaults to an optimistic mode with the protocol-
    /// wide challenge window; concrete tasks always override this in
    /// `submit_task`.
    fn default() -> Self {
        VerificationMode::Optimistic {
            challenge_window: crate::utils::CHALLENGE_WINDOW_SECS,
        }
    }
}

impl VerificationMode {
    /// Lightweight sanity checks the runtime can call at task creation
    /// time. Returns `Err` for nonsense configurations so they never
    /// reach the finalization path.
    pub fn validate(&self) -> Result<()> {
        match self {
            VerificationMode::Optimistic { challenge_window } => {
                require!(*challenge_window > 0, RoboError::InvalidTaskDeadline);
            }
            VerificationMode::NofM { n, m } => {
                require!(*m > 0, RoboError::QuorumNotReached);
                require!(*n > 0 && n <= m, RoboError::QuorumNotReached);
                require!(
                    *m <= MAX_QUORUM_PARTICIPANTS,
                    RoboError::QuorumNotReached
                );
            }
            #[cfg(feature = "zk")]
            VerificationMode::Zk { verifier_program } => {
                require!(
                    *verifier_program != Pubkey::default(),
                    RoboError::UnsupportedVerificationMode
                );
            }
        }
        Ok(())
    }
}

/// Result of evaluating a verification mode against on-chain state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerificationOutcome {
    /// `true` if the operator's result should be accepted.
    pub accepted: bool,
    /// `true` if a slash should be triggered alongside rejection.
    pub slash: bool,
}

impl VerificationOutcome {
    pub const ACCEPT: Self = Self {
        accepted: true,
        slash: false,
    };
    pub const REJECT: Self = Self {
        accepted: false,
        slash: false,
    };
    pub const REJECT_AND_SLASH: Self = Self {
        accepted: false,
        slash: true,
    };
}

/// Inputs needed to verify a single task. Pulled out into a struct so
/// the call-site at finalization is one line and unit tests can build
/// inputs without touching Anchor `Context`.
pub struct VerifyInputs<'a> {
    pub mode: &'a VerificationMode,
    pub submitted_at: i64,
    pub now: i64,
    /// For `Optimistic`: number of valid challenges observed in the
    /// challenge window. Zero means "no challenge, accept".
    pub challenges: u32,
    /// For `NofM`: how many operators voted in agreement with the
    /// submitted result hash.
    pub matching_votes: u8,
}

/// Top-level dispatch. Returns `VerificationOutcome` rather than
/// mutating accounts so the caller controls all state writes.
pub fn verify(inputs: VerifyInputs) -> Result<VerificationOutcome> {
    match inputs.mode {
        VerificationMode::Optimistic { challenge_window } => {
            verify_optimistic(*challenge_window, &inputs)
        }
        VerificationMode::NofM { n, m } => verify_n_of_m(*n, *m, &inputs),
        #[cfg(feature = "zk")]
        VerificationMode::Zk { verifier_program } => verify_zk(*verifier_program, &inputs),
    }
}

/// Accept if no successful challenge landed before the window closed.
fn verify_optimistic(
    challenge_window: i64,
    inputs: &VerifyInputs,
) -> Result<VerificationOutcome> {
    let window_close = inputs
        .submitted_at
        .checked_add(challenge_window)
        .ok_or(RoboError::ArithmeticOverflow)?;
    require!(inputs.now >= window_close, RoboError::ChallengeWindowOpen);

    if inputs.challenges == 0 {
        Ok(VerificationOutcome::ACCEPT)
    } else {
        // At least one valid challenge succeeded — slash the operator.
        Ok(VerificationOutcome::REJECT_AND_SLASH)
    }
}

/// Accept only if `matching_votes >= n` and a full quorum of `m` voted.
fn verify_n_of_m(n: u8, m: u8, inputs: &VerifyInputs) -> Result<VerificationOutcome> {
    require!(m > 0, RoboError::QuorumNotReached);
    if inputs.matching_votes >= n {
        Ok(VerificationOutcome::ACCEPT)
    } else if inputs.matching_votes == 0 {
        // Nobody confirmed — operator may have submitted a bogus hash.
        Ok(VerificationOutcome::REJECT_AND_SLASH)
    } else {
        Ok(VerificationOutcome::REJECT)
    }
}

#[cfg(feature = "zk")]
fn verify_zk(_verifier_program: Pubkey, _inputs: &VerifyInputs) -> Result<VerificationOutcome> {
    // TODO(roadmap §3.4): wire to the Groth16 verifier program via
    // CPI. The verifier exposes a single `verify_proof(proof,
    // public_inputs)` ix that returns `Ok(())` on success. Once it
    // is live this function should:
    //   1. Pull the proof + public inputs from a sibling account.
    //   2. Build a `CpiContext::new(verifier_program_ai, VerifyProof
    //      { ... })` and invoke it.
    //   3. Map a successful CPI to `VerificationOutcome::ACCEPT`,
    //      a failed CPI to `REJECT_AND_SLASH`.
    //
    // The verifier program is currently in audit; until it is deployed
    // we keep this branch reachable only when the `zk` feature is
    // explicitly enabled so default builds cannot panic.
    todo!("ZK verifier integration is on the roadmap (section 3.4)")
}
