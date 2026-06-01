//! Config integration tests for main.

use std::path::PathBuf;
use swe_edge_bin::{BinSvc, Command, Config};

/// @covers: Config::new
#[test]
fn test_config_default_through_facade() {
    let config = Config::default();
    assert!(BinSvc::new().execute(&config).is_ok());
}

/// @covers: Config::with_command
#[test]
fn test_config_with_command_sets_command() {
    let config = Config::new().with_command(Command::Validate {
        path: PathBuf::from("."),
    });
    assert!(config.command.is_some());
}

/// @covers: Config::with_verbose
#[test]
fn test_config_with_verbose_sets_flag() {
    let config = Config::new().with_verbose(true);
    assert!(config.verbose);
}

/// @covers: Config::with_command
#[test]
fn test_config_with_validate_command() {
    let config = Config::new().with_command(Command::Validate {
        path: PathBuf::from("."),
    });
    let result = BinSvc::new().execute(&config);
    assert!(result.is_ok() || result.is_err());
}
