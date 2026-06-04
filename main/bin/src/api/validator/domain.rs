//! Interface counterpart for core/validator/domain.rs.
//!
//! `core/validator/domain.rs` implements the `Validator` trait as `DomainValidator`.

use crate::api::traits::validator::Validator;

/// Trait object bound for the domain validator implementation.
///
/// File-level counterpart type: `api/validator/domain.rs` → `Domain` (bound to `dyn Validator`).
pub type Domain = dyn Validator;
