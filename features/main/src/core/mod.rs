//! Core implementation layer (L3).
//!
//! All structs here should use `pub(crate)` visibility.
//! Implement traits from api/ here.

pub(crate) mod config_validator;
pub(crate) mod curl_scaffolder;
pub(crate) mod default_main;
pub(crate) mod domain_validator;

pub(crate) use config_validator::ConfigValidator;
pub(crate) use curl_scaffolder::CurlScaffolder;
pub(crate) use domain_validator::DomainValidator;
