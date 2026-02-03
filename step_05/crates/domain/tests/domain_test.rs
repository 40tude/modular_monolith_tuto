// domain_test.rs

/// Unit tests specifically for domain logic
/// These tests verify business rules in isolation
use domain::greet;

const MAX_LENGTH: usize = 25;
// const GREETING_PREFIX: &str = "Hello ";
// const GREETING_SUFFIX: &str = ".";
const TRAILER: &str = "...";

#[test]
fn empty_name_returns_error() {
    let result = greet("");
    assert!(result.is_err());
    // assert_eq!(result.unwrap_err(), "Name cannot be empty");
    let err = result.unwrap_err();
    assert_eq!(err.to_string(), "Name cannot be empty");
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
    // Case sensitive - "roberto" should get normal greeting
    let result = greet("roberto");
    assert_eq!(result.unwrap(), "Hello roberto.");

    // Different name
    let result = greet("Robert");
    assert_eq!(result.unwrap(), "Hello Robert.");
}

#[test]
fn greeting_length_limit() {
    // "Hello " (6) + "." (1) = 7, so max name is 18 chars for MAX_LENGTH total
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
    // 19 chars should trigger truncation (6 + 19 + 1 = 26, exceeds MAX_LENGTH)
    let name = "NineteenCharactersX";
    let result = greet(name);
    assert!(result.is_ok());

    let greeting = result.unwrap();
    assert!(greeting.ends_with(TRAILER));
    assert_eq!(greeting.len(), MAX_LENGTH);
}

#[test]
fn domain_should_handle_unicode_names() {
    let result = greet("José");
    assert_eq!(result.unwrap(), "Hello José.");

    let result = greet("François");
    assert_eq!(result.unwrap(), "Hello François.");
}

#[test]
fn domain_should_truncate_long_unicode_names() {
    // Note: Unicode characters may have different byte lengths
    let long_unicode_name = "Müller-Öffentlicher-Straßenbahn-Überführung";
    let result = greet(long_unicode_name);

    assert!(result.is_ok());
    let greeting = result.unwrap();
    assert_eq!(greeting.len(), MAX_LENGTH);
    assert!(greeting.ends_with(TRAILER));
}
