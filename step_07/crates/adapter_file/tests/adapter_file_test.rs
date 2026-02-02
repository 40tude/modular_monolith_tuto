//! Tests for the adapter_file crate.

use adapter_file::{FileInput, FileOutput};
use domain::{GreetingWriter, NameReader};

/// Reading a missing file produces an error containing the file path.
#[test]
fn read_missing_file_error_contains_path() {
    let input = FileInput::new("nonexistent_test_file.txt");
    let result = input.read_name();

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("nonexistent_test_file.txt"),
        "error should contain the file path, got: {err_msg}"
    );
    assert!(
        err_msg.contains("failed to read"),
        "error should contain operation context, got: {err_msg}"
    );
}

/// Writing to an invalid path produces an error containing the path.
#[test]
fn write_to_invalid_path_error_contains_path() {
    // Use a path that cannot be opened (directory that does not exist)
    let output = FileOutput::new("nonexistent_dir/output.txt");
    let result = output.write_greeting("Hello!");

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("nonexistent_dir"),
        "error should contain the file path, got: {err_msg}"
    );
}

/// Compile-time verification that file adapters implement domain port traits.
#[test]
fn adapters_implement_domain_traits() {
    fn assert_name_reader<T: NameReader>() {}
    fn assert_greeting_writer<T: GreetingWriter>() {}

    assert_name_reader::<FileInput>();
    assert_greeting_writer::<FileOutput>();
}
