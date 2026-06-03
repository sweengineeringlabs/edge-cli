//! Integration tests for ValidationProvider trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_bin::{NoopValidationProvider, ValidationProvider};

/// @covers: ValidationProvider::provide_validation
#[test]
fn test_validation_provider_noop_returns_clean_report() {
    let provider = NoopValidationProvider::new();
    let temp = tempfile::TempDir::new().unwrap();
    let report = provider.provide_validation(temp.path()).unwrap();
    assert!(report.is_clean());
}

/// @covers: ValidationProvider
#[test]
fn test_validation_provider_is_object_safe() {
    fn _accept(_p: &dyn ValidationProvider) {}
}
