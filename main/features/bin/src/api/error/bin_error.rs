//! BinError — domain error type for the bin crate.

/// Errors that can occur in the bin crate.
///
/// `Io` wraps OS-level file errors (e.g. missing config directory).
/// `Config` carries a descriptive message for semantic configuration
/// errors (e.g. conflicting handler IDs).
///
/// # Examples
///
/// ```rust
/// use swe_edge_bin::BinError;
///
/// let err = BinError::Config { message: "handler id 'greet' registered twice".to_string() };
/// assert!(err.to_string().contains("Configuration error"));
/// assert!(err.to_string().contains("registered twice"));
/// ```
#[derive(Debug, thiserror::Error)]
pub enum BinError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// A configuration error occurred.
    #[error("Configuration error: {message}")]
    Config { message: String },
}
