// output.rs

// File Output Adapter.
// Implements the `GreetingWriter` port for writing greetings to a file.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::errors::FileError;
use domain::GreetingWriter;
use domain::InfraError;

/// Adapter for writing greetings to a file.
///
/// This adapter appends greetings to a file, creating it if it doesn't exist.
pub struct FileOutput {
    path: PathBuf,
}

impl FileOutput {
    // Creates a new `FileOutput` adapter.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    // Opens the file for appending.
    fn open_file(&self) -> Result<File, FileError> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|source| {
                let err: FileError = FileError::Open {
                    path: self.path.clone(),
                    source,
                }
                .into();
                err
            })
    }
}

impl GreetingWriter for FileOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>> {
        let mut file = self.open_file()?;
        writeln!(file, "{}", greeting).map_err(|source| -> Box<dyn InfraError> {
            Box::new(
                FileError::Write {
                    path: self.path.clone(),
                    source,
                }
                .into(),
            )
        })?;
        Ok(())
    }
}
