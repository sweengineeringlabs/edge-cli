//! Handler specification type.

/// Handler specification for curl test generation.
#[derive(Debug, Clone)]
pub struct HandlerSpec {
    pub id: String,
    pub pattern: String,
    pub method: String,
}
