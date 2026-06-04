//! Scaffold specification type.

use super::handler_spec::HandlerSpec;

/// Specification for scaffold generation.
///
/// Describes the handlers and base URL for which `BinSvc` will generate
/// curl test files and other artifacts. Built from the validated handler
/// registry before passing to registered scaffolders.
///
/// # Examples
///
/// ```rust
/// use swe_edge_bin::{HandlerSpec, ScaffoldSpec};
///
/// let spec = ScaffoldSpec {
///     handlers: vec![
///         HandlerSpec { id: "greet".to_string(), pattern: "/api/v1/greet".to_string(), method: "POST".to_string() },
///     ],
///     base_url: "http://localhost:8080".to_string(),
/// };
///
/// assert_eq!(spec.handlers.len(), 1);
/// assert_eq!(spec.base_url, "http://localhost:8080");
/// ```
#[derive(Debug, Clone)]
pub struct ScaffoldSpec {
    /// Handlers to generate scaffolding for.
    pub handlers: Vec<HandlerSpec>,
    /// Base URL of the running service (e.g. `"http://localhost:8080"`).
    pub base_url: String,
}
