//! Application Crate - Use Cases and Orchestration.
//!
//! This crate contains:
//! - Application services (orchestration logic)
//! - Application-level errors
//! - Use case implementations
//!
//! The application layer coordinates between domain and adapters.

pub mod error;
pub mod greeting_service;

// Re-export commonly used items
pub use error::ApplicationError;
pub use greeting_service::GreetingService;
