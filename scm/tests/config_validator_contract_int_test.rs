//! Integration tests for ConfigValidatorContract.

use swe_edge_bin::{BinSvc, ConfigValidatorContract};

/// @covers: ConfigValidatorContract
#[test]
fn test_config_validator_contract_is_dyn_validator() {
    // ConfigValidatorContract is a type alias for dyn Validator
    // Verify the factory returns an implementor
    let svc = BinSvc::new();
    let _v: Box<ConfigValidatorContract> = svc.config_validator();
}
