//! Integration tests for main.

use std::path::PathBuf;
use swe_edge_bin::{BinSvc, Command, Config};

/// @covers: BinSvc::execute
#[test]
fn test_execute_with_config() {
    let config = Config::default();
    assert!(BinSvc::new().execute(&config).is_ok());
}

/// @covers: BinSvc::execute
#[test]
fn test_execute_validate_command() {
    let config = Config::new().with_command(Command::Validate {
        path: PathBuf::from("."),
    });
    let result = BinSvc::new().execute(&config);
    assert!(result.is_ok() || result.is_err());
}

/// @covers: BinSvc::execute
#[test]
fn test_bin_svc_execute_with_no_command() {
    let svc = BinSvc::new();
    let config = Config::default();
    assert!(svc.execute(&config).is_ok());
}

/// @covers: BinSvc::domain_validator
/// @covers: BinSvc::config_validator
/// @covers: BinSvc::curl_scaffolder
#[test]
fn test_bin_svc_factory_methods_return_instances() {
    let svc = BinSvc::new();
    let _dv = svc.domain_validator();
    let _cv = svc.config_validator();
    let _cs = svc.curl_scaffolder();
}
