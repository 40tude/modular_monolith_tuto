// adapter_file_test.rs

//! Integration tests for the adapter_file crate.
//!
//! These tests demonstrate how to verify file-based adapters
//! using temporary files for isolation.

use adapter_file::errors::FileError;
use adapter_file::{FileInput, FileOutput};
use domain::GreetingWriter;

/// Creating a FileInput from a nonexistent file returns a FileError
/// that can be matched as an I/O NotFound error.
#[test]
fn new_from_missing_file_returns_file_error() {
    let err = FileInput::new("nonexistent_file_that_does_not_exist.txt").unwrap_err();

    assert!(
        matches!(err, FileError::Io(ref e) if e.kind() == std::io::ErrorKind::NotFound),
        "expected NotFound, got: {err}"
    );
}

/// Writing multiple greetings appends each one on its own line.
// #[test]
// fn write_greeting_appends_to_file() {
//     let path = std::env::temp_dir().join("adapter_file_test_output.txt");
//     let output = FileOutput::new(&path);

//     output.write_greeting("Hello Alice!").unwrap();
//     output.write_greeting("Ciao Roberto!").unwrap();

//     let content = std::fs::read_to_string(&path).unwrap();
//     let lines: Vec<&str> = content.lines().collect();

//     assert_eq!(lines.len(), 2);
//     assert_eq!(lines[0], "Hello Alice!");
//     assert_eq!(lines[1], "Ciao Roberto!");

//     // Cleanup
//     let _ = std::fs::remove_file(&path);
// }
