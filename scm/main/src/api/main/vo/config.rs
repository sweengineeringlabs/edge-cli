//! Configuration type for main.

use crate::api::vo::command::Command;

/// Configuration for main.
#[derive(Debug, Clone)]
pub struct Config {
    /// Enable verbose output.
    pub verbose: bool,
    /// CLI command to execute.
    pub command: Option<Command>,
}

impl Config {
    /// Create a new default configuration.
    pub fn new() -> Self {
        Self {
            verbose: false,
            command: None,
        }
    }

    /// Set verbose mode.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Set command.
    pub fn with_command(mut self, command: Command) -> Self {
        self.command = Some(command);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
