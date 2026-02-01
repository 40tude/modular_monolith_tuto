//! Console Adapter Crate.
//!
//! This crate provides console I/O adapters:
//! - `ConsoleInput`: Reads names from stdin
//! - `ConsoleOutput`: Writes greetings to stdout

pub mod input;
pub mod output;

// Re-export for convenience
// ConsoleInput becomes public, as if it were located at the root of the adapters_console module.
// We will still have to write `use adapter_console::{ConsoleInput, ConsoleOutput};` which is good
pub use input::ConsoleInput;
pub use output::ConsoleOutput;
