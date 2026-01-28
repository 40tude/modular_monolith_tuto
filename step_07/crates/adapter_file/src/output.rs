/// File Output Adapter
///
/// Implements the `GreetingWriter` port for writing greetings to a file.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use domain::GreetingWriter;

/// Adapter for writing greetings to a file.
///
/// This adapter appends greetings to a file, creating it if it doesn't exist.
pub struct FileOutput {
    path: PathBuf,
}

impl FileOutput {
    /// Creates a new `FileOutput` adapter.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to write to
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Opens the file for appending.
    fn open_file(&self) -> Result<File, std::io::Error> {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
    }
}

impl GreetingWriter for FileOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = self.open_file()?;
        writeln!(file, "{}", greeting)?;
        Ok(())
    }
}
