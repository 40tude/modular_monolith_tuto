//! Error types for the file adapter crate.

use std::path::PathBuf;

/// Errors produced by file I/O adapters.
///
/// Each variant captures the file path and the underlying I/O error,
/// providing contextual messages such as:
/// `failed to read input file 'input.txt': No such file or directory (os error 2)`
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to read an input file.
    #[error("failed to read input file '{}': {source}", path.display())]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },

    /// Failed to open an output file.
    #[error("failed to open output file '{}': {source}", path.display())]
    OpenFile {
        path: PathBuf,
        source: std::io::Error,
    },

    /// Failed to write to an output file.
    #[error("failed to write to output file '{}': {source}", path.display())]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },
}

// Rust guideline compliant 2025-05-01
