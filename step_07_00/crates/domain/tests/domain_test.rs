// domain_test.rs

/// Unit tests specifically for domain logic.
/// These tests verify business rules in isolation.
use domain::greet;

const MAX_LENGTH: usize = 25;
const TRAILER: &str = "...";

#[test]
fn empty_name_returns_error() {
    let result = greet("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.to_string(), "name cannot be empty");
}

#[test]
fn normal_greeting() {
    let result = greet("Alice");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello Alice.");
}

#[test]
fn roberto_special_case() {
    let result = greet("Roberto");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Ciao Roberto!");
}

#[test]
fn domain_should_not_use_special_greeting_for_similar_names() {
    let result = greet("roberto");
    assert_eq!(result.unwrap(), "Hello roberto.");

    let result = greet("Robert");
    assert_eq!(result.unwrap(), "Hello Robert.");
}

#[test]
fn greeting_length_limit() {
    let result = greet("ExactlyEighteenChr");
    assert!(result.is_ok());

    let greeting = result.unwrap();
    assert_eq!(greeting, "Hello ExactlyEighteenChr.");
    assert_eq!(greeting.len(), MAX_LENGTH);
}

#[test]
fn truncation_for_long_names() {
    let long_name = "ThisIsAVeryLongNameThatExceedsTheLimit";
    let result = greet(long_name);
    assert!(result.is_ok());

    let greeting = result.unwrap();
    assert!(greeting.starts_with("Hello "));
    assert!(greeting.ends_with(TRAILER));
    assert_eq!(greeting.len(), MAX_LENGTH);
}

#[test]
fn boundary_case_nineteen_chars() {
    let name = "NineteenCharactersX";
    let result = greet(name);
    assert!(result.is_ok());

    let greeting = result.unwrap();
    assert!(greeting.ends_with(TRAILER));
    assert_eq!(greeting.len(), MAX_LENGTH);
}

#[test]
fn domain_should_handle_unicode_names() {
    let result = greet("Jose\u{0301}");
    assert_eq!(result.unwrap(), "Hello Jose\u{0301}.");

    let result = greet("Franc\u{0327}ois");
    assert_eq!(result.unwrap(), "Hello Franc\u{0327}ois.");
}

#[test]
fn domain_should_truncate_long_unicode_names() {
    let long_unicode_name = "Mu\u{0308}ller-O\u{0308}ffentlicher-Straßenbahn-U\u{0308}berfu\u{0308}hrung";
    let result = greet(long_unicode_name);

    assert!(result.is_ok());
    let greeting = result.unwrap();
    assert_eq!(greeting.len(), MAX_LENGTH);
    assert!(greeting.ends_with(TRAILER));
}
