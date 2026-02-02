# Step 07:

> **Warning (Linux/macOS users):** The `.cargo/` folder contains Windows-specific configuration (custom target-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```


* Add adapter_file
* Add once mode
* Add clap


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