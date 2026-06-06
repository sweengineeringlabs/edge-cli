//! Validator trait definition.

use std::path::Path;

use crate::api::error::BinError;
use crate::api::vo::validation_report::ValidationReport;

/// Validates project structure against domain and config contracts.
pub trait Validator: Send + Sync {
    fn validate(&self, path: &Path) -> Result<ValidationReport, BinError>;
}
