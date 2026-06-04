//! SPI (Service Provider Interface) — extension hooks for downstream consumers.
//!
//! Downstream crates implement [`crate::api::traits::validation_provider::ValidationProvider`]
//! to plug in custom validation.

pub(crate) mod provider;

pub(crate) use provider::ProviderRegistry;
