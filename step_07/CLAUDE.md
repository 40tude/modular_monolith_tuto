# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```powershell
cargo build                              # build all workspace members
cargo test                               # run all tests
cargo test -p domain --test domain_test  # single test file
cargo test -p application                # single crate tests
cargo test -p adapter_console            # adapter tests
cargo test -p integration_tests          # end-to-end tests
cargo run -p app                         # run greeting service
```

## Architecture

Hexagonal architecture (Ports & Adapters) as a Cargo workspace with 6 crates:

```
app (binary) ─── wires adapters into application via manual DI
  ├── application (GreetingService) ─── orchestrates domain + ports
  │     └── domain ─── pure business logic + port trait definitions
  ├── adapter_console ─── stdin/stdout impl of ports
  │     └── domain
  ├── adapter_file ─── file I/O impl of ports (WIP, not in workspace members)
  │     └── domain
  └── integration_tests ─── end-to-end tests with mock adapters
```

**Domain** defines two port traits (`NameReader`, `GreetingWriter`) and `InfraError` trait. Application depends only on these traits, never on adapter crates.

**Dependency injection** is manual: `main.rs` creates concrete adapters and passes them as `&mut dyn NameReader` / `&dyn GreetingWriter` to `GreetingService`.

## Error Handling Pattern

Three-tier error system:

1. **DomainError** - Business rule violations (empty name). Recoverable; application continues in loop mode.
2. **InfraError** (trait in domain) - Infrastructure failures. Fatal; propagated up. Each adapter defines its own error type implementing `InfraError`.
3. **ApplicationError** - Wraps both via `#[from]` for domain, manual conversion for infra.

**Orphan rule workaround**: Adapters wrap `std::io::Error` in their own error type (e.g., `ConsoleError`) to implement `InfraError`. Helper `into_infra()` converts and boxes.

## Key Domain Rules

- `greet("Roberto")` → `"Ciao Roberto!"` (special case)
- `greet(name)` → `"Hello {name}."` (normal)
- Max greeting length: 25 chars; truncated with `"..."`
- Empty name → `DomainError::EmptyName`

## Testing Patterns

- **Domain tests**: Pure unit tests on `greet()` function
- **Application/integration tests**: Inline mock adapters (`MockNameReader` with sequence of names, `MockGreetingWriter` with `RefCell<Vec>` capture)
- **Adapter tests**: Trait bound verification, `InfraError` downcasting checks

## Notes

- `.cargo/config.toml` has Windows-specific target-dir and CPU flags; delete on non-Windows
- Edition 2024, deps: `thiserror = "2.0"`, `anyhow = "1.0"`
- `adapter_file` is WIP: `FileInput::read_name` has a bug (`self.name` doesn't exist), crate commented out of workspace members
- `NameReader::read_name` takes `&mut self`; mocks and adapters must match this signature
