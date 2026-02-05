// output.rs

// Console Output Adapter.
// Implements the `GreetingWriter` port for writing greetings to standard output (stdout).

use domain::GreetingWriter;
use domain::InfraError;

// Adapter for writing greetings to the console (stdout).
// This adapter prints greeting messages to standard output.
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
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>> {
        // In a real scenario, println! could fail (e.g., stdout redirected to full disk)
        // For this example, we assume it always succeeds
        println!("{greeting}");
        Ok(())
    }
}
