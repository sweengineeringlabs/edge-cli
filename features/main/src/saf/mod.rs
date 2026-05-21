//! SAF facade layer (L4) - public surface.
//!
//! Re-export types from api/ and expose core functionality
//! via standalone public functions.

mod facade;

// Re-export public types
pub use crate::api::commands::Command;
pub use crate::api::config::Config;
pub use crate::api::error::Error;
pub use crate::api::main::Main;
pub use crate::api::scaffolder::{HandlerSpec, ScaffoldReport, ScaffoldSpec, TestScaffolder};
pub use crate::api::validator::{ValidationFailure, ValidationReport, Validator};

// Re-export facade functions
pub use facade::{config_validator, curl_scaffolder, domain_validator, execute, run};
