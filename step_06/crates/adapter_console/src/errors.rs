// error.rs

// Adapter-specific error types implementing the domain's InfraError trait

use domain::InfraError;
use std::any::Any;
use thiserror::Error;

// Console-related infrastructure errors
#[derive(Error, Debug)]
pub enum ConsoleError {
    #[error("Console output error: {0}")]
    Output(String),

    #[error("Console I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl InfraError for ConsoleError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
