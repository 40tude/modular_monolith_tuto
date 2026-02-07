// output.rs

use crate::errors::into_infra;
use domain::{GreetingWriter, InfraError};
use std::path::PathBuf;

pub struct FileOutput {
    path: PathBuf,
}

impl FileOutput {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let _ = std::fs::remove_file(&path);
        Self { path }
    }
}

impl GreetingWriter for FileOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>> {
        std::fs::write(&self.path, format!("{greeting}\n")).map_err(into_infra)?;
        //.map_err(|e| Box::new(FileError::from(e)) as Box<dyn InfraError>)
        Ok(())
    }
}
