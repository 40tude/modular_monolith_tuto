// console_output.rs

// Implements the `GreetingWriter` port for writing greetings to standard output (stdout).
use crate::error::Result;
use crate::ports;

// Adapter for writing greetings to the console (stdout).
// This adapter prints greeting messages to standard output.
pub struct ConsoleOutput;

#[derive(Default)]
impl ConsoleOutput {
    // Creates a new `ConsoleOutput` adapter.
    pub fn new() -> Self {
        Self
    }
}

impl ports::GreetingWriter for ConsoleOutput {
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        println!("{}", greeting);
        Ok(())
    }
}
