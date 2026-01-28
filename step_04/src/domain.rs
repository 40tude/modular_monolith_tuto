use crate::Result;

/// Business domain for greeting logic
/// Generates a greeting according to business rules.
/// Rules:
/// - Default: "Hello {name}." with a maximum of 25 characters total
/// - Special case: "Roberto" returns "Ciao Roberto!"
/// - If the name is too long, it is truncated and suffixed with "..."
///
/// # Errors
/// Returns an error if the name is empty.
pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string().into());
    }

    // Special case for Roberto
    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    const MAX_LENGTH: usize = 25;
    const GREETING_PREFIX: &str = "Hello ";
    const GREETING_SUFFIX: &str = ".";
    const TRAILER: &str = "...";

    let available_for_name = MAX_LENGTH - GREETING_PREFIX.len() - GREETING_SUFFIX.len();

    // Name fits within the allowed length
    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    // Name is too long, truncate and add ellipsis
    let truncate_length = MAX_LENGTH - GREETING_PREFIX.len() - TRAILER.len();

    let truncated_name = &name[..truncate_length.min(name.len())];
    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}

/// Ports (Interfaces) for the Hexagonal Architecture
///
/// Ports define the contracts between the domain and the outside world.
/// They are implemented by adapters which handle the actual I/O operations.
///
/// This module defines two primary ports:
/// - `NameReader`: For reading names from various sources (console, file, HTTP, etc.)
/// - `GreetingWriter`: For writing greetings to various destinations (console, file, etc.)

// use crate::Result;

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
