//! BinSvc implementation — CLI dispatch facade.

use clap::Parser;
use std::path::PathBuf;

use crate::api::bin::BinSvc;
use crate::api::error::BinError;
use crate::api::traits::main::Main;
use crate::api::traits::scaffolder::Scaffolder;
use crate::api::traits::validator::Validator;
use crate::api::types::command::Command;
use crate::api::types::config::Config;
use crate::core::{ConfigValidator, CurlScaffolder, DefaultMain, DomainValidator};
use crate::spi::ProviderRegistry;

#[derive(Parser)]
#[command(name = "edge")]
#[command(about = "edge CLI — domain and config validator, curl test scaffolder")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Parser)]
enum Commands {
    /// Validate domain and config structure
    Validate {
        /// Path to validate
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Generate integration tests
    TestGen {
        #[command(subcommand)]
        command: TestGenCommands,
    },
}

#[derive(Parser)]
enum TestGenCommands {
    /// Generate curl integration tests
    Curl {
        /// Path to scan for handlers
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Output directory for generated tests
        #[arg(short, long, default_value = "./tests/curl")]
        output: PathBuf,

        /// Base URL for curl commands
        #[arg(short, long, default_value = "http://localhost:8080")]
        base_url: String,
    },
}

impl BinSvc {
    /// Create a new BinSvc instance.
    pub fn new() -> Self {
        Self
    }

    /// Factory method for domain validator.
    pub fn domain_validator(&self) -> Box<dyn Validator> {
        Box::new(DomainValidator::new())
    }

    /// Factory method for config validator.
    pub fn config_validator(&self) -> Box<dyn Validator> {
        Box::new(ConfigValidator::new())
    }

    /// Factory method for curl test scaffolder.
    pub fn curl_scaffolder(&self) -> Box<dyn Scaffolder> {
        Box::new(CurlScaffolder::new())
    }

    /// Execute the primary operation with the given configuration.
    pub fn execute(&self, config: &Config) -> Result<(), BinError> {
        let main_impl = DefaultMain::new();
        // Use the SPI provider registry for noop validation when no command.
        let _registry = ProviderRegistry::new();
        match &config.command {
            Some(Command::Validate { path }) => self.execute_validate(path, config.verbose),
            Some(Command::TestGen {
                path,
                output,
                base_url,
            }) => self.execute_test_gen(path, output, base_url, config.verbose),
            None => main_impl.execute(config),
        }
    }

    fn execute_validate(&self, path: &std::path::Path, verbose: bool) -> Result<(), BinError> {
        let domain_val = self.domain_validator();
        let config_val = self.config_validator();

        tracing::info!("Validating domain structure...");
        let domain_report = domain_val.validate(path)?;

        if verbose {
            for passed in &domain_report.passed {
                tracing::info!("  pass: {}", passed);
            }
        }
        for failed in &domain_report.failed {
            tracing::warn!("  fail: {}: {}", failed.path.display(), failed.message);
        }

        tracing::info!("Validating config files...");
        let config_report = config_val.validate(path)?;

        if verbose {
            for passed in &config_report.passed {
                tracing::info!("  pass: {}", passed);
            }
        }
        for failed in &config_report.failed {
            tracing::warn!("  fail: {}: {}", failed.path.display(), failed.message);
        }

        if domain_report.is_clean() && config_report.is_clean() {
            tracing::info!("All validations passed");
            Ok(())
        } else {
            Err(BinError::Config {
                message: "Validation failed".to_string(),
            })
        }
    }

    fn execute_test_gen(
        &self,
        _path: &std::path::Path,
        output: &std::path::Path,
        base_url: &str,
        _verbose: bool,
    ) -> Result<(), BinError> {
        let scaffolder = self.curl_scaffolder();

        let spec = crate::api::types::scaffold::scaffold_spec::ScaffoldSpec {
            handlers: vec![
                crate::api::types::scaffold::handler_spec::HandlerSpec {
                    id: "health".to_string(),
                    pattern: "/health".to_string(),
                    method: "GET".to_string(),
                },
                crate::api::types::scaffold::handler_spec::HandlerSpec {
                    id: "list_handlers".to_string(),
                    pattern: "/handlers".to_string(),
                    method: "GET".to_string(),
                },
            ],
            base_url: base_url.to_string(),
        };

        let report = scaffolder.scaffold(&spec, output)?;

        tracing::info!("Generated {} test files:", report.files_written.len());
        for file in &report.files_written {
            tracing::info!("  {}", file.display());
        }

        Ok(())
    }

    /// Parse CLI arguments and execute.
    pub fn run(&self) -> Result<(), BinError> {
        let args = Cli::parse();

        let command = match args.command {
            Commands::Validate { path } => Command::Validate { path },
            Commands::TestGen { command } => match command {
                TestGenCommands::Curl {
                    path,
                    output,
                    base_url,
                } => Command::TestGen {
                    path,
                    output,
                    base_url,
                },
            },
        };

        let config = Config::new()
            .with_verbose(args.verbose)
            .with_command(command);
        self.execute(&config)
    }
}

impl Default for BinSvc {
    fn default() -> Self {
        Self::new()
    }
}
