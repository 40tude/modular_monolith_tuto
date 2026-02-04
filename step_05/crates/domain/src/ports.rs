// ports.rs

use crate::errors::{DomainError, InfraError};

// Port for reading a name
pub trait NameReader {
    fn read_name(&self) -> Result<String, NameReaderError>;
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

// Port for writing a greeting to an output destination.
pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>>;
}
