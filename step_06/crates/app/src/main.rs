use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 06 - Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    // Dependency injection: Create adapters
    let input = Box::new(ConsoleInput::new());
    let output = Box::new(ConsoleOutput::new());

    // Create application service
    let service = GreetingService::new();

    // Run the greeting loop
    service
        .run_interactive_loop(input.as_ref(), output.as_ref())
        .context("Failed to run interactive loop")?;

    Ok(())
}
