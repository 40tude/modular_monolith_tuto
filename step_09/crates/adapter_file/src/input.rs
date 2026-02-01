//! File Input Adapter
//!
//! Implements the `NameReader` port for reading names from a file.

use std::fs;
use std::path::PathBuf;

use domain::NameReader;

/// Adapter for reading names from a file.
///
/// This adapter reads the entire file content and returns it as a single name.
/// For reading multiple names, consider using `FileLineInput`.
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
    fn read_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&self.path)?;
        let name = content.trim().to_string();
        Ok(name)
    }
}
