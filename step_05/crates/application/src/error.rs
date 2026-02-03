// error.rs

//! Error types for the application crate.

/// Errors produced by the application layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A domain rule was violated.
    #[error(transparent)]
    Domain(#[from] domain::Error),

    /// An adapter (port implementation) returned an error.
    #[error(transparent)]
    // The Box<dyn std::error::Error + Send + Sync> is necessary because adapter errors
    // can have different types (console error, database error, etc.). Since Rust does not
    // allow multiple types in the same enum variant, we use an object trait.
    Adapter(Box<dyn std::error::Error + Send + Sync>),
}

/// Application-specific result alias.
pub type Result<T> = std::result::Result<T, Error>;
