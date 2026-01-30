/// Step 02: Extracted Domain
///
/// This step demonstrates the separation of business logic from infrastructure.
/// The domain module contains pure business rules that are independent of I/O.
pub mod domain;

// DO NOT re-export greet() for convenience
// I want to write domain::greet() in main.rs
// pub use domain::greet;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
