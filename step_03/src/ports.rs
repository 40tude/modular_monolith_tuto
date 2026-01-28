/// Ports (Interfaces) for the Hexagonal Architecture
///
/// Ports define the contracts between the domain and the outside world.
/// They are implemented by adapters which handle the actual I/O operations.
///
/// This module defines two primary ports:
/// - `NameReader`: For reading names from various sources (console, file, HTTP, etc.)
/// - `GreetingWriter`: For writing greetings to various destinations (console, file, etc.)
///
use crate::Result;

/// Port for reading a name from an input source.
///
/// Implementations (adapters) can read from:
/// - Console (stdin)
/// - File
/// - HTTP request
/// - Database
/// - etc.
pub trait NameReader {
    /// Reads a name from the input source.
    ///
    /// # Errors
    ///
    /// Returns an error if the name cannot be read from the source.
    fn read_name(&self) -> Result<String>;
}

/// Port for writing a greeting to an output destination.
///
/// Implementations (adapters) can write to:
/// - Console (stdout)
/// - File
/// - HTTP response
/// - Database
/// - etc.
pub trait GreetingWriter {
    /// Writes a greeting to the output destination.
    ///
    /// # Arguments
    ///
    /// * `greeting` - The greeting message to write
    ///
    /// # Errors
    ///
    /// Returns an error if the greeting cannot be written to the destination.
    fn write_greeting(&self, greeting: &str) -> Result<()>;
}
