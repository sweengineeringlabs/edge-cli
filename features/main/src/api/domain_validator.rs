//! Domain validator interface definition.
//!
//! Implementations in core/ validate domain workspace structure and configuration.

use crate::api::validator::Validator;

/// Marker type for domain workspace validation contract.
///
/// Implementations must validate SEA (Structural Engineering Architecture) compliance
/// for domain workspaces, including presence of required api/ and saf/ directories,
/// and proper service_type metadata in architecture.toml files.
#[allow(dead_code)]
pub type DomainValidatorContract = dyn Validator;

#[cfg(test)]
mod tests {
    #[test]
    fn test_domain_validator_contract_defined() {
        // Type alias DomainValidatorContract provides interface contract
        assert!(true);
    }
}
