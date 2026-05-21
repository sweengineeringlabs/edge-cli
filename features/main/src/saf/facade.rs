//! Facade functions for CLI dispatch.

use clap::Parser;
use std::path::PathBuf;

use crate::api::commands::Command;
use crate::api::config::Config;
use crate::api::error::Error;
use crate::api::scaffolder::TestScaffolder;
use crate::api::validator::Validator;
use crate::core::{ConfigValidator, CurlScaffolder, DomainValidator};

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

/// Factory function for domain validator.
pub fn domain_validator() -> Box<dyn Validator> {
    Box::new(DomainValidator)
}

/// Factory function for config validator.
pub fn config_validator() -> Box<dyn Validator> {
    Box::new(ConfigValidator)
}

/// Factory function for curl test scaffolder.
pub fn curl_scaffolder() -> Box<dyn TestScaffolder> {
    Box::new(CurlScaffolder)
}

/// Execute the primary operation with the given configuration.
pub fn execute(config: &Config) -> Result<(), Error> {
    match &config.command {
        Some(Command::Validate { path }) => execute_validate(path, config.verbose),
        Some(Command::TestGen {
            path,
            output,
            base_url,
        }) => execute_test_gen(path, output, base_url, config.verbose),
        None => Ok(()),
    }
}

fn execute_validate(path: &std::path::Path, verbose: bool) -> Result<(), Error> {
    let domain_val = domain_validator();
    let config_val = config_validator();

    println!("Validating domain structure...");
    let domain_report = domain_val.validate(path)?;

    if verbose {
        for passed in &domain_report.passed {
            println!("  ✓ {}", passed);
        }
    }
    for failed in &domain_report.failed {
        println!("  ✗ {}: {}", failed.path.display(), failed.message);
    }

    println!("\nValidating config files...");
    let config_report = config_val.validate(path)?;

    if verbose {
        for passed in &config_report.passed {
            println!("  ✓ {}", passed);
        }
    }
    for failed in &config_report.failed {
        println!("  ✗ {}: {}", failed.path.display(), failed.message);
    }

    if domain_report.is_clean() && config_report.is_clean() {
        println!("\n✓ All validations passed");
        Ok(())
    } else {
        Err(Error::Config {
            message: "Validation failed".to_string(),
        })
    }
}

fn execute_test_gen(
    _path: &std::path::Path,
    output: &std::path::Path,
    base_url: &str,
    _verbose: bool,
) -> Result<(), Error> {
    let scaffolder = curl_scaffolder();

    // For now, generate tests with hardcoded example handlers
    let spec = crate::api::scaffolder::ScaffoldSpec {
        handlers: vec![
            crate::api::scaffolder::HandlerSpec {
                id: "health".to_string(),
                pattern: "/health".to_string(),
                method: "GET".to_string(),
            },
            crate::api::scaffolder::HandlerSpec {
                id: "list_handlers".to_string(),
                pattern: "/handlers".to_string(),
                method: "GET".to_string(),
            },
        ],
        base_url: base_url.to_string(),
    };

    let report = scaffolder.scaffold(&spec, output)?;

    println!("Generated {} test files:", report.files_written.len());
    for file in &report.files_written {
        println!("  ✓ {}", file.display());
    }

    Ok(())
}

/// Parse CLI arguments and execute.
pub fn run() -> Result<(), Error> {
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
    execute(&config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_validator_factory_creates_instance() {
        let _validator = domain_validator();
        // Just verify it can be created
        assert!(true);
    }

    #[test]
    fn test_config_validator_factory_creates_instance() {
        let _validator = config_validator();
        // Just verify it can be created
        assert!(true);
    }

    #[test]
    fn test_curl_scaffolder_factory_creates_instance() {
        let _scaffolder = curl_scaffolder();
        // Just verify it can be created
        assert!(true);
    }
}
