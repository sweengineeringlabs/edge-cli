//! Integration tests for MainContract type alias.

use swe_edge_bin::{BinSvc, MainContract};

/// @covers: MainContract
#[test]
fn test_main_contract_is_object_safe() {
    fn _accept(_m: &MainContract) {}
}

/// @covers: MainContract
#[test]
fn test_bin_svc_implements_main_contract() {
    let svc = BinSvc::new();
    let config = swe_edge_bin::Config::default();
    // BinSvc implements the Main trait
    assert!(svc.execute(&config).is_ok());
}
