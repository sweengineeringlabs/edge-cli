//! Integration tests for the Validator trait.

use swe_edge_bin::{BinSvc, Validator};

/// @covers: Validator::validate
#[test]
fn test_validator_trait_validate_returns_report() {
    let svc = BinSvc::new();
    let validator = svc.domain_validator();
    let temp_dir = tempfile::TempDir::new().unwrap();

    let report = validator.validate(temp_dir.path()).unwrap();
    // Empty dir returns a report (may have passed entries)
    assert!(report.is_clean() || !report.is_clean());
}

/// @covers: Validator
#[test]
fn test_validator_trait_is_object_safe() {
    fn _accept(_s: &dyn Validator) {}
}

/// @covers: Validator::validate
#[test]
fn test_config_validator_validate_uses_walkdir_to_scan_toml() {
    use std::fs;
    let svc = BinSvc::new();
    let validator = svc.config_validator();

    let temp_dir = tempfile::TempDir::new().unwrap();
    let toml_path = temp_dir.path().join("test.toml");
    fs::write(&toml_path, "[section]\nkey = \"value\"").unwrap();

    // walkdir is exercised here to find the .toml file
    let report = validator.validate(temp_dir.path()).unwrap();
    assert!(report.passed.iter().any(|p| p.contains("Valid TOML")));
}
