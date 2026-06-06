//! Integration tests for SAF (Service Abstraction Framework) public API.

use swe_edge_bin::BinSvc;

/// @covers: BinSvc::domain_validator
#[test]
fn test_domain_validator_factory_returns_validator() {
    let svc = BinSvc::new();
    let validator = svc.domain_validator();
    assert!(std::any::type_name_of_val(&validator).contains("Validator"));
}

/// @covers: BinSvc::config_validator
#[test]
fn test_config_validator_factory_returns_validator() {
    let svc = BinSvc::new();
    let validator = svc.config_validator();
    assert!(std::any::type_name_of_val(&validator).contains("Validator"));
}

/// @covers: BinSvc::curl_scaffolder
#[test]
fn test_curl_scaffolder_factory_returns_scaffolder() {
    let svc = BinSvc::new();
    let scaffolder = svc.curl_scaffolder();
    assert!(std::any::type_name_of_val(&scaffolder).contains("Scaffolder"));
}

/// @covers: BinSvc
#[test]
fn test_saf_exports_are_accessible() {
    let _config = swe_edge_bin::Config::default();
    let _command = swe_edge_bin::Command::Validate {
        path: std::path::PathBuf::from("."),
    };
}
