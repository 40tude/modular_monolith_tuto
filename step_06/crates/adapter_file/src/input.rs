// input.rs

use crate::errors::FileError;
use domain::{NameReader, NameReaderError};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;

pub struct FileInput {
    path: PathBuf,
    lines: RefCell<Option<Vec<String>>>,
    index: RefCell<usize>,
}

impl FileInput {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            lines: RefCell::new(None),
            index: RefCell::new(0),
        }
    }

    // Lazy load: read file on first call
    fn ensure_loaded(&self) -> Result<(), NameReaderError> {
        if self.lines.borrow().is_some() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.path)
            .map_err(|e| NameReaderError::Infrastructure(Box::new(FileError::from(e))))?;

        let lines: Vec<String> = content
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        *self.lines.borrow_mut() = Some(lines);
        Ok(())
    }
}

impl NameReader for FileInput {
    fn read_name(&self) -> Result<String, NameReaderError> {
        self.ensure_loaded()?;

        let idx = *self.index.borrow();

        // Scope the borrow to release it before mutating index
        let name = {
            let lines_ref = self.lines.borrow();
            let lines = lines_ref.as_ref().unwrap();

            if idx >= lines.len() {
                return Ok("quit".to_string());
            }

            lines[idx].clone()
        };

        *self.index.borrow_mut() = idx + 1;
        Ok(name)
    }
}
