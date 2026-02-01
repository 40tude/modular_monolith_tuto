# Step 02:

> **Warning (Linux/macOS users):** The `.cargo/` folder contains Windows-specific configuration (custom target-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```


## usage

```powershell
cargo test
cargo run
```

## Notes
* Extract the tests from domain.rs
* In lib.rs, no longer re-export `greet()` from `domain`. I want to have to write `domain::greet()`
* Create a `tests/` folder
    * Create `domain_test.rs` (because `greet()` is public)
    * Create `integration_test.rs`

At this point `domain_test.rs` and `integration_test.rs` look very similar because we only have one component (`domain.rs`)


## Project Structure

```
step_02/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point + console I/O
│   ├── domain.rs        # Business rules (isolated)
│   └── lib.rs           # Library re-exports
└── tests/
    ├── integration_test.rs  # Integration tests
    └── domain_test.rs       # Domain unit tests
```




