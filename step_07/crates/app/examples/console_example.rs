/// Example: Console to Console
///
/// Demonstrates reading from console and writing to console.
///
/// Run with: cargo run --example console_example

use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;

fn main() -> anyhow::Result<()> {
    println!("=== Console Example ===");
    println!("Enter a name (this example processes one name only):\n");

    let input = ConsoleInput::new();
    let output = ConsoleOutput::new();
    let service = GreetingService::new();

    service.greet_once(&input, &output)?;

    Ok(())
}
