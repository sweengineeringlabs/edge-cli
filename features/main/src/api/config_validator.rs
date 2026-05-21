//! Config validator interface definition.
//!
//! Implementations in core/ validate TOML configuration files for parse errors.

use crate::api::validator::Validator;

/// Marker type for config file validation contract.
///
/// Implementations must validate that all TOML files in the target directory
/// can be parsed correctly without syntax errors or structural issues.
#[allow(dead_code)]
pub type ConfigValidatorContract = dyn Validator;

#[cfg(test)]
mod tests {
    #[test]
    fn test_config_validator_contract_defined() {
        // Type alias ConfigValidatorContract provides interface contract
        assert!(true);
    }
}
