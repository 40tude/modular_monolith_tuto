//! Error types for the domain crate.

/// Common error type alias using dynamic dispatch.
pub type Error = Box<dyn std::error::Error>;

/// Common result type alias.
pub type Result<T> = std::result::Result<T, Error>;
