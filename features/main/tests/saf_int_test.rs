//! Integration tests for SAF (Service Abstraction Framework) public API.

use main::{config_validator, curl_scaffolder, domain_validator};

#[test]
fn test_domain_validator_factory_returns_validator() {
    let validator = domain_validator();
    assert!(std::any::type_name_of_val(&validator).contains("Validator"));
}

#[test]
fn test_config_validator_factory_returns_validator() {
    let validator = config_validator();
    assert!(std::any::type_name_of_val(&validator).contains("Validator"));
}

#[test]
fn test_curl_scaffolder_factory_returns_scaffolder() {
    let scaffolder = curl_scaffolder();
    assert!(std::any::type_name_of_val(&scaffolder).contains("Scaffolder"));
}

#[test]
fn test_saf_exports_are_accessible() {
    // Test that core types from saf are publicly accessible
    let _config = main::Config::default();
    let _command = main::Command::Validate {
        path: std::path::PathBuf::from("."),
    };
}
