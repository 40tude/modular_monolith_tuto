// input.rs

// Console Input Adapter.
// Implements the `NameReader` port for reading names from standard input (stdin).

use crate::errors::ConsoleError;
use domain::{NameReader, NameReaderError};
use std::io::{self, Write};

// Adapter for reading names from the console (stdin).
// This adapter prompts the user and reads a line of text from standard input.
#[derive(Default)]
pub struct ConsoleInput;

impl ConsoleInput {
    // Creates a new `ConsoleInput` adapter.
    pub fn new() -> Self {
        Self
    }
}

impl NameReader for ConsoleInput {
    fn read_name(&self) -> Result<String, NameReaderError> {
        // Prompt for input
        print!("> ");
        io::stdout()
            .flush()
            .map_err(|e| NameReaderError::Infrastructure(Box::new(ConsoleError::from(e))))?;

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| NameReaderError::Infrastructure(Box::new(ConsoleError::from(e))))?;

        let name = input.trim().to_string();

        Ok(name)
    }
}
