//! Default Main implementation.

use crate::api::error::BinError;
use crate::api::traits::main::Main;
use crate::api::types::config::Config;

/// Default implementation of the Main trait.
#[derive(Debug, Default)]
pub(crate) struct DefaultMain;

impl DefaultMain {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Main for DefaultMain {
    fn execute(&self, config: &Config) -> Result<(), BinError> {
        if config.verbose {
            tracing::info!("executing with verbose=true");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_default_main() {
        let _svc = DefaultMain::new();
    }

    #[test]
    fn test_execute_succeeds_with_default_config() {
        let svc = DefaultMain::new();
        let config = Config::default();
        assert!(svc.execute(&config).is_ok());
    }

    #[test]
    fn test_execute_succeeds_in_verbose_mode() {
        let svc = DefaultMain::new();
        let config = Config::default().with_verbose(true);
        assert!(svc.execute(&config).is_ok());
    }
}
