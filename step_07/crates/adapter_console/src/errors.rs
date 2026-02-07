// error.rs

// Adapter-specific error types implementing the domain's InfraError trait

use domain::InfraError;
use std::any::Any;
use thiserror::Error;

// ConsoleError is needed because of the orphan rule. We can't implement
// InfraError for std::io::Error in the adapter crate. We own neither the trait
// (it's in domain) nor the type (it's in std). ConsoleError is the type that
// makes impl InfraError for ConsoleError legal.
//
// A future adapter_file would need its own FileError for the same reason.
//
// So ConsoleError with a single Io variant is structurally necessary. It's the
// minimum cost of the orphan rule.
#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error("Console I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl InfraError for ConsoleError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// 1. e is a std::io::Error
// 2. into_infra calls e.into(), this triggers the #[from] conversion
// 3. #[from] generates impl From<std::io::Error> for ConsoleError, which builds
//    ConsoleError::Io(e)
// 4. That ConsoleError::Io(e) gets boxed as Box<dyn InfraError> So
//    ConsoleError::Io is constructed every time an I/O error occurs. We
//    don't see it explicitly in the code because into() does it for us.
pub(crate) fn into_infra(e: impl Into<ConsoleError>) -> Box<dyn InfraError> {
    Box::new(e.into())
}
