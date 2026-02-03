// mixed_example.rs

/// Example: File to Console
///
/// Demonstrates reading from a file and writing to console.
/// This shows the power of mixing different adapters!
///
/// Run with: cargo run --example mixed_example

use std::fs;
use adapter_console::ConsoleOutput;
use adapter_file::FileInput;
use application::GreetingService;

fn main() -> anyhow::Result<()> {
    println!("=== Mixed Example (File → Console) ===\n");

    // Create a test input file
    let input_path = "test_name.txt";
    fs::write(input_path, "Alice")?;
    println!("Created input file with name: Alice\n");

    // Process greeting: file input, console output
    let input = FileInput::new(input_path);
    let output = ConsoleOutput::new();
    let service = GreetingService::new();

    println!("Greeting from file:");
    service.greet_once(&input, &output)?;

    // Cleanup
    fs::remove_file(input_path)?;
    println!("\nCleaned up test file");

    Ok(())
}
