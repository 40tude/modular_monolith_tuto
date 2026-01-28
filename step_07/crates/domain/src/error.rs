//! Domain error types.
//!
//! These errors represent business rule violations specific to the domain.

#[derive(Debug, thiserror::Error)]
pub enum GreetingError {
    #[error("Name cannot be empty")]
    EmptyName,

    #[error("Name too long: {0} characters (max {1})")]
    NameTooLong(usize, usize),
}
