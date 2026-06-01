//! BinError — domain error type for the bin crate.

/// Errors that can occur in the bin crate.
#[derive(Debug, thiserror::Error)]
pub enum BinError {
    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// A configuration error occurred.
    #[error("Configuration error: {message}")]
    Config { message: String },
}
