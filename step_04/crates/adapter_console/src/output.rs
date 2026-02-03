// output.rs

//! Console Output Adapter.
//!
//! Implements the `GreetingWriter` port for writing greetings to standard output (stdout).

use crate::error::Result;
use domain::GreetingWriter;

/// Adapter for writing greetings to the console (stdout).
///
/// This adapter prints greeting messages to standard output.
pub struct ConsoleOutput;

impl ConsoleOutput {
    /// Creates a new `ConsoleOutput` adapter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for ConsoleOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl GreetingWriter for ConsoleOutput {
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        println!("{greeting}");
        Ok(())
    }
}
