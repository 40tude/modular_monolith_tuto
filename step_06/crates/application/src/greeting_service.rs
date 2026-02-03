//! Greeting Service - Application Layer.
//!
//! This service orchestrates the greeting flow by coordinating
//! between input adapters, domain logic, and output adapters.

use crate::error::{Error, Result};

/// Service that orchestrates the use cases.
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

    /// Processes a single greeting operation.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] if any step in the greeting pipeline fails.
    pub fn greet_once(
        &self,
        input: &dyn domain::NameReader,
        output: &dyn domain::GreetingWriter,
    ) -> Result<()> {
        let name = input.read_name().map_err(Error::Adapter)?;
        let greeting = domain::greet(&name)?;
        output.write_greeting(&greeting).map_err(Error::Adapter)?;
        Ok(())
    }

    /// Runs an interactive greeting loop.
    ///
    /// Continuously reads names and generates greetings until
    /// the user enters "quit" or "exit".
    ///
    /// # Errors
    ///
    /// Bubbles up errors from adapters or domain logic.
    pub fn run_greeting_loop(
        &self,
        input: &dyn domain::NameReader,
        output: &dyn domain::GreetingWriter,
    ) -> Result<()> {
        loop {
            let name = input.read_name().map_err(Error::Adapter)?;

            if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
                println!("\nGoodbye!");
                break;
            }

            if name.is_empty() {
                continue;
            }

            let greeting = domain::greet(&name)?;
            output.write_greeting(&greeting).map_err(Error::Adapter)?;

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
