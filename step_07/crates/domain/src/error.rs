//! Domain error types.

/// Errors produced by domain business rules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The provided name is empty.
    #[error("name cannot be empty")]
    EmptyName,
}

/// Domain-specific result alias.
pub type Result<T> = std::result::Result<T, Error>;

