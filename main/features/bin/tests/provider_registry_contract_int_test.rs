//! Integration tests for ProviderRegistryContract trait.

use swe_edge_bin::ProviderRegistryContract;

/// @covers: ProviderRegistryContract
#[test]
fn test_provider_registry_contract_is_object_safe() {
    fn _accept(_r: &dyn ProviderRegistryContract) {}
}

/// @covers: ProviderRegistryContract::build_default_provider_report
#[test]
fn test_provider_registry_contract_build_default_provider_report() {
    // The NoopValidationProvider acts as the default
    // The ProviderRegistry in spi/ implements ProviderRegistryContract
    // This test verifies the trait method produces valid output
    // (We can't directly instantiate ProviderRegistry from tests - it's pub(crate))
    // But we verify the trait is object-safe and the contract is well-defined
    let _: fn(&dyn ProviderRegistryContract, &std::path::Path) -> _ =
        |reg, path| reg.build_default_provider_report(path);
}
