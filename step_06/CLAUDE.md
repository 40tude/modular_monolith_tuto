# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```powershell
cargo build                                    # build all crates
cargo test                                     # run all tests
cargo test -p domain                           # test single crate
cargo test -p domain --test domain_test        # run specific test file
cargo test -p domain --test domain_test greet  # run tests matching "greet"
cargo run                                      # run app (or: cargo run -p app)
```

## Architecture

Hexagonal architecture (ports & adapters) as a Cargo workspace with strict dependency direction:

```
domain (core) <-- application <-- adapters <-- app
```

### Crates

- **domain**: Business logic + port traits (`NameReader`, `GreetingWriter`, `InfraError`). Zero external dependencies except `thiserror`. Defines error contracts that adapters must implement.
- **application**: `GreetingService` orchestrates flow between ports. Depends only on domain.
- **adapter_console**: Implements domain ports for stdin/stdout. Depends on domain.
- **app**: Entry point. Wires adapters to application via dependency injection.
- **integration_tests**: End-to-end tests using mock adapters.

### Error Architecture

Three-layer error system with `thiserror`:
- `DomainError`: Business rule violations (e.g., `EmptyName`)
- `InfraError` trait: Marker for adapter errors, requires `as_any()` for downcasting
- `ApplicationError`: Combines domain + infrastructure errors via `From` impls
- `NameReaderError`: Specialized enum for input operations (domain OR infra errors)

### Key Pattern: Port Traits

Domain defines traits (`ports.rs`) that adapters implement:
```rust
pub trait NameReader {
    fn read_name(&self) -> Result<String, NameReaderError>;
}
pub trait GreetingWriter {
    fn write_greeting(&self, greeting: &str) -> Result<(), Box<dyn InfraError>>;
}
```

Adapters implement `InfraError` for their error types to enable downcasting while maintaining domain purity.

## Project Config

- Edition 2024, resolver v3
- `.cargo/config.toml` contains Windows-specific settings (custom target-dir, CPU flags) - Linux/macOS users should delete it
