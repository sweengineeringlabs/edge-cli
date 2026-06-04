//! Integration tests for HandlerSpec.

use swe_edge_bin::HandlerSpec;

/// @covers: HandlerSpec
#[test]
fn test_handler_spec_constructs_with_fields() {
    let spec = HandlerSpec {
        id: "list_items".to_string(),
        pattern: "/items".to_string(),
        method: "GET".to_string(),
    };
    assert_eq!(spec.id, "list_items");
    assert_eq!(spec.pattern, "/items");
    assert_eq!(spec.method, "GET");
}

/// @covers: HandlerSpec
#[test]
fn test_handler_spec_clone() {
    let spec = HandlerSpec {
        id: "test".to_string(),
        pattern: "/test".to_string(),
        method: "POST".to_string(),
    };
    let cloned = spec.clone();
    assert_eq!(cloned.id, spec.id);
}
