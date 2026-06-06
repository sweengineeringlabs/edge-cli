//! SAF facade layer — public surface.
//!
//! Re-exports types from api/ and exposes BinSvc for CLI dispatch.

mod bin_svc;

// Primary facade type
pub use crate::api::types::bin_svc::BinSvc;

// XDG-compliant path resolution
pub use bin_svc::default_config_path;

// Public error type
pub use crate::api::error::BinError;

// Public data types
pub use crate::api::main::vo::config::Config;
pub use crate::api::scaffold::vo::handler_spec::HandlerSpec;
pub use crate::api::scaffold::vo::scaffold_report::ScaffoldReport;
pub use crate::api::scaffold::vo::scaffold_spec::ScaffoldSpec;
pub use crate::api::vo::command::Command;
pub use crate::api::vo::validation_failure::ValidationFailure;
pub use crate::api::vo::validation_report::ValidationReport;
