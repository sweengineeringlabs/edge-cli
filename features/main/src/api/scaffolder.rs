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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaffold_report_new_is_empty() {
        let report = ScaffoldReport::new();
        assert!(report.files_written.is_empty());
    }

    #[test]
    fn test_scaffold_report_add_file() {
        let mut report = ScaffoldReport::new();
        let path = PathBuf::from("/tmp/test.sh");
        report.add_file(path.clone());
        assert_eq!(report.files_written.len(), 1);
        assert_eq!(report.files_written[0], path);
    }

    #[test]
    fn test_handler_spec_constructs() {
        let spec = HandlerSpec {
            id: "list_items".to_string(),
            pattern: "/items".to_string(),
            method: "GET".to_string(),
        };
        assert_eq!(spec.id, "list_items");
        assert_eq!(spec.pattern, "/items");
        assert_eq!(spec.method, "GET");
    }

    #[test]
    fn test_scaffold_spec_constructs() {
        let spec = ScaffoldSpec {
            handlers: vec![HandlerSpec {
                id: "test".to_string(),
                pattern: "/test".to_string(),
                method: "POST".to_string(),
            }],
            base_url: "http://localhost:8080".to_string(),
        };
        assert_eq!(spec.handlers.len(), 1);
        assert_eq!(spec.base_url, "http://localhost:8080");
    }
}
