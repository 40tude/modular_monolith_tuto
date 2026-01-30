use step_02::domain;

const MAX_LENGTH: usize = 25;
// const GREETING_PREFIX: &str = "Hello ";
// const GREETING_SUFFIX: &str = ".";
const TRAILER: &str = "...";

#[test]
fn greet_integration() {
    // Test normal greeting
    let result = domain::greet("World");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Hello World.");
}

#[test]
fn roberto_integration() {
    // Test special case
    let result = domain::greet("Roberto");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Ciao Roberto!");
}

#[test]
fn empty_name_integration() {
    // Test error case
    let result = domain::greet("");
    assert!(result.is_err());
}

#[test]
fn long_name_integration() {
    // Test truncation
    let result = domain::greet("VeryLongNameThatWillBeTruncated");
    assert!(result.is_ok());

    let greeting = result.unwrap();
    assert_eq!(greeting.len(), MAX_LENGTH);
    assert!(greeting.ends_with(TRAILER));
}
