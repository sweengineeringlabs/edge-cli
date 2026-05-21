//! Domain workspace structure validator.

use std::path::Path;

use walkdir::WalkDir;

use crate::api::error::Error;
use crate::api::validator::{ValidationReport, Validator};

/// Validates domain workspace structure against SEA requirements.
pub struct DomainValidator;

impl Validator for DomainValidator {
    fn validate(&self, path: &Path) -> Result<ValidationReport, Error> {
        let mut report = ValidationReport::new();

        // Walk the path looking for architecture.toml files
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == "architecture.toml")
        {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                match toml::from_str::<toml::Value>(&content) {
                    Ok(toml_val) => {
                        // Check if this is a handler workspace
                        if let Some(service_type) = toml_val.get("package").and_then(|p| {
                            p.get("metadata").and_then(|m| {
                                m.get("struct-engine").and_then(|s| s.get("service_type"))
                            })
                        }) {
                            if service_type.as_str() == Some("handler") {
                                let workspace_dir = entry.path().parent().unwrap_or(path);

                                // Check for api/ directory
                                let api_dir = workspace_dir.join("src").join("api");
                                if api_dir.exists() && api_dir.is_dir() {
                                    report.add_passed(format!(
                                        "Found api/ in {}",
                                        workspace_dir.display()
                                    ));
                                } else {
                                    report.add_failed(
                                        api_dir.clone(),
                                        "Missing src/api/ directory".to_string(),
                                    );
                                }

                                // Check for saf/ directory
                                let saf_dir = workspace_dir.join("src").join("saf");
                                if saf_dir.exists() && saf_dir.is_dir() {
                                    report.add_passed(format!(
                                        "Found saf/ in {}",
                                        workspace_dir.display()
                                    ));
                                } else {
                                    report.add_failed(
                                        saf_dir.clone(),
                                        "Missing src/saf/ directory".to_string(),
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        report.add_failed(
                            entry.path().to_path_buf(),
                            format!("Failed to parse architecture.toml: {}", e),
                        );
                    }
                }
            }
        }

        if report.passed.is_empty() && report.failed.is_empty() {
            report.add_passed("No handler workspaces found to validate".to_string());
        }

        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_domain_validator_finds_handler_architecture() {
        let temp = TempDir::new().unwrap();
        let workspace = temp.path().join("test-workspace");
        fs::create_dir_all(&workspace).unwrap();

        // Create architecture.toml with handler service type
        let arch_content = r#"
[package.metadata.struct-engine]
service_type = "handler"
"#;
        fs::write(workspace.join("architecture.toml"), arch_content).unwrap();

        // Create api/ and saf/ directories
        fs::create_dir_all(workspace.join("src").join("api")).unwrap();
        fs::create_dir_all(workspace.join("src").join("saf")).unwrap();

        let validator = DomainValidator;
        let report = validator.validate(temp.path()).unwrap();

        assert!(!report.failed.is_empty() || !report.passed.is_empty());
        assert!(report
            .passed
            .iter()
            .any(|p| p.contains("api/") || p.contains("saf/")));
    }

    #[test]
    fn test_domain_validator_reports_missing_api_directory() {
        let temp = TempDir::new().unwrap();
        let workspace = temp.path().join("test-workspace");
        fs::create_dir_all(&workspace).unwrap();

        let arch_content = r#"
[package.metadata.struct-engine]
service_type = "handler"
"#;
        fs::write(workspace.join("architecture.toml"), arch_content).unwrap();
        fs::create_dir_all(workspace.join("src").join("saf")).unwrap();

        let validator = DomainValidator;
        let report = validator.validate(temp.path()).unwrap();

        assert!(report.failed.iter().any(|f| f.message.contains("api/")));
    }
}
