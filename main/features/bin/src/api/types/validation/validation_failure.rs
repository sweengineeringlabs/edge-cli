//! Validation failure type.

use std::path::PathBuf;

/// Single validation failure.
#[derive(Debug, Clone)]
pub struct ValidationFailure {
    pub path: PathBuf,
    pub message: String,
}
