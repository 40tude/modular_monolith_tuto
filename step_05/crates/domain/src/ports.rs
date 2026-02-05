// ports.rs

use crate::errors::DomainError;
use std::any::Any;

// Trait for infrastructure errors - implemented by adapters
// This trait defines the contract for infrastructure errors without
// coupling the domain to specific infrastructure implementations.
// Adapters implement this trait for their specific error types.
pub trait InfraError: std::error::Error + Send + Sync + 'static {
    /// Returns self as Any for downcasting to concrete error types
    fn as_any(&self) -> &dyn Any;
}

// Errors that can occur when retrieving names
// Combines domain AND infrastructure errors for input operations
#[derive(Debug)]
pub enum NameReaderError {
    Domain(DomainError),
    Infrastructure(Box<dyn InfraError>),
}

impl std::fmt::Display for NameReaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Domain(e) => write!(f, "Domain error: {e}"),
            Self::Infrastructure(e) => write!(f, "Infrastructure error: {e}"),
        }
    }
}

impl std::error::Error for NameReaderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Domain(e) => Some(e),
            Self::Infrastructure(e) => Some(e.as_ref()),
        }
    }
}

impl From<DomainError> for NameReaderError {
    fn from(e: DomainError) -> Self {
        Self::Domain(e)
    }
}

// Port for reading a name
pub trait NameReader {
    fn read_name(&self) -> Result<String, NameReaderError>;
}

// Port for writing a greeting to an output destination.
pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>>;
}
