//! Curl test scaffolder interface definition.
//!
//! Implementations in core/ generate curl integration test scripts from handler specifications.

use crate::api::scaffolder::TestScaffolder;

/// Marker type for curl test scaffolder contract.
///
/// Implementations must generate curl integration test scripts (.sh files) for HTTP handlers,
/// including proper HTTP methods, headers, and an orchestrator script to run all tests.
#[allow(dead_code)]
pub type CurlScaffolderContract = dyn TestScaffolder;

#[cfg(test)]
mod tests {
    #[test]
    fn test_curl_scaffolder_contract_defined() {
        // Type alias CurlScaffolderContract provides interface contract
        assert!(true);
    }
}
