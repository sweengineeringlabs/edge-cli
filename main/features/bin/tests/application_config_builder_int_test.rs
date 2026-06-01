//! Integration tests for ApplicationConfigBuilder.

use swe_edge_bin::ApplicationConfigBuilder;

/// @covers: ApplicationConfigBuilder::new
#[test]
fn test_application_config_builder_new() {
    let builder = ApplicationConfigBuilder::new();
    assert_eq!(builder.name(), "swe-edge-bin");
    assert_eq!(builder.log_level(), "info");
}

/// @covers: ApplicationConfigBuilder::with_name
#[test]
fn test_application_config_builder_with_name() {
    let builder = ApplicationConfigBuilder::new().with_name("my-app".to_string());
    assert_eq!(builder.name(), "my-app");
}

/// @covers: ApplicationConfigBuilder::with_log_level
#[test]
fn test_application_config_builder_with_log_level() {
    let builder = ApplicationConfigBuilder::new().with_log_level("debug".to_string());
    assert_eq!(builder.log_level(), "debug");
}
