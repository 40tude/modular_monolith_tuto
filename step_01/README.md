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