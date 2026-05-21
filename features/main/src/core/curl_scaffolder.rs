//! Curl integration test scaffolder.

use std::fs;
use std::path::Path;

use crate::api::error::Error;
use crate::api::scaffolder::{ScaffoldReport, ScaffoldSpec, TestScaffolder};

/// Generates curl integration test scripts from handler specifications.
pub struct CurlScaffolder;

impl TestScaffolder for CurlScaffolder {
    fn scaffold(&self, spec: &ScaffoldSpec, output: &Path) -> Result<ScaffoldReport, Error> {
        let mut report = ScaffoldReport::new();

        // Create output directory if it doesn't exist
        fs::create_dir_all(output).map_err(|e| Error::Config {
            message: format!("Failed to create output directory: {}", e),
        })?;

        // Generate one script per handler
        for handler in &spec.handlers {
            let script_name = format!("{}.sh", handler.id.replace('/', "_"));
            let script_path = output.join(&script_name);

            let script_content = format!(
                "#!/bin/bash\nset -e\n\n# Generated curl test for handler: {}\n{}\n\necho \"✓ Test for {} passed\"\n",
                handler.id,
                generate_curl_command(&spec.base_url, handler),
                handler.id
            );

            fs::write(&script_path, script_content).map_err(|e| Error::Config {
                message: format!("Failed to write script {}: {}", script_name, e),
            })?;

            report.add_file(script_path);
        }

        // Generate orchestrator script
        let orchestrator = output.join("run-all-tests.sh");
        let mut orchestrator_content =
            "#!/bin/bash\nset -e\n\necho \"Running all curl integration tests...\"\n\n".to_string();

        for handler in &spec.handlers {
            let script_name = format!("{}.sh", handler.id.replace('/', "_"));
            orchestrator_content.push_str(&format!("bash {}\n", script_name));
        }

        orchestrator_content.push_str("\necho \"✓ All tests passed\"\n");

        fs::write(&orchestrator, &orchestrator_content).map_err(|e| Error::Config {
            message: format!("Failed to write orchestrator: {}", e),
        })?;

        report.add_file(orchestrator);

        Ok(report)
    }
}

fn generate_curl_command(base_url: &str, handler: &crate::api::scaffolder::HandlerSpec) -> String {
    let url = format!("{}{}", base_url, handler.pattern);
    match handler.method.to_uppercase().as_str() {
        "GET" => format!("curl -X GET '{}' -H 'Accept: application/json'", url),
        "POST" => format!(
            "curl -X POST '{}' -H 'Content-Type: application/json' -d '{{}}'",
            url
        ),
        "PUT" => format!(
            "curl -X PUT '{}' -H 'Content-Type: application/json' -d '{{}}'",
            url
        ),
        "DELETE" => format!("curl -X DELETE '{}'", url),
        _ => format!("curl -X {} '{}'", handler.method.to_uppercase(), url),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_curl_scaffolder_generates_handler_scripts() {
        let temp = TempDir::new().unwrap();
        let spec = ScaffoldSpec {
            handlers: vec![crate::api::scaffolder::HandlerSpec {
                id: "list_items".to_string(),
                pattern: "/items".to_string(),
                method: "GET".to_string(),
            }],
            base_url: "http://localhost:8080".to_string(),
        };

        let scaffolder = CurlScaffolder;
        let report = scaffolder.scaffold(&spec, temp.path()).unwrap();

        assert_eq!(report.files_written.len(), 2); // handler script + orchestrator
        assert!(temp.path().join("list_items.sh").exists());
    }

    #[test]
    fn test_curl_scaffolder_generates_correct_get_command() {
        let cmd = generate_curl_command(
            "http://localhost:8080",
            &crate::api::scaffolder::HandlerSpec {
                id: "get_user".to_string(),
                pattern: "/users/{id}".to_string(),
                method: "GET".to_string(),
            },
        );

        assert!(cmd.contains("curl -X GET"));
        assert!(cmd.contains("http://localhost:8080/users/{id}"));
    }

    #[test]
    fn test_curl_scaffolder_generates_correct_post_command() {
        let cmd = generate_curl_command(
            "http://localhost:8080",
            &crate::api::scaffolder::HandlerSpec {
                id: "create_item".to_string(),
                pattern: "/items".to_string(),
                method: "POST".to_string(),
            },
        );

        assert!(cmd.contains("curl -X POST"));
        assert!(cmd.contains("-H 'Content-Type: application/json'"));
    }
}
