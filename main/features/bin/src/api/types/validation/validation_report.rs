//! Validation report type.

use std::path::PathBuf;

use super::validation_failure::ValidationFailure;

/// Result of a validation run.
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    pub passed: Vec<String>,
    pub failed: Vec<ValidationFailure>,
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
