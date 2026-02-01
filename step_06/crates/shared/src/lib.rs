// Rust guideline compliant 2025-05-10
//! Shared types and utilities for the application.
//!
//! This crate provides common type aliases used across all layers
//! of the hexagonal architecture. It has no dependencies on domain
//! or infrastructure, making it safe to import everywhere.

// Common error type alias using dynamic dispatch.
// pub type Error = Box<dyn std::error::Error>;

// Common result type alias.
// pub type Result<T> = std::result::Result<T, Error>;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("custom error: {0}")]
    Custom(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}
