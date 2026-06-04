//! Error integration tests for the bin crate.

use swe_edge_bin::BinError;

/// @covers: BinError::Io
#[test]
fn test_error_display_io() {
    let err = BinError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
    assert!(err.to_string().contains("I/O error"));
}

/// @covers: BinError::Config
#[test]
fn test_error_display_config() {
    let err = BinError::Config {
        message: "bad value".to_string(),
    };
    assert!(err.to_string().contains("bad value"));
}
