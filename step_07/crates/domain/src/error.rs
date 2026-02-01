//! Domain error types.
//!
//! These errors represent business rule violations specific to the domain.

use std::fmt;

/// Domain errors representing business rule violations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GreetingError {
    /// Name cannot be empty.
    EmptyName,
    /// Name exceeds maximum allowed length.
    NameTooLong { len: usize, max: usize },
}

impl fmt::Display for GreetingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "Name cannot be empty"),
            Self::NameTooLong { len, max } => {
                write!(f, "Name too long: {len} characters (max {max})")
            }
        }
    }
}

impl std::error::Error for GreetingError {}
