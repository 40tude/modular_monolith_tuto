// ports.rs

use std::any::Any;

// Trait for infrastructure errors implemented by adapters. Defines the
// contract for infrastructure errors without coupling the domain to specific
// infrastructure implementations.
pub trait InfraError: std::error::Error + Send + Sync + 'static {
    /// Returns self as Any for downcasting to concrete error types.
    fn as_any(&self) -> &dyn Any;
}

// Port for reading a name from an input source.
pub trait NameReader {
    fn read_name(&self) -> Result<String, Box<dyn InfraError>>;
}

// Port for writing a greeting to an output destination.
pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>>;
}
