//! Unit tests for main.

use main::*;

/// @covers: execute
#[test]
fn test_execute_with_default_config() {
    let config = Config::default();
    assert!(execute(&config).is_ok());
}

/// @covers: run
#[test]
fn test_run_with_defaults() {
    assert!(run().is_ok());
}
