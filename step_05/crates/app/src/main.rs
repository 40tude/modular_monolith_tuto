// main.rs

use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 05 - Modular Monolith & Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    // Dependency injection: Create adapters
    let input = ConsoleInput::new();
    let output = ConsoleOutput::new();

    // Create application service and run either in a loop or once
    let service = GreetingService::new();

    // service
    //     .run_greeting_loop(&input, &output)
    //     .context("Failed to run interactive loop")?;

    service
        .run_greeting_once(&input, &output)
        .context("Failed to run the greeting service once")?;

    println!("\nGoodbye!");
    Ok(())
}
