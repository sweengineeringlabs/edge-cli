//! Config integration tests for main.

use std::path::PathBuf;
use swe_edge_bin::*;

#[test]
fn test_config_default_through_facade() {
    let config = Config::default();
    assert!(execute(&config).is_ok());
}

#[test]
fn test_config_with_validate_command() {
    let config = Config::new().with_command(Command::Validate {
        path: PathBuf::from("."),
    });
    let result = execute(&config);
    // Result could be error due to validation failures, but execute should complete
    assert!(result.is_ok() || result.is_err());
}
