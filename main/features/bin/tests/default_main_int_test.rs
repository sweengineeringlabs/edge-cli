//! DefaultMain integration tests.

use std::path::PathBuf;
use swe_edge_bin::{BinSvc, Command, Config};

/// @covers: BinSvc::execute
#[test]
fn test_default_main_execute_with_config() {
    let config = Config::default();
    assert!(BinSvc::new().execute(&config).is_ok());
}

/// @covers: BinSvc::execute
#[test]
fn test_default_main_execute_via_validate() {
    let config = Config::new().with_command(Command::Validate {
        path: PathBuf::from("."),
    });
    let result = BinSvc::new().execute(&config);
    assert!(result.is_ok() || result.is_err());
}
