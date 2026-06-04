//! Configuration TOML validator.

use std::path::Path;

use walkdir::WalkDir;

use crate::api::error::BinError;
use crate::api::traits::validator::Validator;
use crate::api::types::validation::validation_report::ValidationReport;

/// Validates that all TOML config files parse correctly.
pub(crate) struct ConfigValidator;

impl ConfigValidator {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Validator for ConfigValidator {
    fn validate(&self, path: &Path) -> Result<ValidationReport, BinError> {
        let mut report = ValidationReport::new();

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map(|n| n.ends_with(".toml"))
                    .unwrap_or(false)
            })
        {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                match toml::from_str::<toml::Value>(&content) {
                    Ok(_) => {
                        report.add_passed(format!("Valid TOML: {}", entry.path().display()));
                    }
                    Err(e) => {
                        report.add_failed(
                            entry.path().to_path_buf(),
                            format!("Failed to parse TOML: {}", e),
                        );
                    }
                }
            }
        }

        if report.passed.is_empty() && report.failed.is_empty() {
            report.add_passed("No TOML files found to validate".to_string());
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
    fn test_new_creates_config_validator() {
        let _v = ConfigValidator::new();
    }

    #[test]
    fn test_config_validator_accepts_valid_toml() {
        let temp = TempDir::new().unwrap();
        let config_file = temp.path().join("config.toml");

        let content = r#"
[application]
name = "test"
"#;
        fs::write(&config_file, content).unwrap();

        let validator = ConfigValidator::new();
        let report = validator.validate(temp.path()).unwrap();

        assert!(report.failed.is_empty());
        assert!(report.passed.iter().any(|p| p.contains("Valid TOML")));
    }

    #[test]
    fn test_config_validator_rejects_invalid_toml() {
        let temp = TempDir::new().unwrap();
        let config_file = temp.path().join("config.toml");

        let content = r#"
[application
name = "test"
"#;
        fs::write(&config_file, content).unwrap();

        let validator = ConfigValidator::new();
        let report = validator.validate(temp.path()).unwrap();

        assert!(!report.failed.is_empty());
        assert!(report
            .failed
            .iter()
            .any(|f| f.message.contains("Failed to parse TOML")));
    }
}
