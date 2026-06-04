//! CLI command definitions.

use std::path::PathBuf;

/// Top-level CLI command.
///
/// Parsed from `argv` by `BinSvc` before dispatching to the appropriate
/// subcommand handler. `Validate` checks the handler registry for
/// correctness; `TestGen` generates curl test files.
///
/// # Examples
///
/// ```rust
/// use std::path::PathBuf;
/// use swe_edge_bin::Command;
///
/// let cmd = Command::Validate { path: PathBuf::from("config/") };
/// assert!(matches!(cmd, Command::Validate { .. }));
///
/// let cmd = Command::TestGen {
///     path: PathBuf::from("config/"),
///     output: PathBuf::from("tests/curl/"),
///     base_url: "http://localhost:8080".to_string(),
/// };
/// if let Command::TestGen { base_url, .. } = &cmd {
///     assert_eq!(base_url, "http://localhost:8080");
/// }
/// ```
#[derive(Debug, Clone)]
pub enum Command {
    /// Validate the handler registry and config at `path`.
    Validate {
        /// Root directory of the service config.
        path: PathBuf,
    },
    /// Generate curl test files at `output` for the service at `base_url`.
    TestGen {
        /// Root directory of the service config.
        path: PathBuf,
        /// Directory to write generated test files into.
        output: PathBuf,
        /// Base URL of the running service.
        base_url: String,
    },
}
