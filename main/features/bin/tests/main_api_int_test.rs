//! API trait integration tests for main.

use swe_edge_bin::*;

/// @covers: Main
#[test]
fn test_main_trait_is_object_safe() {
    fn _accept(_s: &dyn Main) {}
}
