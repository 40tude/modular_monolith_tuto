/// Integration tests for the complete greeting flow
///
/// These tests verify that all crates work together correctly.

use adapter_console::{ConsoleInput, ConsoleOutput};
use application::GreetingService;
use domain::{greet, GreetingWriter, NameReader};

// ============================================================================
// Mock Adapters for Testing
// ============================================================================

struct MockInput {
    name: String,
}

impl MockInput {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl NameReader for MockInput {
    fn read_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.name.clone())
    }
}

struct MockOutput {
    written: std::cell::RefCell<Vec<String>>,
}

impl MockOutput {
    fn new() -> Self {
        Self {
            written: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn last_written(&self) -> Option<String> {
        self.written.borrow().last().cloned()
    }
}

impl GreetingWriter for MockOutput {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.written.borrow_mut().push(greeting.to_string());
        Ok(())
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn domain_greet_function() {
    let result = greet("Alice");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello Alice.");
}

#[test]
fn service_with_mocks() {
    let input = MockInput::new("Roberto");
    let output = MockOutput::new();
    let service = GreetingService::new();

    let result = service.greet_once(&input, &output);
    assert!(result.is_ok());

    let written = output.last_written().unwrap();
    assert_eq!(written, "Ciao Roberto!");
}

#[test]
fn complete_flow_normal_greeting() {
    let input = MockInput::new("World");
    let output = MockOutput::new();
    let service = GreetingService::new();

    service.greet_once(&input, &output).unwrap();

    let written = output.last_written().unwrap();
    assert_eq!(written, "Hello World.");
}

#[test]
fn complete_flow_long_name() {
    let input = MockInput::new("VeryLongNameThatWillBeTruncated");
    let output = MockOutput::new();
    let service = GreetingService::new();

    service.greet_once(&input, &output).unwrap();

    let greeting = output.last_written().unwrap();
    assert_eq!(greeting.len(), 25);
    assert!(greeting.ends_with("..."));
}

#[test]
fn empty_name_error_handling() {
    let result = greet("");
    assert!(result.is_err());
}
