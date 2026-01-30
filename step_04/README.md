ports.rs is included ins domain.rs
Now in console_input.rs we have `impl domain::NameReader for ConsoleInput {...}` while in Step_03 we had `impl ports::NameReader for ConsoleInput {...}`


```powershell
cargo test --bin step_04 greeting_loop_with_mocks
```

Z! in test/adapters_test see
```rust
use step_04::domain::{self, GreetingWriter, NameReader};
```