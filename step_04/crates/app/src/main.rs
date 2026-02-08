// main.rs

use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;

mod error;
use error::Result;

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 04 - Modular Monolith & Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    // Dependency injection: Create adapters
    let mut input = ConsoleInput::new();
    let output = ConsoleOutput::new();

    // Create application service and run
    let service = GreetingService::new();
    service.run_greeting_loop(&mut input, &output)?;
    println!("\nGoodbye!");
    Ok(())
}
