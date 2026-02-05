// main.rs

use adapter_file::{FileInput, FileOutput};
use application::GreetingService;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("=== Greeting Service (Step 06 - File Adapter Demo) ===");

    // Dependency injection: Create file-based adapters
    let input = FileInput::new("input.txt");
    let output = FileOutput::new("output.txt");

    // Create application service and run
    let service = GreetingService::new();
    service
        .run_greeting_loop(&input, &output)
        .context("Failed to run greeting service")?;

    Ok(())
}
