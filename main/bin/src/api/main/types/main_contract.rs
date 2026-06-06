//! Main interface contract.
//!
//! `core/main/default.rs` implements the `Main` trait as `DefaultMain`.

use crate::api::main::traits::main::Main;

/// Type alias for the primary Main implementation contract.
///
/// Downstream callers that need a box of the main service can use this type alias.
pub type MainContract = dyn Main;
