// input.rs

// Console Input Adapter.
// Implements the `NameReader` port for reading names from standard input (stdin).

use crate::errors::into_infra;
use domain::{InfraError, NameReader};
use std::io::{self, Write};

// Adapter for reading names from the console (stdin).
#[derive(Default)]
pub struct ConsoleInput;

impl ConsoleInput {
    // Creates a new `ConsoleInput` adapter.
    pub fn new() -> Self {
        Self
    }
}

impl NameReader for ConsoleInput {
    fn read_name(&self) -> Result<String, Box<dyn InfraError>> {
        print!("> ");
        io::stdout().flush().map_err(into_infra)?;
        //                  .map_err(|e| Box::new(ConsoleError::from(e)) as Box<dyn InfraError>)?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(into_infra)?;
        //                               .map_err(|e| Box::new(ConsoleError::from(e)) as Box<dyn InfraError>)?;

        Ok(input.trim().to_string())
    }
}
