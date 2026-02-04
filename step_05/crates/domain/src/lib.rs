// lib.rs

pub mod errors;
pub mod greeting;
pub mod ports;

// Re-export
pub use greeting::greet;
pub use ports::{GreetingWriter, NameReader};
