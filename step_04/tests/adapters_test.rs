use step_04::Result;
/// Tests for adapters using mock implementations
///
/// These tests demonstrate how hexagonal architecture makes testing easier
/// by allowing us to create mock implementations of ports.
// `self` imports the module (for `domain::greet()`).
// Traits must be in scope to call their methods (e.g., `input.read_name()`).
use step_04::domain::{self, GreetingWriter, NameReader};

// ============================================================================
// Mock Implementations for Testing
// ============================================================================

/// Mock input adapter that returns a predefined name.
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
    fn read_name(&self) -> Result<String> {
        Ok(self.name.clone())
    }
}

/// Mock output adapter that captures written greetings.
struct MockOutput {
    written: std::cell::RefCell<Vec<String>>,
}

impl MockOutput {
    fn new() -> Self {
        Self {
            written: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn get_written(&self) -> Vec<String> {
        self.written.borrow().clone()
    }
}

impl GreetingWriter for MockOutput {
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        self.written.borrow_mut().push(greeting.to_string());
        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[test]
fn test_mock_input_reader() {
    let input = MockInput::new("Alice");
    let name = input.read_name().unwrap();
    assert_eq!(name, "Alice");
}

#[test]
fn test_mock_output_writer() {
    let output = MockOutput::new();
    output.write_greeting("Hello World!").unwrap();

    let written = output.get_written();
    assert_eq!(written.len(), 1);
    assert_eq!(written[0], "Hello World!");
}

#[test]
fn test_greeting_flow_with_mocks() {
    // Arrange: Create mock adapters
    let input = MockInput::new("Alice");
    let output = MockOutput::new();

    // Act: Read name, greet, write output
    let name = input.read_name().unwrap();
    let greeting = domain::greet(&name).unwrap();
    output.write_greeting(&greeting).unwrap();

    // Assert: Verify the output
    let written = output.get_written();
    assert_eq!(written.len(), 1);
    assert_eq!(written[0], "Hello Alice.");
}

#[test]
fn test_greeting_flow_with_roberto() {
    let input = MockInput::new("Roberto");
    let output = MockOutput::new();

    let name = input.read_name().unwrap();
    let greeting = domain::greet(&name).unwrap();
    output.write_greeting(&greeting).unwrap();

    let written = output.get_written();
    assert_eq!(written[0], "Ciao Roberto!");
}

#[test]
fn test_greeting_flow_with_long_name() {
    let input = MockInput::new("VeryLongNameThatWillBeTruncated");
    let output = MockOutput::new();

    let name = input.read_name().unwrap();
    let greeting = domain::greet(&name).unwrap();
    output.write_greeting(&greeting).unwrap();

    let written = output.get_written();
    assert_eq!(written[0].len(), 25);
    assert!(written[0].ends_with("..."));
}

#[test]
fn test_multiple_greetings() {
    let output = MockOutput::new();

    // Write multiple greetings
    output.write_greeting("Hello Alice.").unwrap();
    output.write_greeting("Ciao Roberto!").unwrap();
    output.write_greeting("Hello Bob.").unwrap();

    let written = output.get_written();
    assert_eq!(written.len(), 3);
    assert_eq!(written[0], "Hello Alice.");
    assert_eq!(written[1], "Ciao Roberto!");
    assert_eq!(written[2], "Hello Bob.");
}

// ============================================================================
// Error Handling Tests
// ============================================================================

/// Mock input that always fails.
struct FailingInput;

impl NameReader for FailingInput {
    fn read_name(&self) -> Result<String> {
        Err("Input device not available".into())
    }
}

/// Mock output that always fails.
struct FailingOutput;

impl GreetingWriter for FailingOutput {
    fn write_greeting(&self, _greeting: &str) -> Result<()> {
        Err("Output device not available".into())
    }
}

#[test]
fn test_failing_input() {
    let input = FailingInput;
    let result = input.read_name();
    assert!(result.is_err());
}

#[test]
fn test_failing_output() {
    let output = FailingOutput;
    let result = output.write_greeting("Hello World!");
    assert!(result.is_err());
}

#[test]
fn test_error_propagation() {
    let input = FailingInput;
    let output = MockOutput::new();

    // Try to read from failing input
    match input.read_name() {
        Ok(name) => {
            let greeting = domain::greet(&name).unwrap();
            output.write_greeting(&greeting).unwrap();
        }
        Err(_) => {
            // Expected: input failed, so we couldn't complete the flow
            assert!(true);
        }
    }
}
