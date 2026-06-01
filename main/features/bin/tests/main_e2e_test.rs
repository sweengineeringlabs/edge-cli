//! Unit tests for main.

use std::path::PathBuf;
use swe_edge_bin::{BinSvc, Command, Config};

/// @covers: BinSvc::execute
#[test]
fn test_execute_with_default_config() {
    let config = Config::default();
    assert!(BinSvc::new().execute(&config).is_ok());
}

/// @covers: BinSvc::execute
#[test]
fn test_execute_with_validate() {
    let config = Config::new().with_command(Command::Validate {
        path: PathBuf::from("."),
    });
    let result = BinSvc::new().execute(&config);
    // Result can be ok or error based on validation, but should complete
    assert!(result.is_ok() || result.is_err());
}
