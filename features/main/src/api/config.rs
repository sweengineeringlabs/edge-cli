//! Configuration type for main.

use super::commands::Command;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new_returns_default() {
        let config = Config::new();
        assert!(!config.verbose);
        assert!(config.command.is_none());
    }

    #[test]
    fn test_config_with_verbose_sets_flag() {
        let config = Config::new().with_verbose(true);
        assert!(config.verbose);
    }
}
