// error.rs

use thiserror::Error;

// Domain errors represent business rule violations
#[derive(Error, Debug)]
pub enum DomainError {
    /// The provided name is empty.
    #[error("Name cannot be empty")]
    EmptyName,
}

// Type alias.
pub type Result<T> = std::result::Result<T, DomainError>;
