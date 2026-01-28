// Rust guideline compliant 2025-05-10
//! Integration tests for the adapter_console crate.
//!
//! Console adapters are inherently difficult to test because they
//! interact with stdin/stdout. This file demonstrates patterns for
//! testing adapters that depend on I/O.

use adapter_console::{ConsoleInput, ConsoleOutput};
use domain::{GreetingWriter, NameReader};

/// Verifies that `ConsoleOutput` correctly implements `GreetingWriter`.
///
/// Since `ConsoleOutput` writes to stdout, we can only verify that
/// the method completes successfully. In a real scenario, you might
/// use dependency injection to pass a writer trait object.
#[test]
fn console_output_implements_greeting_writer() {
    // Arrange
    let output = ConsoleOutput::new();

    // Act - write_greeting returns Ok(()) on success
    let result = output.write_greeting("Hello, Test!");

    // Assert
    assert!(result.is_ok());
}

/// Compile-time verification that adapters implement required traits.
///
/// This is a useful pattern to ensure adapters stay compatible
/// with their port interfaces as the codebase evolves.
#[test]
fn adapters_implement_domain_traits() {
    fn assert_name_reader<T: NameReader>() {}
    fn assert_greeting_writer<T: GreetingWriter>() {}

    // These calls verify trait bounds at compile time
    assert_name_reader::<ConsoleInput>();
    assert_greeting_writer::<ConsoleOutput>();
}
