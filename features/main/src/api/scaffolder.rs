//! Test scaffolder trait and types for curl integration test generation.

use std::path::PathBuf;

use crate::api::error::Error;

/// Generates integration test files from a specification.
pub trait TestScaffolder: Send + Sync {
    fn scaffold(
        &self,
        spec: &ScaffoldSpec,
        output: &std::path::Path,
    ) -> Result<ScaffoldReport, Error>;
}

/// Specification for scaffold generation.
#[derive(Debug, Clone)]
pub struct ScaffoldSpec {
    pub handlers: Vec<HandlerSpec>,
    pub base_url: String,
}

/// Handler specification for curl test generation.
#[derive(Debug, Clone)]
pub struct HandlerSpec {
    pub id: String,
    pub pattern: String,
    pub method: String,
}

/// Result of scaffold generation.
#[derive(Debug, Clone, Default)]
pub struct ScaffoldReport {
    pub files_written: Vec<PathBuf>,
}

impl ScaffoldReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, path: PathBuf) {
        self.files_written.push(path);
    }
}
