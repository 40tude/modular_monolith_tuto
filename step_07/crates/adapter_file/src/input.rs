// input.rs

// File Input Adapter.
// Implements the `NameReader` port for reading names from a file.

use crate::errors::FileError;
use domain::{NameReader, NameReaderError};
use std::fs;
use std::path::PathBuf;

// Adapter for reading names from a file.
// This adapter reads the entire file content and returns it as a single name.
pub struct FileInput {
    path: PathBuf,
}

impl FileInput {
    // Creates a new `FileInput` adapter.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl NameReader for FileInput {
    fn read_name(&self) -> Result<String, NameReaderError> {
        let content = fs::read_to_string(&self.path)
            .map_err(|e| NameReaderError::Infrastructure(Box::new(FileError::from(e))))?;
        Ok(content.trim().to_string())
    }
}
