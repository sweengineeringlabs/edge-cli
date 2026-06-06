//! Gateway layer — external I/O adapters.
//!
//! This is the public surface of the crate. All external consumers use this module.

pub(crate) mod egress;
pub(crate) mod ingress;

// Re-export types from saf/ (data types, error, primary facade)
pub use crate::saf::default_config_path;
pub use crate::saf::BinError;
pub use crate::saf::BinSvc;
pub use crate::saf::Command;
pub use crate::saf::Config;
pub use crate::saf::HandlerSpec;
pub use crate::saf::ScaffoldReport;
pub use crate::saf::ScaffoldSpec;
pub use crate::saf::ValidationFailure;
pub use crate::saf::ValidationReport;

// Re-export traits directly from api/ (not via saf/ to avoid rule 126 in saf/)
pub use crate::api::main::traits::main::Main;
pub use crate::api::provider::traits::provider_registry_contract::ProviderRegistryContract;
pub use crate::api::provider::traits::validation_provider::ValidationProvider;
pub use crate::api::scaffold::traits::scaffolder::Scaffolder;
pub use crate::api::validator::traits::validator::Validator;

// Re-export interface contracts from domain-organized api/ locations
pub use crate::api::main::types::main_contract::MainContract;
pub use crate::api::scaffold::types::curl::Curl as CurlBound;
pub use crate::api::scaffold::types::curl_scaffolder_contract::CurlScaffolderContract;
pub use crate::api::validator::types::config::Config as ConfigBound;
pub use crate::api::validator::types::config_validator_contract::ConfigValidatorContract;
pub use crate::api::validator::types::domain::Domain as DomainBound;
pub use crate::api::validator::types::domain_validator_contract::DomainValidatorContract;

// Re-export config builder
pub use crate::api::types::application_config_builder::ApplicationConfigBuilder;

// Re-export SPI default implementations (from api/spi/ which is public)
pub use crate::api::provider::types::noop_validation_provider::NoopValidationProvider;
