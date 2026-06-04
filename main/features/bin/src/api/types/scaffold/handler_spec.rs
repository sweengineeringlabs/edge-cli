//! Handler specification type.

/// Handler specification for curl test generation.
///
/// Each registered `Handler` contributes one `HandlerSpec` to the
/// `ScaffoldSpec`. `id` is the handler's stable key; `pattern` is the
/// URL path; `method` is the HTTP method in uppercase.
///
/// # Examples
///
/// ```rust
/// use swe_edge_bin::HandlerSpec;
///
/// let spec = HandlerSpec {
///     id: "create-order".to_string(),
///     pattern: "/api/v1/orders".to_string(),
///     method: "POST".to_string(),
/// };
///
/// assert_eq!(spec.id, "create-order");
/// assert!(spec.pattern.starts_with('/'));
/// assert_eq!(spec.method, "POST");
/// ```
#[derive(Debug, Clone)]
pub struct HandlerSpec {
    /// Stable handler identifier, used as the file basename for generated tests.
    pub id: String,
    /// URL path pattern (e.g. `"/api/v1/orders"`).
    pub pattern: String,
    /// HTTP method in uppercase (e.g. `"GET"`, `"POST"`).
    pub method: String,
}
