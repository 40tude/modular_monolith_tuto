/// Adapters (Implementations) for the Hexagonal Architecture
///
/// Adapters are concrete implementations of ports (traits).
/// They handle the actual I/O operations and adapt external interfaces
/// to the domain's expectations.
///
/// This module contains:
/// - `console_input`: Reads names from stdin
/// - `console_output`: Writes greetings to stdout
pub mod console_input;
pub mod console_output;

// Re-export for convenience
// ConsoleInput becomes public, as if it were located at the root of the adapters module.
// We will still have to write adapters::ConsoleInput... which is good
pub use console_input::ConsoleInput;
pub use console_output::ConsoleOutput;
