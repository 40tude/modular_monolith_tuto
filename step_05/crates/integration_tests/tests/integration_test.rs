/// Integration tests for the complete greeting flow
///
/// These tests verify that all crates work together correctly.
use application::GreetingService;
use domain::{GreetingWriter, NameReader};
use shared::Result;

// Mock Adapters for Testing
struct MockNameReader {
    names: Vec<String>,
    index: std::cell::Cell<usize>,
}

impl MockNameReader {
    fn new(names: Vec<&str>) -> Self {
        Self {
            names: names.into_iter().map(String::from).collect(),
            index: std::cell::Cell::new(0),
        }
    }
}

impl NameReader for MockNameReader {
    fn read_name(&self) -> Result<String> {
        let idx = self.index.get();
        if idx < self.names.len() {
            self.index.set(idx + 1);
            Ok(self.names[idx].clone())
        } else {
            Ok("quit".to_owned())
        }
    }
}

/// Mock adapter that captures written greetings.
struct MockGreetingWriter {
    greetings: std::cell::RefCell<Vec<String>>,
}

impl MockGreetingWriter {
    fn new() -> Self {
        Self {
            greetings: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn greetings(&self) -> Vec<String> {
        self.greetings.borrow().clone()
    }
}

impl GreetingWriter for MockGreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<()> {
        self.greetings.borrow_mut().push(greeting.to_owned());
        Ok(())
    }
}

// Integration Tests
#[test]
fn domain_greet_function() {
    // Arrange
    let reader = MockNameReader::new(vec!["Alice", "Bob", "quit"]);
    let writer = MockGreetingWriter::new();
    let service = GreetingService::new();

    // Act
    let result = service.run_greeting_loop(&reader, &writer);

    // Assert
    assert!(result.is_ok());
    let greetings = writer.greetings();
    assert_eq!(greetings.len(), 2);
    assert!(greetings[0].contains("Alice"));
    assert!(greetings[1].contains("Bob"));
}

#[test]
fn service_with_mocks() {
    let reader = MockNameReader::new(vec!["Roberto", "quit"]);
    let writer = MockGreetingWriter::new();
    let service = GreetingService::new();

    // Act
    let result = service.run_greeting_loop(&reader, &writer);

    // Assert
    assert!(result.is_ok());
    let greetings = writer.greetings();
    assert_eq!(greetings.len(), 1);
    assert_eq!(greetings[0], "Ciao Roberto!");
}

#[test]
fn complete_flow_normal_greeting() {
    let reader = MockNameReader::new(vec!["World", "quit"]);
    let writer = MockGreetingWriter::new();
    let service = GreetingService::new();

    // Act
    let result = service.run_greeting_loop(&reader, &writer);

    // Assert
    assert!(result.is_ok());
    let greetings = writer.greetings();
    assert_eq!(greetings.len(), 1);
    assert_eq!(greetings[0], "Hello World.");
}

#[test]
fn complete_flow_long_name() {
    let reader = MockNameReader::new(vec!["VeryLongNameThatWillBeTruncated", "quit"]);
    let writer = MockGreetingWriter::new();
    let service = GreetingService::new();

    // Act
    let result = service.run_greeting_loop(&reader, &writer);

    // Assert
    assert!(result.is_ok());
    let greetings = writer.greetings();
    assert_eq!(greetings.len(), 1);
    assert_eq!(greetings[0].len(), 25);
    assert!(greetings[0].ends_with("..."));
}

#[test]
fn empty_name_error_handling() {
    let reader = MockNameReader::new(vec!["", "quit"]);
    let writer = MockGreetingWriter::new();
    let service = GreetingService::new();

    // Act
    let result = service.run_greeting_loop(&reader, &writer);

    // Assert
    assert!(result.is_ok());
    let greetings = writer.greetings();
    assert_eq!(greetings.len(), 0);
}
