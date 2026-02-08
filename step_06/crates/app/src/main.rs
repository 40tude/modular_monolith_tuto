// main.rs

use adapter_console::{ConsoleInput, ConsoleOutput};
use adapter_file::{FileInput, FileOutput};

use application::GreetingService;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 06 - File Adapter Demo) ===");

    // Dependency injection: Create file-based adapters
    let output = ConsoleOutput::new();
    // let input = ConsoleInput::new();

    // let output = FileOutput::new("output.txt");
    let input = match FileInput::new("input.txt") {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Failed to read input file: {e}");
            return Ok(());
        }
    };

    // Create application service and run
    let service = GreetingService::new();
    service
        .run_greeting_once(&input, &output)
        // .run_greeting_loop(&input, &output)
        .context("Failed to run greeting service")?;

    println!("\nGoodbye!");
    Ok(())
}
