// output.rs

// Console Output Adapter.
// Implements the `GreetingWriter` port for writing greetings to standard output (stdout).

use crate::errors::into_infra;
use domain::GreetingWriter;
use domain::InfraError;
use std::io::{self, Write};

// Adapter for writing greetings to the console (stdout).
#[derive(Default)]
pub struct ConsoleOutput;

impl ConsoleOutput {
    /// Creates a new `ConsoleOutput` adapter.
    pub fn new() -> Self {
        Self
    }
}

impl GreetingWriter for ConsoleOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>> {
        // println!("{greeting}"); is replaced by the next expression to show
        // how to handle error (e.g., stdout redirected to full disk)
        writeln!(io::stdout(), "{greeting}").map_err(into_infra)?;
        //                                  .map_err(|e| Box::new(ConsoleError::from(e)) as Box<dyn InfraError>)?;
        Ok(())
    }
}
