//! Integration tests for NoopValidationProvider.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_bin::{NoopValidationProvider, ValidationProvider};

/// @covers: NoopValidationProvider::new
#[test]
fn test_noop_validation_provider_new() {
    let _p = NoopValidationProvider::new();
}

/// @covers: NoopValidationProvider::provide_validation
#[test]
fn test_noop_validation_provider_always_passes() {
    let provider = NoopValidationProvider::new();
    let temp = tempfile::TempDir::new().unwrap();
    let report = provider.provide_validation(temp.path()).unwrap();
    assert!(report.is_clean());
}

/// @covers: NoopValidationProvider
#[test]
fn test_noop_validation_provider_implements_provider_trait() {
    fn _accept(_p: &dyn ValidationProvider) {}
    let provider = NoopValidationProvider::new();
    _accept(&provider);
}
