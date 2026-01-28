use step_04::adapters;
use step_04::domain;

use step_04::Result;

/// Application entry point with dependency injection.
///
/// This main function demonstrates hexagonal architecture:
/// - Adapters are created at the composition root (main)
/// - Domain logic receives dependencies through trait objects
/// - Easy to swap implementations without changing domain code
fn main() -> Result<()> {
    println!("=== Greeting Service (Step 03 - Hexagonal Architecture) ===");
    println!("Enter a name to greet (or 'quit' to exit):\n");

    // Dependency injection: Create adapters
    let input: adapters::ConsoleInput = adapters::ConsoleInput::new();
    let output: adapters::ConsoleOutput = adapters::ConsoleOutput::new();

    // Run the greeting loop
    run_greeting_loop(&input, &output)?;

    Ok(())
}

/// Runs the interactive greeting loop using dependency-injected adapters.
///
/// # Arguments
///
/// * `input` - Adapter for reading names (implements `NameReader`)
/// * `output` - Adapter for writing greetings (implements `GreetingWriter`)
///
/// This function demonstrates the power of dependency injection:
/// - It doesn't know the concrete types (ConsoleInput, ConsoleOutput)
/// - It only depends on the traits (NameReader, GreetingWriter)
/// - We could pass FileInput, HttpInput, etc. without changing this code
fn run_greeting_loop(
    input: &dyn domain::NameReader,
    output: &dyn domain::GreetingWriter,
) -> Result<()> {
    loop {
        // Read name from input adapter
        let name = input.read_name()?;

        // Exit condition
        if name.eq_ignore_ascii_case("quit") || name.eq_ignore_ascii_case("exit") {
            println!("\nGoodbye!");
            break;
        }

        // Skip empty input
        if name.is_empty() {
            continue;
        }

        // Call domain logic (pure business rules)
        match domain::greet(&name) {
            Ok(greeting) => {
                // Write greeting to output adapter
                output.write_greeting(&greeting)?;
                println!(); // Extra newline for readability
            }
            Err(e) => {
                eprintln!("Error: {}\n", e);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock input for testing
    struct MockInput {
        names: Vec<String>,
        index: std::cell::RefCell<usize>,
    }

    impl MockInput {
        fn new(names: Vec<&str>) -> Self {
            Self {
                names: names.iter().map(|s| s.to_string()).collect(),
                index: std::cell::RefCell::new(0),
            }
        }
    }

    impl domain::NameReader for MockInput {
        fn read_name(&self) -> Result<String> {
            let mut idx = self.index.borrow_mut();
            if *idx < self.names.len() {
                let name = self.names[*idx].clone();
                *idx += 1;
                Ok(name)
            } else {
                Ok("quit".to_string())
            }
        }
    }

    /// Mock output for testing
    struct MockOutput {
        greetings: std::cell::RefCell<Vec<String>>,
    }

    impl MockOutput {
        fn new() -> Self {
            Self {
                greetings: std::cell::RefCell::new(Vec::new()),
            }
        }

        fn get_greetings(&self) -> Vec<String> {
            self.greetings.borrow().clone()
        }
    }

    impl domain::GreetingWriter for MockOutput {
        fn write_greeting(&self, greeting: &str) -> Result<()> {
            self.greetings.borrow_mut().push(greeting.to_string());
            Ok(())
        }
    }

    #[test]
    fn test_greeting_loop_with_mocks() {
        let input = MockInput::new(vec!["Alice", "Roberto"]);
        let output = MockOutput::new();

        // This should process Alice and Roberto, then quit
        run_greeting_loop(&input, &output).unwrap();

        let greetings = output.get_greetings();
        assert_eq!(greetings.len(), 2);
        assert_eq!(greetings[0], "Hello Alice.");
        assert_eq!(greetings[1], "Ciao Roberto!");
    }
}
