//! Curl scaffolder interface contract.
//!
//! `core/scaffold/curl.rs` implements this as `CurlScaffolder`.

use crate::api::traits::scaffolder::Scaffolder;

/// Type alias for the curl scaffolder implementation contract.
///
/// Downstream callers that need a boxed curl scaffolder can use this type alias.
pub type CurlScaffolderContract = dyn Scaffolder;
