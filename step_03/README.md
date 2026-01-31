# Step 03: Move toward Hexagonal Architecture (Ports & Adapters)

> **Warning (Linux/macOS users):** The .cargo/ folder contains Windows-specific configuration (custom 	arget-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```


## Usage

```powershell
# Build
cargo build

# Run
cargo run

# Run tests
cargo test
cargo test greeting_loop_with_mocks
cargo test --test adapters_test
cargo test --test domain_test
cargo test --test integration_test
```