//! Integration tests for BinError.

use swe_edge_bin::BinError;

/// @covers: BinError::Io
#[test]
fn test_bin_error_io_display() {
    let err = BinError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "missing"));
    assert!(err.to_string().contains("I/O error"));
}

/// @covers: BinError::Config
#[test]
fn test_bin_error_config_display() {
    let err = BinError::Config {
        message: "bad value".to_string(),
    };
    assert!(err.to_string().contains("bad value"));
}
