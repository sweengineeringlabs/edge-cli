//! SAF facade layer — public surface.
//!
//! Re-exports types from api/ and exposes BinSvc for CLI dispatch.

mod bin_svc;

// Primary facade type
pub use crate::api::bin::BinSvc;

// Public error type
pub use crate::api::error::BinError;

// Public data types
pub use crate::api::types::command::Command;
pub use crate::api::types::config::Config;
pub use crate::api::types::scaffold::handler_spec::HandlerSpec;
pub use crate::api::types::scaffold::scaffold_report::ScaffoldReport;
pub use crate::api::types::scaffold::scaffold_spec::ScaffoldSpec;
pub use crate::api::types::validation::validation_failure::ValidationFailure;
pub use crate::api::types::validation::validation_report::ValidationReport;
