// lib.rs

//! File Adapter Crate
//!
//! This crate provides file I/O adapters:
//! - `FileInput`: Reads names from a file
//! - `FileOutput`: Writes greetings to a file

pub mod errors;
pub mod input;
pub mod output;

// Re-export for convenience
// ConsoleInput becomes public, as if it were located at the root of the adapters_console module.
// We will still have to write `use adapter_file::{FileInput, FileOutput};` which is good
pub use input::FileInput;
pub use output::FileOutput;
