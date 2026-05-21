//! Main trait definition.
//!
//! Implement this trait in core/ to define main's primary behavior.

use super::config::Config;
use super::error::Error;

/// Primary service trait for main.
pub trait Main: Send + Sync {
    /// Execute the primary operation with the given configuration.
    fn execute(&self, config: &Config) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_trait_is_object_safe() {
        fn _accept(_s: &dyn Main) {}
    }
}
