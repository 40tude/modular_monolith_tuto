// input.rs

use crate::errors::into_infra;
use domain::{InfraError, NameReader};
use std::path::PathBuf;

/// Reads names from a text file, one per line.
///
/// The file is loaded lazily on the first call to `read_name`.
/// Once all lines have been returned, subsequent calls yield `"quit"`
/// so that `run_greeting_loop` terminates naturally.
#[derive(Debug)]
pub struct FileInput {
    path: PathBuf,
    names: Option<Vec<String>>,
    index: usize,
}

impl FileInput {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            names: None,
            index: 0,
        }
    }

    /// Loads the file contents into `self.names`, splitting by lines
    /// and filtering out empty lines.
    fn load_names(&mut self) -> Result<(), Box<dyn InfraError>> {
        let content = std::fs::read_to_string(&self.path).map_err(into_infra)?;
        self.names = Some(
            content
                .lines()
                .map(|line| line.trim().to_string())
                // .filter(|line| !line.is_empty())
                .collect(),
        );
        self.index = 0;
        Ok(())
    }
}

impl NameReader for FileInput {
    fn read_name(&mut self) -> Result<String, Box<dyn InfraError>> {
        if self.names.is_none() {
            self.load_names()?;
        }

        // Safe: load_names always sets self.names to Some
        let names = self.names.as_ref().expect("names must be loaded");

        if self.index < names.len() {
            let name = names[self.index].clone();
            self.index += 1;
            Ok(name)
        } else {
            // Signal end of input — "quit" stops run_greeting_loop
            Ok("quit".to_string())
        }
    }
}
