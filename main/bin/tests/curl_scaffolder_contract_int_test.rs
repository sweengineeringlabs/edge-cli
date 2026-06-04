//! Integration tests for CurlScaffolderContract.

use swe_edge_bin::{BinSvc, CurlScaffolderContract};

/// @covers: CurlScaffolderContract
#[test]
fn test_curl_scaffolder_contract_is_dyn_scaffolder() {
    // CurlScaffolderContract is a type alias for dyn Scaffolder
    // Verify the factory returns an implementor
    let svc = BinSvc::new();
    let _s: Box<CurlScaffolderContract> = svc.curl_scaffolder();
}
