// console_input.rs

// Implements the `NameReader` port for reading names from standard input (stdin).
use std::io::{self, Write};
use crate::error::Result;
use crate::ports;

// Adapter for reading names from the console (stdin).
// This adapter prompts the user and reads a line of text from standard input.
pub struct ConsoleInput;

#[derive(Default)]
impl ConsoleInput {
    // Creates a new `ConsoleInput` adapter.
    pub fn new() -> Self {
        Self
    }
}

impl ports::NameReader for ConsoleInput {
    fn read_name(&self) -> Result<String> {
        // Prompt for input
        print!("> ");
        io::stdout()
            .flush()
            .map_err(|e| format!("Failed to flush stdout: {}", e))?;

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read from stdin: {}", e))?;

        let name = input.trim().to_string();

        Ok(name)
    }
}
