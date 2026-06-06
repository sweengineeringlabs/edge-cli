//! SPI (Service Provider Interface) — extension hooks for downstream consumers.
//!
//! Downstream crates implement
//! [`crate::api::provider::traits::validation_provider::ValidationProvider`]
//! to plug in custom validation, or fall back to
//! [`crate::api::provider::types::noop_validation_provider::NoopValidationProvider`].
//!
//! This crate has no external-library backend wrappers, so `spi/` carries
//! no implementations of its own.
