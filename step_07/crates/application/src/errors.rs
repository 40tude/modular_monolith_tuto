// error.rs

use domain::InfraError;
use domain::errors::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Infrastructure error: {0}")]
    Infrastructure(Box<dyn InfraError>),
}

// Type alias.
pub type Result<T> = std::result::Result<T, ApplicationError>;

impl From<Box<dyn InfraError>> for ApplicationError {
    fn from(e: Box<dyn InfraError>) -> Self {
        Self::Infrastructure(e)
    }
}
