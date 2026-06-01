//! Integration tests for ProviderRegistryContract.

use swe_edge_bin::ProviderRegistryContract;

/// @covers: ProviderRegistryContract
#[test]
fn test_provider_registry_contract_is_object_safe() {
    fn _accept(_r: &dyn ProviderRegistryContract) {}
}
