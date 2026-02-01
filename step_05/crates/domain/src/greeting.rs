/// Business domain for greeting logic
///
/// This module contains the core business rules for greeting generation.
// use crate::error::GreetingError;
use shared::Result;

/// Generates a greeting according to business rules.
///
/// Rules:
/// - Default: "Hello {name}." with a maximum of 25 characters total
/// - Special case: "Roberto" returns "Ciao Roberto!"
/// - If the name is too long, it is truncated and suffixed with "..."
///
/// # Errors
///
/// Returns a `GreetingError` if:
/// - The name is empty (`GreetingError::EmptyName`)
/// - The name exceeds reasonable limits
pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string().into());
    }

    // Special case for Roberto
    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    const MAX_LENGTH: usize = 25;
    const GREETING_PREFIX: &str = "Hello ";
    const GREETING_SUFFIX: &str = ".";
    const TRAILER: &str = "...";

    let available_for_name = MAX_LENGTH - GREETING_PREFIX.len() - GREETING_SUFFIX.len();

    // Name fits within the allowed length
    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    // Name is too long, truncate and add ellipsis
    let truncate_length = MAX_LENGTH - GREETING_PREFIX.len() - TRAILER.len();
    let truncated_name = &name[..truncate_length.min(name.len())];

    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
