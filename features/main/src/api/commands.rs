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
