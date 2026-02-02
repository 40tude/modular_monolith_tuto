//! File Input Adapter.
//!
//! Implements the `NameReader` port for reading names from a file.

use std::fs;
use std::path::PathBuf;

use domain::ports::PortError;
use domain::NameReader;

use crate::error::Error;

/// Adapter for reading names from a file.
///
/// This adapter reads the entire file content and returns it as a single name.
pub struct FileInput {
    path: PathBuf,
}

impl FileInput {
    /// Creates a new `FileInput` adapter.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to read from
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl NameReader for FileInput {
    fn read_name(&self) -> Result<String, PortError> {
        let content = fs::read_to_string(&self.path).map_err(|source| Error::ReadFile {
            path: self.path.clone(),
            source,
        })?;
        Ok(content.trim().to_string())
    }
}
