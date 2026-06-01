//! Domain validator interface contract.
//!
//! `core/validator/domain.rs` implements this as `DomainValidator`.

use crate::api::traits::validator::Validator;

/// Type alias for the domain validator implementation contract.
///
/// Downstream callers that need a boxed domain validator can use this type alias.
pub type DomainValidatorContract = dyn Validator;
