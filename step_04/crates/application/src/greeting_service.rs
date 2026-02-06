// greeting_service.rs

//! Greeting Service - Application Layer.
//!
//! This service orchestrates the greeting flow by coordinating
//! between input adapters, domain logic, and output adapters.

use crate::error::Result;

/// Service that orchestrates the use cases.
///
/// This service:
/// 1. Reads a name from an input source (via `NameReader` port)
/// 2. Applies business rules (via `greet` domain function)
/// 3. Writes the greeting to an output destination (via `GreetingWriter` port)
#[derive(Default)]
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
    /// Bubble-up the error returned by .read_name() and greet()
    pub fn run_greeting_loop(
        &self,
        input: &dyn domain::NameReader,
        output: &dyn domain::GreetingWriter,
    ) -> Result<()> {
        loop {
            // Read name from input adapter
            let name = input.read_name()?;

            // Exit condition
            if name.eq_ignore_ascii_case("quit")
                || name.eq_ignore_ascii_case("exit")
                || name.eq_ignore_ascii_case("q!")
            {
                println!("\nGoodbye!");
                break;
            }

            // Skip empty input
            if name.is_empty() {
                continue;
            }

            // Call domain logic (pure business rules)
            match domain::greet(&name) {
                Ok(greeting) => {
                    // Write greeting to output adapter
                    output.write_greeting(&greeting)?;
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
