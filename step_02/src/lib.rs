// lib.rs

pub mod domain;
pub mod error;

// I DO NOT re-export greet() for convenience
// I want to write domain::greet() in main.rs
// pub use domain::greet;
