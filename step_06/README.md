# Step 06:


> **Warning (Linux/macOS users):** The .cargo/ folder contains Windows-specific configuration (custom 	arget-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```

Same as Step_05 but use anyhow and thiserror




## Usage

```powershell
cargo test -p domain domain_test

```