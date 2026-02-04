// adapter_console_test.rs

//! Integration tests for the adapter_console crate.
//!
//! Console adapters are inherently difficult to test because they
//! interact with stdin/stdout. This file demonstrates patterns for
//! testing adapters that depend on I/O.

use adapter_console::errors::ConsoleError;
use adapter_console::{ConsoleInput, ConsoleOutput};
use domain::errors::InfraError;
use domain::{GreetingWriter, NameReader};
use std::io::{self, ErrorKind};

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
/// This help to ensure adapters stay compatible
/// with their port interfaces as the codebase evolves.
#[test]
fn adapters_implement_domain_traits() {
    fn assert_name_reader<T: NameReader>() {}
    fn assert_greeting_writer<T: GreetingWriter>() {}

    // These calls verify trait bounds at compile time
    assert_name_reader::<ConsoleInput>();
    assert_greeting_writer::<ConsoleOutput>();
}

/// Verifies that `ConsoleError` implements `InfraError` and can be downcasted.
///
/// This test demonstrates the pattern for recovering concrete error types
/// from a `Box<dyn InfraError>`, which is useful when callers need to
/// inspect specific error variants for logging or error handling.
#[test]
fn console_error_implements_infra_error_and_downcasts() {
    // Arrange: create a ConsoleError from an I/O error
    let io_err = io::Error::new(ErrorKind::BrokenPipe, "stdout broken");
    let console_err = ConsoleError::Io(io_err);

    // Act: box it as dyn InfraError (simulates what GreetingWriter returns)
    let boxed: Box<dyn InfraError> = Box::new(console_err);

    // Assert: downcast back to ConsoleError via as_any()
    let downcasted = boxed.as_any().downcast_ref::<ConsoleError>();
    assert!(downcasted.is_some(), "should downcast to ConsoleError");

    // Verify we get the right variant with correct data
    match downcasted.unwrap() {
        ConsoleError::Io(e) => {
            assert_eq!(e.kind(), ErrorKind::BrokenPipe);
            assert_eq!(e.to_string(), "stdout broken");
        }
        _ => panic!("expected Io variant"),
    }
}

/// Verifies downcasting works for the Output variant as well.
#[test]
fn console_error_output_variant_downcasts() {
    let err = ConsoleError::Output("write failed".into());
    let boxed: Box<dyn InfraError> = Box::new(err);

    let downcasted = boxed.as_any().downcast_ref::<ConsoleError>();
    assert!(matches!(downcasted, Some(ConsoleError::Output(msg)) if msg == "write failed"));
}
