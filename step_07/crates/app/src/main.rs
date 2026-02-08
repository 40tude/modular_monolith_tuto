// main.rs

use adapter_console::{ConsoleInput, ConsoleOutput};
use adapter_file::{FileInput, FileOutput};

use application::GreetingService;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 07 - File Adapter Demo) ===");

    // Dependency injection: Create file-based adapters
    // let output = ConsoleOutput::new();
    // let mut input = ConsoleInput::new();

    let output = FileOutput::new("output.txt");
    let mut input = FileInput::new("input.txt");

    // Create application service and run
    let service = GreetingService::new();
    service
        // .run_greeting_once(&mut input, &output)
        .run_greeting_loop(&mut input, &output)
        .context("Failed to run greeting service")?;

    println!("\nGoodbye!");
    Ok(())
}
