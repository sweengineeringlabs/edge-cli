//! Validator trait and types for domain and config validation.

use std::path::{Path, PathBuf};

use crate::api::error::Error;

/// Validates project structure against domain and config contracts.
pub trait Validator: Send + Sync {
    fn validate(&self, path: &Path) -> Result<ValidationReport, Error>;
}

/// Result of a validation run.
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    pub passed: Vec<String>,
    pub failed: Vec<ValidationFailure>,
}

/// Single validation failure.
#[derive(Debug, Clone)]
pub struct ValidationFailure {
    pub path: PathBuf,
    pub message: String,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_clean(&self) -> bool {
        self.failed.is_empty()
    }

    pub fn add_passed(&mut self, msg: String) {
        self.passed.push(msg);
    }

    pub fn add_failed(&mut self, path: PathBuf, msg: String) {
        self.failed.push(ValidationFailure { path, message: msg });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_report_new_is_clean() {
        let report = ValidationReport::new();
        assert!(report.is_clean());
        assert!(report.passed.is_empty());
        assert!(report.failed.is_empty());
    }

    #[test]
    fn test_validation_report_add_passed() {
        let mut report = ValidationReport::new();
        report.add_passed("Check 1 passed".to_string());
        assert!(report.is_clean());
        assert_eq!(report.passed.len(), 1);
    }

    #[test]
    fn test_validation_report_add_failed() {
        let mut report = ValidationReport::new();
        report.add_failed(PathBuf::from("/test/path"), "Check failed".to_string());
        assert!(!report.is_clean());
        assert_eq!(report.failed.len(), 1);
        assert_eq!(report.failed[0].message, "Check failed");
    }

    #[test]
    fn test_validation_failure_stores_path_and_message() {
        let path = PathBuf::from("/some/path");
        let msg = "Something went wrong".to_string();
        let failure = ValidationFailure {
            path: path.clone(),
            message: msg.clone(),
        };
        assert_eq!(failure.path, path);
        assert_eq!(failure.message, msg);
    }
}
