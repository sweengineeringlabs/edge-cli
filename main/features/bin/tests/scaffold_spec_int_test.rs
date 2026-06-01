//! Integration tests for ScaffoldSpec.

use swe_edge_bin::{HandlerSpec, ScaffoldSpec};

/// @covers: ScaffoldSpec
#[test]
fn test_scaffold_spec_constructs() {
    let spec = ScaffoldSpec {
        handlers: vec![HandlerSpec {
            id: "test".to_string(),
            pattern: "/test".to_string(),
            method: "POST".to_string(),
        }],
        base_url: "http://localhost:8080".to_string(),
    };
    assert_eq!(spec.handlers.len(), 1);
    assert_eq!(spec.base_url, "http://localhost:8080");
}
