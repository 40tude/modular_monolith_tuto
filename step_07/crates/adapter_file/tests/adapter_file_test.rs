// adapter_file_test.rs

//! Integration tests for the adapter_file crate.
//!
//! These tests verify file-based adapters using temporary files for isolation.

use adapter_file::errors::FileError;
use adapter_file::{FileInput, FileOutput};
use domain::{GreetingWriter, NameReader};

/// Reading from a nonexistent file returns a FileError with NotFound kind.
#[test]
fn read_name_from_missing_file_returns_file_error() {
    let mut input = FileInput::new("nonexistent_file_that_does_not_exist.txt");
    let err = input.read_name().unwrap_err();

    let file_err = err.as_any().downcast_ref::<FileError>().unwrap();
    assert!(
        matches!(file_err, FileError::Io(e) if e.kind() == std::io::ErrorKind::NotFound),
        "expected NotFound, got: {file_err}"
    );
}

/// Reading names returns each line in order, then "quit" at the end.
#[test]
fn read_names_returns_lines_then_quit() {
    let path = std::env::temp_dir().join("adapter_file_test_input.txt");
    std::fs::write(&path, "Alice\nBob\nRoberto\n").unwrap();

    let mut input = FileInput::new(&path);

    assert_eq!(input.read_name().unwrap(), "Alice");
    assert_eq!(input.read_name().unwrap(), "Bob");
    assert_eq!(input.read_name().unwrap(), "Roberto");
    assert_eq!(input.read_name().unwrap(), "quit");
    // Subsequent calls keep returning "quit"
    assert_eq!(input.read_name().unwrap(), "quit");

    let _ = std::fs::remove_file(&path);
}

/// Writing multiple greetings appends each one on its own line.
#[test]
fn write_greeting_appends_to_file() {
    let path = std::env::temp_dir().join("adapter_file_test_output.txt");
    let output = FileOutput::new(&path);

    output.write_greeting("Hello Alice.").unwrap();
    output.write_greeting("Ciao Roberto!").unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = content.lines().collect();

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "Hello Alice.");
    assert_eq!(lines[1], "Ciao Roberto!");

    let _ = std::fs::remove_file(&path);
}
// Rust guideline compliant 2025-05-07
