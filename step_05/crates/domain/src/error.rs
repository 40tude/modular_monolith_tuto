// error.rs

//! Custom Error variants for the domain crate.

/// Errors produced by domain business rules.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The provided name is empty.
    #[error("Name cannot be empty")]
    EmptyName,
}

/// Used for business errors (EmptyName) in domain logic
pub type Result<T> = std::result::Result<T, Error>;
