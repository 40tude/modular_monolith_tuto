/// End-to-end integration tests
///
/// These tests verify the complete greeting flow using dependency injection
/// with mock adapters, demonstrating how the ports and adapters work together.
use step_03::domain::greet;
// Traits must be in scope to call their methods (e.g., `input.read_name()`).
use step_03::Result;
use step_03::ports::{GreetingWriter, NameReader};

// ============================================================================
// Mock Adapters for Integration Testing
// ============================================================================

struct TestInput {
    name: String,
}

impl TestInput {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl NameReader for TestInput {
    fn read_name(&self) -> Result<String> {
        Ok(self.name.clone())
    }
}

struct TestOutput {
    written: std::cell::RefCell<Vec<String>>,
}

impl TestOutput {
    fn new() -> Self {
        Self {
            written: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn get_written(&self) -> Vec<String> {
        self.written.borrow().clone()
    }

    fn last_written(&self) -> Option<String> {
        self.written.borrow().last().cloned()
    }
}

impl GreetingWriter for TestOutput {
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        self.written.borrow_mut().push(greeting.to_string());
        Ok(())
    }
}

// ============================================================================
// Helper function to run the complete flow
// ============================================================================

fn run_greeting_flow(input: &dyn NameReader, output: &dyn GreetingWriter) -> Result<()> {
    let name = input.read_name()?;
    let greeting = greet(&name)?;
    output.write_greeting(&greeting)?;
    Ok(())
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_greet_integration() {
    let input = TestInput::new("World");
    let output = TestOutput::new();

    run_greeting_flow(&input, &output).unwrap();

    let written = output.last_written().unwrap();
    assert_eq!(written, "Hello World.");
}

#[test]
fn test_roberto_integration() {
    let input = TestInput::new("Roberto");
    let output = TestOutput::new();

    run_greeting_flow(&input, &output).unwrap();

    let written = output.last_written().unwrap();
    assert_eq!(written, "Ciao Roberto!");
}

#[test]
fn test_empty_name_integration() {
    let input = TestInput::new("");
    let output = TestOutput::new();

    let result = run_greeting_flow(&input, &output);
    assert!(result.is_err());
}

#[test]
fn test_long_name_integration() {
    let input = TestInput::new("VeryLongNameThatWillBeTruncated");
    let output = TestOutput::new();

    run_greeting_flow(&input, &output).unwrap();

    let greeting = output.last_written().unwrap();
    assert_eq!(greeting.len(), 25);
    assert!(greeting.ends_with("..."));
}

#[test]
fn test_multiple_greetings_integration() {
    let output = TestOutput::new();

    // Process multiple names
    let names = vec!["Alice", "Roberto", "Bob"];
    for name in names {
        let input = TestInput::new(name);
        run_greeting_flow(&input, &output).unwrap();
    }

    let written = output.get_written();
    assert_eq!(written.len(), 3);
    assert_eq!(written[0], "Hello Alice.");
    assert_eq!(written[1], "Ciao Roberto!");
    assert_eq!(written[2], "Hello Bob.");
}

#[test]
fn test_end_to_end_with_dependency_injection() {
    // This test demonstrates the power of dependency injection:
    // We can test the entire flow without any real I/O

    let input = TestInput::new("Philippe");
    let output = TestOutput::new();

    // Domain logic
    let name = input.read_name().unwrap();
    assert_eq!(name, "Philippe");

    // Business rules
    let greeting = greet(&name).unwrap();
    assert_eq!(greeting, "Hello Philippe.");

    // Output
    output.write_greeting(&greeting).unwrap();
    assert_eq!(output.last_written().unwrap(), "Hello Philippe.");
}
