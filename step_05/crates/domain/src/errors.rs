// error.rs

use std::any::Any;
use thiserror::Error;

// Domain errors represent business rule violations
#[derive(Error, Debug)]
pub enum DomainError {
    /// The provided name is empty.
    #[error("Name cannot be empty")]
    EmptyName,
}


// Trait for infrastructure errors - implemented by adapters
// This trait defines the contract for infrastructure errors without
// coupling the domain to specific infrastructure implementations.
// Adapters implement this trait for their specific error types.
pub trait InfraError: std::error::Error + Send + Sync + 'static {
    /// Returns self as Any for downcasting to concrete error types
    fn as_any(&self) -> &dyn Any;
}
