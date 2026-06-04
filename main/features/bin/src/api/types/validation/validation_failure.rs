//! Validation failure type.

use std::path::PathBuf;

/// Single validation failure.
///
/// Collected into [`ValidationReport::failed`] by `add_failed`. `path` identifies
/// the source file where the failure was detected; `message` explains the
/// violated constraint.
///
/// [`ValidationReport::failed`]: crate::ValidationReport::failed
///
/// # Examples
///
/// ```rust
/// use std::path::PathBuf;
/// use swe_edge_bin::ValidationFailure;
///
/// let failure = ValidationFailure {
///     path: PathBuf::from("src/api/handler.rs"),
///     message: "Handler::id() must not be empty".to_string(),
/// };
/// assert_eq!(failure.path.file_name().unwrap(), "handler.rs");
/// assert!(failure.message.contains("id()"));
/// ```
#[derive(Debug, Clone)]
pub struct ValidationFailure {
    /// File where the failure was detected.
    pub path: PathBuf,
    /// Human-readable description of the violated constraint.
    pub message: String,
}
