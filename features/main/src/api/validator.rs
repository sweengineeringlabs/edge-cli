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
