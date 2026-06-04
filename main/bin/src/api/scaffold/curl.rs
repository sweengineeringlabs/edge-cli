//! Interface counterpart for core/scaffold/curl.rs.
//!
//! `core/scaffold/curl.rs` implements the `Scaffolder` trait as `CurlScaffolder`.

use crate::api::traits::scaffolder::Scaffolder;

/// Trait object bound for the curl scaffolder implementation.
///
/// File-level counterpart type: `api/scaffold/curl.rs` → `Curl` (bound to `dyn Scaffolder`).
pub type Curl = dyn Scaffolder;
