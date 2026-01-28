//! Application-level errors.
//!
//! These errors represent problems at the application/orchestration layer.

use std::fmt;

use domain::GreetingError;

/// Application layer errors.
#[derive(Debug)]
pub enum ApplicationError {
    /// Failed to read input from adapter.
    InputError(String),
    /// Failed to write output to adapter.
    OutputError(String),
    /// Domain greeting error.
    GreetingError(GreetingError),
    /// Generic application error.
    Other(String),
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputError(msg) => write!(f, "Failed to read input: {msg}"),
            Self::OutputError(msg) => write!(f, "Failed to write output: {msg}"),
            Self::GreetingError(e) => write!(f, "Greeting failed: {e}"),
            Self::Other(msg) => write!(f, "Application error: {msg}"),
        }
    }
}

impl std::error::Error for ApplicationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::GreetingError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<GreetingError> for ApplicationError {
    fn from(err: GreetingError) -> Self {
        Self::GreetingError(err)
    }
}
