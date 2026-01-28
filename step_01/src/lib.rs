pub mod domain;

// Re-export domain functions. This allows the functions of the domain module
// (such as greet()) to be used directly via the main crate, without having to
// write `domain::greet`. This simplifies importing for crate users. They can
// `use crate::greet;` instead of `use crate::domain::greet;`. It is therefore a
// question of ease of use and clarity for code consumers.
pub use domain::greet;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
