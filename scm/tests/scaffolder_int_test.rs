//! Integration tests for the Scaffolder trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_bin::{BinSvc, HandlerSpec, ScaffoldSpec, Scaffolder};

/// @covers: Scaffolder::scaffold
#[test]
fn test_scaffolder_trait_scaffold_generates_files() {
    let svc = BinSvc::new();
    let scaffolder = svc.curl_scaffolder();
    let spec = ScaffoldSpec {
        handlers: vec![HandlerSpec {
            id: "health".to_string(),
            pattern: "/health".to_string(),
            method: "GET".to_string(),
        }],
        base_url: "http://localhost:8080".to_string(),
    };

    let temp_dir = tempfile::TempDir::new().unwrap();
    let report = scaffolder.scaffold(&spec, temp_dir.path()).unwrap();

    assert!(!report.files_written.is_empty());
}

/// @covers: Scaffolder
#[test]
fn test_scaffolder_trait_is_object_safe() {
    fn _accept(_s: &dyn Scaffolder) {}
}
