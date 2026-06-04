//! Core implementation layer (L3).
//!
//! All structs here use `pub(crate)` visibility.
//! Implements traits from api/.

pub(crate) mod main;
pub(crate) mod scaffold;
pub(crate) mod validator;

pub(crate) use main::DefaultMain;
pub(crate) use scaffold::CurlScaffolder;
pub(crate) use validator::ConfigValidator;
pub(crate) use validator::DomainValidator;
