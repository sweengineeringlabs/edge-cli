//! Scaffolder trait definition.

use crate::api::error::BinError;
use crate::api::types::scaffold::scaffold_report::ScaffoldReport;
use crate::api::types::scaffold::scaffold_spec::ScaffoldSpec;

/// Generates integration test files from a specification.
pub trait Scaffolder: Send + Sync {
    fn scaffold(
        &self,
        spec: &ScaffoldSpec,
        output: &std::path::Path,
    ) -> Result<ScaffoldReport, BinError>;
}
