//! Integration tests for Command type.

use std::path::PathBuf;
use swe_edge_bin::Command;

/// @covers: Command::Validate
#[test]
fn test_command_validate_variant_constructs() {
    let cmd = Command::Validate {
        path: PathBuf::from("/tmp"),
    };
    match cmd {
        Command::Validate { path } => assert_eq!(path, PathBuf::from("/tmp")),
        _ => panic!("Expected Validate variant"),
    }
}

/// @covers: Command::TestGen
#[test]
fn test_command_testgen_variant_constructs() {
    let cmd = Command::TestGen {
        path: PathBuf::from("/src"),
        output: PathBuf::from("/out"),
        base_url: "http://localhost:8080".to_string(),
    };
    match cmd {
        Command::TestGen {
            path,
            output,
            base_url,
        } => {
            assert_eq!(path, PathBuf::from("/src"));
            assert_eq!(output, PathBuf::from("/out"));
            assert_eq!(base_url, "http://localhost:8080");
        }
        _ => panic!("Expected TestGen variant"),
    }
}

/// @covers: Command
#[test]
fn test_command_derives_debug_clone() {
    let cmd = Command::Validate {
        path: PathBuf::from("."),
    };
    let cloned = cmd.clone();
    assert_eq!(format!("{:?}", cmd), format!("{:?}", cloned));
}
