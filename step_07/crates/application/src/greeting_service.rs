//! Greeting Service - Application Layer.
//!
//! This service orchestrates the greeting flow by coordinating
//! between input adapters, domain logic, and output adapters.

use domain::{greet, GreetingWriter, NameReader};

use crate::error::ApplicationError;

/// Service that orchestrates the greeting process.
///
/// This service:
/// 1. Reads a name from an input source (via `NameReader` port)
/// 2. Applies business rules (via `greet` domain function)
/// 3. Writes the greeting to an output destination (via `GreetingWriter` port)
pub struct GreetingService;

impl GreetingService {
    /// Creates a new greeting service.
    pub fn new() -> Self {
        Self
    }

    /// Runs an interactive greeting loop.
    ///
    /// Continuously reads names and generates greetings until
    /// the user enters "quit" or "exit".
    ///
    /// # Arguments
    ///
    /// * `input` - Adapter for reading names
    /// * `output` - Adapter for writing greetings
    ///
    /// # Errors
    ///
    /// Returns `ApplicationError` if a fatal error occurs.
    pub fn run_interactive_loop(
        &self,
        input: &dyn NameReader,
        output: &dyn GreetingWriter,
    ) -> Result<(), ApplicationError> {
        loop {
            // Read name
            let name = input
                .read_name()
                .map_err(|e| ApplicationError::InputError(e.to_string()))?;

            // Check for exit commands
            if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
                println!("\nGoodbye!");
                break;
            }

            // Skip empty input
            if name.is_empty() {
                continue;
            }

            // Process greeting
            match greet(&name) {
                Ok(greeting) => {
                    output
                        .write_greeting(&greeting)
                        .map_err(|e| ApplicationError::OutputError(e.to_string()))?;
                }
                Err(e) => {
                    eprintln!("Error: {}\n", e);
                }
            }
            println!(); // Extra newline for readability
        }

        Ok(())
    }
}

impl Default for GreetingService {
    fn default() -> Self {
        Self::new()
    }
}
