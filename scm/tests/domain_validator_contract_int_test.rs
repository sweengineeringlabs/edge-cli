//! Integration tests for DomainValidatorContract.

use swe_edge_bin::{BinSvc, DomainValidatorContract};

/// @covers: DomainValidatorContract
#[test]
fn test_domain_validator_contract_is_dyn_validator() {
    // DomainValidatorContract is a type alias for dyn Validator
    // Verify the factory returns an implementor
    let svc = BinSvc::new();
    let _v: Box<DomainValidatorContract> = svc.domain_validator();
}
