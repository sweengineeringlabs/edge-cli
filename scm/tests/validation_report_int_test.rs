//! Integration tests for ValidationReport and ValidationFailure.

use std::path::PathBuf;
use swe_edge_bin::{ValidationFailure, ValidationReport};

/// @covers: ValidationReport::new
#[test]
fn test_validation_report_new_is_clean() {
    let report = ValidationReport::new();
    assert!(report.is_clean());
    assert!(report.passed.is_empty());
    assert!(report.failed.is_empty());
}

/// @covers: ValidationReport::add_passed
#[test]
fn test_validation_report_add_passed() {
    let mut report = ValidationReport::new();
    report.add_passed("Check 1 passed".to_string());
    assert!(report.is_clean());
    assert_eq!(report.passed.len(), 1);
}

/// @covers: ValidationReport::add_failed
#[test]
fn test_validation_report_add_failed() {
    let mut report = ValidationReport::new();
    report.add_failed(PathBuf::from("/test/path"), "Check failed".to_string());
    assert!(!report.is_clean());
    assert_eq!(report.failed.len(), 1);
    assert_eq!(report.failed[0].message, "Check failed");
}

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

/// @covers: ValidationReport::is_clean
#[test]
fn test_validation_report_is_clean_returns_false_when_failed() {
    let mut report = ValidationReport::new();
    assert!(report.is_clean());
    report.add_failed(PathBuf::from("/p"), "fail".to_string());
    assert!(!report.is_clean());
}
