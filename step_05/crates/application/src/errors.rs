// error.rs

// Errors produced by the application layer.

use domain::errors::{DomainError, InfraError};
use domain::ports::NameReaderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Infrastructure error: {0}")]
    Infrastructure(Box<dyn InfraError>),
}

impl From<Box<dyn InfraError>> for ApplicationError {
    fn from(e: Box<dyn InfraError>) -> Self {
        Self::Infrastructure(e)
    }
}

impl From<NameReaderError> for ApplicationError {
    fn from(e: NameReaderError) -> Self {
        match e {
            NameReaderError::Domain(d) => Self::Domain(d),
            NameReaderError::Infrastructure(i) => Self::Infrastructure(i),
        }
    }
}