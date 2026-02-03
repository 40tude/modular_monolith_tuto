// lib.rs

//! Domain Crate - Core Business Logic.
//!
//! This crate contains:
//! - Business rules (greeting logic)
//! - Domain errors (business rule violations)
//! - Ports (trait definitions for I/O boundaries)
//!
//! The domain has ZERO dependencies on infrastructure or adapters.
//! It only defines what it needs through traits (ports).

pub mod error;
pub mod greeting;
pub mod ports;

pub use error::{Error, Result};
pub use greeting::greet;
pub use ports::{GreetingWriter, NameReader, PortError};
