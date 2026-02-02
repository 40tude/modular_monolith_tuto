/// Example: File to File
///
/// Demonstrates reading from a file and writing to a file.
///
/// Run with: cargo run --example file_example

use std::fs;
use adapter_file::{FileInput, FileOutput};
use application::GreetingService;

fn main() -> anyhow::Result<()> {
    println!("=== File Example ===\n");

    // Create a test input file
    let input_path = "test_input.txt";
    let output_path = "test_output.txt";

    fs::write(input_path, "Roberto")?;
    println!("Created input file: {}", input_path);

    // Process greeting
    let input = FileInput::new(input_path);
    let output = FileOutput::new(output_path);
    let service = GreetingService::new();

    service.greet_once(&input, &output)?;
    println!("Processed greeting");

    // Display result
    let result = fs::read_to_string(output_path)?;
    println!("\nOutput file content:");
    println!("{}", result);

    // Cleanup
    fs::remove_file(input_path)?;
    fs::remove_file(output_path)?;
    println!("Cleaned up test files");

    Ok(())
}
