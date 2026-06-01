# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in swe-edge-bin, please email security@sweengineeringlabs.io with details of the vulnerability.

Please do **not** open a public issue for security vulnerabilities.

## Supported Versions

The latest version of swe-edge-bin is supported. Security updates will be released as needed.

## Dependencies

This crate uses:
- `clap` for CLI argument parsing
- `tracing` for structured logging
- `toml` for TOML parsing
- `serde` for serialization
- `walkdir` for directory traversal
- `thiserror` for error handling

All dependencies are regularly updated and audited for vulnerabilities.
