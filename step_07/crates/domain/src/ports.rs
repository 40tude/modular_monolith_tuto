//! Ports (Interfaces) for the Hexagonal Architecture.
//!
//! Ports define the contracts between the domain and the outside world.
//! They are implemented by adapters which handle the actual I/O operations.
//!
//! Port methods return `Result<T, Box<dyn std::error::Error + Send + Sync>>`
//! so adapters are free to return their own error types without depending
//! on domain errors.

/// Boxed error type used by port trait methods.
pub type PortError = Box<dyn std::error::Error + Send + Sync>;

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
    fn read_name(&self) -> std::result::Result<String, PortError>;
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
    /// # Errors
    ///
    /// Returns an error if the greeting cannot be written to the destination.
    fn write_greeting(&self, greeting: &str) -> std::result::Result<(), PortError>;
}
