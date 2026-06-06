//! Scaffold report type.

use std::path::PathBuf;

/// Result of scaffold generation.
#[derive(Debug, Clone, Default)]
pub struct ScaffoldReport {
    pub files_written: Vec<PathBuf>,
}

impl ScaffoldReport {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a file to the report.
    pub fn add_file(&mut self, path: PathBuf) {
        self.files_written.push(path);
    }
}
