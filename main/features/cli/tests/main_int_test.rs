//! Integration tests for main.

use main::*;

/// @covers: run
#[test]
fn test_run_succeeds() {
    assert!(run().is_ok());
}

/// @covers: execute
#[test]
fn test_execute_with_config() {
    let config = Config::default();
    assert!(execute(&config).is_ok());
}
