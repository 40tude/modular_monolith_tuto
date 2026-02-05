// error.rs

use domain::InfraError;
use std::any::Any;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    /// Failed to read an input file.
    #[error("failed to read input file '{}': {source}", path.display())]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },

    /// Failed to open an output file.
    #[error("failed to open output file '{}': {source}", path.display())]
    Open {
        path: PathBuf,
        source: std::io::Error,
    },

    /// Failed to write to an output file.
    #[error("failed to write to output file '{}': {source}", path.display())]
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl InfraError for FileError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
