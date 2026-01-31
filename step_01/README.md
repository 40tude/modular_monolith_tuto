# Step 01:

> **Warning (Linux/macOS users):** The .cargo/ folder contains Windows-specific configuration (custom 	arget-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```


## Usage

```powershell
cargo run
cargo test
```

## Notes
Step 01: Extracted Domain

This step demonstrates the separation of business logic from infrastructure.
The domain module contains pure business rules that are independent of I/O.

Split ex07.rs in domain.rs, lib.rs and main.rs
Keep the test in domain.rs even if `greet()` is public and so the test could have been externalized