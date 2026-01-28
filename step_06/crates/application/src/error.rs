/// Application-level errors
///
/// These errors represent problems at the application/orchestration layer.
use domain::GreetingError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Failed to read input: {0}")]
    InputError(String),

    #[error("Failed to write output: {0}")]
    OutputError(String),

    #[error("Greeting failed: {0}")]
    GreetingError(#[from] GreetingError),

    #[error("Application error: {0}")]
    Other(String),
}
