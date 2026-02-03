// ex03.rs
// cargo run --example ex03
// cargo test --example ex03
//
// Return error and handle the different rules
// Rules:
// - Default: "Hello {name}." (max 25 characters total)
// - Special case: "Roberto" -> "Ciao Roberto!"
// - If name > 22 chars, truncate to 19 chars + "..."
//
// Add 1 test

fn main() {
    match greet("Roberto") {
        Ok(greeting) => println!("{}\n", greeting),
        Err(e) => eprintln!("Error: {}\n", e),
    }
}

// Generates a greeting according to business rules
fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }

    // Special case for Roberto
    if name == "Roberto" {
        return Ok("Ciao Roberto!".to_string());
    }

    // Calculate greeting length
    let greeting_prefix = "Hello ";
    let greeting_suffix = ".";
    const MAX_LENGTH: usize = 25;
    let available_for_name = MAX_LENGTH - greeting_prefix.len() - greeting_suffix.len();

    // If name fits within limit
    if name.len() <= available_for_name {
        return Ok(format!("Hello {}.", name));
    }

    // Name is too long, truncate with ellipsis
    const TRAILER: &str = "...";
    let truncate_length = MAX_LENGTH - greeting_prefix.len() - TRAILER.len();
    let truncated_name = &name[..truncate_length.min(name.len())];
    Ok(format!("Hello {}{}", truncated_name, TRAILER))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_name_returns_error() {
        let result = greet("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "Name cannot be empty");
    }
}
