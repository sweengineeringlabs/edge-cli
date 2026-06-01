//! Integration tests for ValidationFailure type.

use std::path::PathBuf;
use swe_edge_bin::ValidationFailure;

/// @covers: ValidationFailure
#[test]
fn test_validation_failure_stores_path_and_message() {
    let path = PathBuf::from("/some/path");
    let msg = "Something went wrong".to_string();
    let failure = ValidationFailure {
        path: path.clone(),
        message: msg.clone(),
    };
    assert_eq!(failure.path, path);
    assert_eq!(failure.message, msg);
}

/// @covers: ValidationFailure
#[test]
fn test_validation_failure_clone() {
    let failure = ValidationFailure {
        path: PathBuf::from("/p"),
        message: "err".to_string(),
    };
    let cloned = failure.clone();
    assert_eq!(cloned.path, failure.path);
    assert_eq!(cloned.message, failure.message);
}
