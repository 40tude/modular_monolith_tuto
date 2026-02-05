// output.rs

use crate::errors::FileError;
use domain::{GreetingWriter, InfraError};
use std::fs;
use std::path::PathBuf;

pub struct FileOutput {
    path: PathBuf,
}

impl FileOutput {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl GreetingWriter for FileOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>> {
        fs::write(&self.path, greeting)
            .map_err(|e| Box::new(FileError::from(e)) as Box<dyn InfraError>)
    }
}
