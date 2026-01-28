/// Step 03: Hexagonal Architecture (Ports & Adapters)
///
/// This step introduces the Dependency Inversion Principle (DIP).
/// The domain no longer depends on infrastructure. Instead:
/// - Ports (traits) define contracts
/// - Adapters (implementations) provide concrete I/O operations
///
/// This allows easy swapping of implementations and better testability.
pub mod adapters;
pub mod domain;
pub mod ports;

// Do NOT re-export main domain functions for convenience
// pub use domain::greet;

// Type aliases for error handling
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
