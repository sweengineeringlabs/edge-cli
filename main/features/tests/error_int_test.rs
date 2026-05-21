//! Error integration tests for main.

use main::*;

#[test]
fn test_error_display_io() {
    let err = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
    assert!(err.to_string().contains("I/O error"));
}

#[test]
fn test_error_display_config() {
    let err = Error::Config {
        message: "bad value".to_string(),
    };
    assert!(err.to_string().contains("bad value"));
}
