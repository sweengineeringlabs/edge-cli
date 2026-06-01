//! Application configuration builder.

/// Builder for application configuration.
///
/// Maps to `config/application.toml` at runtime.
#[derive(Debug, Clone, Default)]
pub struct ApplicationConfigBuilder {
    name: Option<String>,
    version: Option<String>,
    log_level: Option<String>,
}

impl ApplicationConfigBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the application name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set the application version.
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the log level.
    pub fn with_log_level(mut self, level: String) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Get the configured name.
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("swe-edge-bin")
    }

    /// Get the configured log level.
    pub fn log_level(&self) -> &str {
        self.log_level.as_deref().unwrap_or("info")
    }
}
