//! Default Main implementation.

use crate::api::config::Config;
use crate::api::error::Error;
use crate::api::main::Main;

/// Default implementation of the Main trait.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub(crate) struct DefaultMain;

impl DefaultMain {
    /// Create a new default instance.
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Main for DefaultMain {
    fn execute(&self, config: &Config) -> Result<(), Error> {
        if config.verbose {
            println!("[main] executing with verbose=true");
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
