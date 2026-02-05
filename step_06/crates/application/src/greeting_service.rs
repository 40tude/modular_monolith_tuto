// greeting_service.rs

// This service orchestrates the greeting flow by coordinating
// between input adapters, domain logic, and output adapters.

// use crate::errors::ApplicationError;
use crate::errors::Result;

pub struct GreetingService;

impl GreetingService {
    /// Creates a new greeting service.
    pub fn new() -> Self {
        Self
    }

    pub fn run_greeting_loop(
        &self,
        input: &dyn domain::NameReader,
        output: &dyn domain::GreetingWriter,
    ) -> Result<()> {
        loop {
            let name = input.read_name()?;

            if name.eq_ignore_ascii_case("quit")
                || name.eq_ignore_ascii_case("exit")
                || name.eq_ignore_ascii_case("q!")
            {
                println!("\nGoodbye!");
                break;
            }

            let greeting = match domain::greet(&name) {
                Ok(g) => g,
                Err(e) => {
                    println!("Error: {e}\n");
                    continue;
                }
            };
            output.write_greeting(&greeting)?;

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
