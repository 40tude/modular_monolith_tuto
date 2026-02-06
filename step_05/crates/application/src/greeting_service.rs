// greeting_service.rs

// This service orchestrates the greeting flow by coordinating
// between input adapters, domain logic, and output adapters.

use crate::errors::Result;

#[derive(Default)]
pub struct GreetingService;

impl GreetingService {
    pub fn new() -> Self {
        Self
    }

    pub fn run_greeting_once(
        &self,
        input: &dyn domain::NameReader,
        output: &dyn domain::GreetingWriter,
    ) -> Result<()> {
        let name = input.read_name()?;
        let greeting = domain::greet(&name)?;
        output.write_greeting(&greeting)?;
        println!("\nGoodbye!");
        Ok(())
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
