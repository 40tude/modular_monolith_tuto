// input.rs

use crate::errors::FileError;
use domain::{NameReader, NameReaderError};
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileInput {
    name: String,
}

impl FileInput {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, FileError> {
        let path = path.into();
        let content = fs::read_to_string(&path).map_err(FileError::from)?;

        let name = content
            .lines()
            .next()
            .unwrap_or_default()
            .trim()
            .to_string();

        Ok(Self { name })
    }
}

impl NameReader for FileInput {
    fn read_name(&self) -> Result<String, NameReaderError> {
        Ok(self.name.clone())
    }
}
