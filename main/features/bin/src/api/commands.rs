//! CLI command definitions.

use std::path::PathBuf;

/// Top-level CLI command.
#[derive(Debug, Clone)]
pub enum Command {
    Validate {
        path: PathBuf,
    },
    TestGen {
        path: PathBuf,
        output: PathBuf,
        base_url: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_validate_variant_constructs() {
        let cmd = Command::Validate {
            path: PathBuf::from("/tmp"),
        };
        match cmd {
            Command::Validate { path } => assert_eq!(path, PathBuf::from("/tmp")),
            _ => panic!("Expected Validate variant"),
        }
    }

    #[test]
    fn test_command_testgen_variant_constructs() {
        let cmd = Command::TestGen {
            path: PathBuf::from("/src"),
            output: PathBuf::from("/out"),
            base_url: "http://localhost:8080".to_string(),
        };
        match cmd {
            Command::TestGen {
                path,
                output,
                base_url,
            } => {
                assert_eq!(path, PathBuf::from("/src"));
                assert_eq!(output, PathBuf::from("/out"));
                assert_eq!(base_url, "http://localhost:8080");
            }
            _ => panic!("Expected TestGen variant"),
        }
    }

    #[test]
    fn test_command_derives_debug_clone() {
        let cmd = Command::Validate {
            path: PathBuf::from("."),
        };
        let cloned = cmd.clone();
        assert_eq!(format!("{:?}", cmd), format!("{:?}", cloned));
    }
}
