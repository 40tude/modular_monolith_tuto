// lib.rs

pub mod errors;
pub mod greeting_service;

// Re-export
pub use greeting_service::GreetingService;
