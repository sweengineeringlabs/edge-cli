//! Interface counterpart for core/validator/config.rs.
//!
//! `core/validator/config.rs` implements the `Validator` trait as `ConfigValidator`.

use crate::api::validator::traits::validator::Validator;

/// Trait object bound for the config validator implementation.
///
/// File-level counterpart type: `api/validator/config.rs` → `Config` (bound to `dyn Validator`).
pub type Config = dyn Validator;
