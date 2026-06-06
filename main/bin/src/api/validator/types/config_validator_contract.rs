//! Config validator interface contract.
//!
//! `core/validator/config.rs` implements this as `ConfigValidator`.

use crate::api::validator::traits::validator::Validator;

/// Type alias for the config validator implementation contract.
///
/// Downstream callers that need a boxed config validator can use this type alias.
pub type ConfigValidatorContract = dyn Validator;
