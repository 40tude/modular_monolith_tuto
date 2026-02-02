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

pub use error::{Error, Result};
pub use greeting_service::GreetingService;