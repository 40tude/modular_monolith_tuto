// errors.rs

use domain::InfraError;
use std::any::Any;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl InfraError for FileError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
