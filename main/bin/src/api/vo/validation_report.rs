//! Validation report type.

use std::path::PathBuf;

use super::validation_failure::ValidationFailure;

/// Result of a validation run.
///
/// Collects all passing and failing checks so operators can resolve all
/// issues in a single pass rather than fix-and-rerun repeatedly.
/// Use [`is_clean`] to decide whether to proceed and the `failed` field
/// to report specific issues with their source paths.
///
/// [`is_clean`]: ValidationReport::is_clean
///
/// # Examples
///
/// ```rust
/// use swe_edge_bin::ValidationReport;
///
/// let mut report = ValidationReport::new();
/// assert!(report.is_clean());
///
/// report.add_passed("handler 'greet' registered correctly".to_string());
/// report.add_failed(
///     std::path::PathBuf::from("src/api/handler.rs"),
///     "Handler::id() returns empty string".to_string(),
/// );
///
/// assert!(!report.is_clean());
/// assert_eq!(report.passed.len(), 1);
/// assert_eq!(report.failed.len(), 1);
/// assert_eq!(report.failed[0].message, "Handler::id() returns empty string");
/// ```
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    /// Messages for checks that passed.
    pub passed: Vec<String>,
    /// Failures with the file path where each was detected.
    pub failed: Vec<ValidationFailure>,
}

impl ValidationReport {
    /// Create an empty report with no checks recorded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_bin::ValidationReport;
    /// let report = ValidationReport::new();
    /// assert!(report.is_clean());
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` when no failures were recorded.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_bin::ValidationReport;
    /// assert!(ValidationReport::new().is_clean());
    /// ```
    pub fn is_clean(&self) -> bool {
        self.failed.is_empty()
    }

    /// Record a passing check message.
    pub fn add_passed(&mut self, msg: String) {
        self.passed.push(msg);
    }

    /// Record a failing check with the source path and a human-readable message.
    pub fn add_failed(&mut self, path: PathBuf, msg: String) {
        self.failed.push(ValidationFailure { path, message: msg });
    }
}
