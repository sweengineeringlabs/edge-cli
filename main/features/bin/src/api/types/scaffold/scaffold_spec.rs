//! Scaffold specification type.

use super::handler_spec::HandlerSpec;

/// Specification for scaffold generation.
#[derive(Debug, Clone)]
pub struct ScaffoldSpec {
    pub handlers: Vec<HandlerSpec>,
    pub base_url: String,
}
