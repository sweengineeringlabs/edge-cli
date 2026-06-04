//! ValidationProvider — SPI extension hook for custom validation.

use crate::api::error::BinError;
use crate::api::types::validation::validation_report::ValidationReport;

/// Extension trait for custom validation providers.
///
/// Downstream consumers implement this to plug in custom validation logic.
/// The presence of `src/spi/` signals this crate is designed for downstream extension.
pub trait ValidationProvider: Send + Sync {
    fn provide_validation(&self, path: &std::path::Path) -> Result<ValidationReport, BinError>;
}
