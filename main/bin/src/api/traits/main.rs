//! Main trait definition.
//!
//! Implement this trait in core/ to define main's primary behavior.

use crate::api::error::BinError;
use crate::api::types::config::Config;

/// Primary service trait for main.
pub trait Main: Send + Sync {
    /// Execute the primary operation with the given configuration.
    fn execute(&self, config: &Config) -> Result<(), BinError>;
}
