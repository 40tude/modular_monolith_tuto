//! Console Adapter Crate.
//!
//! This crate provides console I/O adapters:
//! - `ConsoleInput`: Reads names from stdin
//! - `ConsoleOutput`: Writes greetings to stdout

pub mod error;
pub mod input;
pub mod output;

pub use input::ConsoleInput;
pub use output::ConsoleOutput;
