//! Config integration tests for main.

use main::*;

#[test]
fn test_config_default_through_facade() {
    let config = Config::default();
    assert!(execute(&config).is_ok());
}

#[test]
fn test_config_through_run() {
    assert!(run().is_ok());
}
