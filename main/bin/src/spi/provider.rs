//! SPI provider registry — manages default validation providers.

use crate::api::error::BinError;
use crate::api::spi::NoopValidationProvider;
use crate::api::traits::provider_registry_contract::ProviderRegistryContract;
use crate::api::traits::validation_provider::ValidationProvider;
use crate::api::types::validation::validation_report::ValidationReport;

/// Registry for validation providers.
///
/// Manages which `ValidationProvider` implementation is used at runtime.
pub(crate) struct ProviderRegistry;

impl ProviderRegistry {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl ProviderRegistryContract for ProviderRegistry {
    fn build_default_provider_report(
        &self,
        path: &std::path::Path,
    ) -> Result<ValidationReport, BinError> {
        NoopValidationProvider::new().provide_validation(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_provider_registry() {
        let _r = ProviderRegistry::new();
    }

    #[test]
    fn test_registry_build_default_provider_report() {
        let registry = ProviderRegistry::new();
        let temp = std::path::PathBuf::from(".");
        let report = registry.build_default_provider_report(&temp).unwrap();
        assert!(report.is_clean());
    }
}
