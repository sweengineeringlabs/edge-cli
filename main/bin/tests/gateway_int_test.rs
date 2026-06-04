//! Integration tests for gateway layer exports.

use swe_edge_bin::{BinSvc, Config, Main, Scaffolder, Validator};

/// @covers: gateway
#[test]
fn test_gateway_exports_bin_svc() {
    let _svc = BinSvc::new();
}

/// @covers: gateway
#[test]
fn test_gateway_exports_traits() {
    fn _accept_validator(_s: &dyn Validator) {}
    fn _accept_scaffolder(_s: &dyn Scaffolder) {}
    fn _accept_main(_s: &dyn Main) {}
}

/// @covers: gateway
#[test]
fn test_gateway_exports_config() {
    let _config = Config::default();
}
