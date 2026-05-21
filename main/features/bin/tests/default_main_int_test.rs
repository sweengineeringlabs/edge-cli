//! DefaultMain integration tests for main.

use main::*;

/// @covers: DefaultMain
#[test]
fn test_default_main_execute_via_facade() {
    assert!(run().is_ok());
}

/// @covers: DefaultMain
#[test]
fn test_default_main_execute_with_config() {
    let config = Config::default();
    assert!(execute(&config).is_ok());
}
