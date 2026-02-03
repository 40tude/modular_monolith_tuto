//! Integration tests for the application crate.
//!
//! These tests demonstrate how to test the application layer
//! using mock adapters for input and output ports.

use application::GreetingService;
use domain::{GreetingWriter, NameReader, error::Result};

/// Mock adapter that returns predefined names.
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

#[test]
fn greeting_service_processes_valid_names() {
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
