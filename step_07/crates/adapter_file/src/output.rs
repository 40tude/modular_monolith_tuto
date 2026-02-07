// output.rs

use crate::errors::into_infra;
use domain::{GreetingWriter, InfraError};
use std::io::Write;
use std::path::PathBuf;

/// Writes greetings to a file, appending each on its own line.
///
/// The output file is cleared on construction so each run starts fresh.
pub struct FileOutput {
    path: PathBuf,
}

impl FileOutput {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        // Clear previous output so each run starts fresh
        let _ = std::fs::remove_file(&path);
        Self { path }
    }
}

impl GreetingWriter for FileOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(into_infra)?;
        writeln!(file, "{greeting}").map_err(into_infra)?;
        Ok(())
    }
}
// Rust guideline compliant 2025-05-07
