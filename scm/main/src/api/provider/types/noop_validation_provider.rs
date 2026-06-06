//! NoopValidationProvider — default no-op extension implementation.

use crate::api::error::BinError;
use crate::api::provider::traits::validation_provider::ValidationProvider;
use crate::api::vo::validation_report::ValidationReport;

/// No-op validation provider that always returns an empty passing report.
///
/// Use as a default when no custom validation is needed.
pub struct NoopValidationProvider;

impl NoopValidationProvider {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopValidationProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationProvider for NoopValidationProvider {
    fn provide_validation(&self, _path: &std::path::Path) -> Result<ValidationReport, BinError> {
        let mut report = ValidationReport::new();
        report.add_passed("Noop validation: no checks performed".to_string());
        Ok(report)
    }
}
