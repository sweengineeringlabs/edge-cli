//! Integration tests for BinSvc type.

use swe_edge_bin::{BinSvc, Config};

/// @covers: BinSvc::new
#[test]
fn test_bin_svc_new() {
    let _svc = BinSvc::new();
}

/// @covers: BinSvc::execute
#[test]
fn test_bin_svc_execute_default_config() {
    let svc = BinSvc::new();
    let config = Config::default();
    assert!(svc.execute(&config).is_ok());
}
