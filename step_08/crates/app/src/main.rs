use anyhow::{Context, Result};
use clap::Parser;

use adapter_console::{ConsoleInput, ConsoleOutput};
use adapter_file::{FileInput, FileOutput};
use application::GreetingService;
use domain::{GreetingWriter, NameReader};

/// Greeting Service - Multi-Crate Architecture Demo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input source: "console" or "file:<path>"
    #[arg(short, long, default_value = "console")]
    input: String,

    /// Output destination: "console" or "file:<path>"
    #[arg(short, long, default_value = "console")]
    output: String,

    /// Single execution mode (process one name and exit)
    #[arg(long)]
    once: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("=== Greeting Service (Step 08 - Modular Monolith & Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    // Build input adapter based on CLI args
    let input = build_input_adapter(&args.input).context("Failed to create input adapter")?;

    // Build output adapter based on CLI args
    let output = build_output_adapter(&args.output).context("Failed to create output adapter")?;

    // Create application service
    let service = GreetingService::new();

    // Run in single or interactive mode
    if args.once {
        if args.input == "console" {
            println!("Enter a name to greet:");
        }
        service
            .greet_once(input.as_ref(), output.as_ref())
            .context("Failed to process greeting")?;
    } else {
        println!("Enter names to greet (or 'quit' to exit):\n");
        service
            .run_interactive_loop(input.as_ref(), output.as_ref())
            .context("Failed to run interactive loop")?;
    }

    Ok(())
}

/// Builds an input adapter based on the configuration string.
///
/// Format:
/// - "console" → Console input
/// - "file:<path>" → File input
fn build_input_adapter(config: &str) -> Result<Box<dyn NameReader>> {
    if config == "console" {
        Ok(Box::new(ConsoleInput::new()))
    } else if let Some(path) = config.strip_prefix("file:") {
        Ok(Box::new(FileInput::new(path)))
    } else {
        anyhow::bail!(
            "Unknown input type: '{}'. Use 'console' or 'file:<path>'",
            config
        )
    }
}

/// Builds an output adapter based on the configuration string.
///
/// Format:
/// - "console" → Console output
/// - "file:<path>" → File output
fn build_output_adapter(config: &str) -> Result<Box<dyn GreetingWriter>> {
    if config == "console" {
        Ok(Box::new(ConsoleOutput::new()))
    } else if let Some(path) = config.strip_prefix("file:") {
        Ok(Box::new(FileOutput::new(path)))
    } else {
        anyhow::bail!(
            "Unknown output type: '{}'. Use 'console' or 'file:<path>'",
            config
        )
    }
}
