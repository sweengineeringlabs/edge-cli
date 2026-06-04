//! ProviderRegistry trait definition.

use crate::api::error::BinError;
use crate::api::types::validation::validation_report::ValidationReport;

/// Manages which `ValidationProvider` implementation is used at runtime.
pub trait ProviderRegistryContract: Send + Sync {
    fn build_default_provider_report(
        &self,
        path: &std::path::Path,
    ) -> Result<ValidationReport, BinError>;
}
