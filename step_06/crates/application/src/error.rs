//! Error types for the application crate.

/// Errors produced by the application layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A domain rule was violated.
    #[error(transparent)]
    Domain(#[from] domain::Error),

    /// An adapter (port implementation) returned an error.
    #[error("adapter error: {0}")]
    Adapter(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Application-specific result alias.
pub type Result<T> = std::result::Result<T, Error>;

