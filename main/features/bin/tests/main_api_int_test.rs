//! API trait integration tests for main.

use main::*;

/// @covers: Main
#[test]
fn test_main_trait_is_object_safe() {
    fn _accept(_s: &dyn Main) {}
}
