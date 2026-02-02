//! Error types for the adapter_console crate.

/// Errors produced by console I/O adapters.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error occurred during console interaction.
    #[error("console I/O error: {0}")]
    Io(#[from] std::io::Error),
}

