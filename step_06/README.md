# Step 07ter:

> **Warning (Linux/macOS users):** The `.cargo/` folder contains Windows-specific configuration (custom target-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```


shared is removed

Root Cargo.toml: Removed shared from workspace members, added adapter_file.
  domain crate:
  - New error.rs: Error enum with EmptyName variant + Result<T> alias
  - ports.rs: Traits now return Result<T, Box<dyn std::error::Error + Send + Sync>> (aliased as PortError)
  - greeting.rs: Uses crate::error::{Error, Result} instead of shared::Result
  - lib.rs: Exports error, Error, Result

  adapter_console crate:
  - New error.rs: Error enum wrapping io::Error
  - input.rs / output.rs: Return Result<_, PortError> instead of shared::Result
  - Removed shared dependency

  adapter_file crate:
  - error.rs: Removed From<Error> for shared::Error impl (no longer needed)
  - input.rs / output.rs: Return Result<_, PortError>
  - Removed shared dependency
  - New tests/adapter_file_test.rs with 3 tests (missing file error, invalid path error, trait verification)

  application crate:
  - New error.rs: Error enum with Domain(domain::Error) + Adapter(Box<dyn Error>) variants + Result<T> alias
  - greeting_service.rs: Uses Error::Adapter for port errors, ? for domain errors
  - Removed shared dependency

  app crate: Removed shared dependency, no other changes.

  integration_tests crate: Removed shared dependency, mock return types updated to Result<_, PortError>.

  20 tests pass, 0 clippy warnings. The crates/shared/ directory remains on disk.



## Usage

### Run with Different Adapters

```bash
# Console to console (interactive)
cargo run -p app

# Console to console (single execution)
cargo run -p app -- --once

# File to console
cargo run -p app -- --input file:input.txt --output console --once

# Console to file
cargo run -p app -- --input console --output file:output.txt --once

# File to file
cargo run -p app -- --input file:input.txt --output file:output.txt --once
```

### Run Examples

```bash
# Console example
cargo run --example console_example

# File example
cargo run --example file_example

# Mixed example (file → console)
cargo run --example mixed_example
```

### Run Tests

```bash
# Run all tests across all crates
cargo test

# Test specific crate
cargo test -p domain
cargo test -p application
cargo test -p adapter_console

# Run integration tests only
cargo test --test integration_test

# Run with output visible
cargo test -- --nocapture
```

## Usage

```powershell
cargo test -p adapter_console
cargo test -p adapter_console --test adapter_console_test
cargo test -p adapter_console --test adapter_console_test console # any test containing "console"

cargo test -p adapter_file

cargo test -p application
cargo test -p domain --test domain_test
cargo test -p integration_tests

cargo run -p app
cargo run
```