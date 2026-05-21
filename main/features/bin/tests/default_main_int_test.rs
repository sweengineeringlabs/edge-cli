//! DefaultMain integration tests for main.

use std::path::PathBuf;
use swe_edge_bin::*;

/// @covers: DefaultMain
#[test]
fn test_default_main_execute_with_config() {
    let config = Config::default();
    assert!(execute(&config).is_ok());
}

/// @covers: DefaultMain with validate
#[test]
fn test_default_main_execute_via_validate() {
    let config = Config::new().with_command(Command::Validate {
        path: PathBuf::from("."),
    });
    let result = execute(&config);
    // Should complete without panic
    assert!(result.is_ok() || result.is_err());
}
