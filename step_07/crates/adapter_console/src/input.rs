//! Console Input Adapter.
//!
//! Implements the `NameReader` port for reading names from standard input (stdin).

use std::io::{self, Write};

use domain::ports::PortError;
use domain::NameReader;

/// Adapter for reading names from the console (stdin).
///
/// This adapter prompts the user and reads a line of text from standard input.
pub struct ConsoleInput;

impl ConsoleInput {
    /// Creates a new `ConsoleInput` adapter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsoleInput {
    fn default() -> Self {
        Self::new()
    }
}

impl NameReader for ConsoleInput {
    fn read_name(&self) -> Result<String, PortError> {
        // Prompt for input
        print!("> ");
        io::stdout().flush()?;

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let name = input.trim().to_string();

        Ok(name)
    }
}
