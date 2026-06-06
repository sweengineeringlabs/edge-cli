//! Core implementation layer.
//!
//! All structs here use `pub(crate)` visibility.
//! Implements traits from api/.

pub(crate) mod main;
pub(crate) mod provider;
pub(crate) mod scaffold;
pub(crate) mod validator;

pub(crate) use main::DefaultMain;
pub(crate) use provider::ProviderRegistry;
pub(crate) use scaffold::CurlScaffolder;
pub(crate) use validator::ConfigValidator;
pub(crate) use validator::DomainValidator;
