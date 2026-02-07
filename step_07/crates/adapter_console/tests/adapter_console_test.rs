// adapter_console_test.rs

//! Integration tests for the adapter_console crate.
//!
//! Console adapters are inherently difficult to test because they
//! interact with stdin/stdout. This file demonstrates patterns for
//! testing adapters that depend on I/O.

use adapter_console::errors::ConsoleError;
use adapter_console::{ConsoleInput, ConsoleOutput};
use domain::{GreetingWriter, InfraError, NameReader};
use std::io::{self, ErrorKind};

/// Verifies that `ConsoleOutput` correctly implements `GreetingWriter`.
#[test]
fn console_output_implements_greeting_writer() {
    let output = ConsoleOutput::new();
    let result = output.write_greeting("Hello, Test!");
    assert!(result.is_ok());
}

/// Compile-time verification that adapters implement required traits.
#[test]
fn adapters_implement_domain_traits() {
    fn assert_name_reader<T: NameReader>() {}
    fn assert_greeting_writer<T: GreetingWriter>() {}

    assert_name_reader::<ConsoleInput>();
    assert_greeting_writer::<ConsoleOutput>();
}

/// Verifies that `ConsoleError` implements `InfraError` and can be downcasted.
#[test]
fn console_error_implements_infra_error_and_downcasts() {
    let io_err = io::Error::new(ErrorKind::BrokenPipe, "stdout broken");
    let console_err = ConsoleError::Io(io_err);

    let boxed: Box<dyn InfraError> = Box::new(console_err);

    let downcasted = boxed.as_any().downcast_ref::<ConsoleError>();
    assert!(downcasted.is_some(), "should downcast to ConsoleError");

    match downcasted.unwrap() {
        ConsoleError::Io(e) => {
            assert_eq!(e.kind(), ErrorKind::BrokenPipe);
            assert_eq!(e.to_string(), "stdout broken");
        }
    }
}
