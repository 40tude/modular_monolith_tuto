# Step 04:


> **Warning (Linux/macOS users):** The `.cargo/` folder contains Windows-specific configuration (custom target-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```


ports.rs is included ins domain.rs
Now in console_input.rs we have `impl domain::NameReader for ConsoleInput {...}` while in Step_03 we had `impl ports::NameReader for ConsoleInput {...}`




## Usage


```powershell
cargo test --bin step_04 greeting_loop_with_mocks
```

Z! in test/adapters_test see
```rust
use step_04::domain::{self, GreetingWriter, NameReader};
```