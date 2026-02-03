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
/// the method completes successfully.
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
