# Step 07:

> **Warning (Linux/macOS users):** The `.cargo/` folder contains Windows-specific configuration (custom target-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```

* File adapter support multiple line


## Notes
* Add mut at `fn read_name(&mut self)`

## Usage

```powershell
cargo test -p adapter_console
cargo test -p adapter_console --test adapter_console_test
cargo test -p adapter_console --test adapter_console_test console # any test containing "console"

cargo test -p application
cargo test -p domain --test domain_test
cargo test -p integration_tests

cargo run -p app
cargo run
```