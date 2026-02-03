# Step 04:

> **Warning (Linux/macOS users):** The `.cargo/` folder contains Windows-specific configuration (custom target-dir for OneDrive, CPU flags). Delete or rename before building:
> ```bash
> rm -rf .cargo   # or: mv .cargo .cargo.bak
> ```


* We now have crates for all components
* Crates are independent and in their own workspace
* Added `application`, `app`, `integration_test` and `shared` (errors) component
* Console adapters for console I/O are now in the same crate

```text

crates/                                                                                                                                                                                             shared/           <- Error/Result (aucune dépendance)
    domain/           <- dépend de shared uniquement
    application/      <- dépend de shared + domain
    adapter_console/  <- dépend de shared + domain
    app/              <- dépend de tous
```



```text
step_05/
│   Cargo.toml
└───crates
    ├───adapter_console
    │   │   Cargo.toml
    │   ├───src
    │   │       input.rs
    │   │       lib.rs
    │   │       output.rs
    │   └───tests
    │           adapter_console_test.rs
    ├───app
    │   │   Cargo.toml
    │   └───src
    │           main.rs
    ├───application
    │   │   Cargo.toml
    │   ├───src
    │   │       greeting_service.rs
    │   │       lib.rs
    │   └───tests
    │           application_test.rs
    ├───domain
    │   │   Cargo.toml
    │   ├───src
    │   │       greeting.rs
    │   │       lib.rs
    │   │       ports.rs
    │   └───tests
    │           domain_test.rs
    ├───integration_tests
    │   │   Cargo.toml
    │   ├───src
    │   │       lib.rs
    │   └───tests
    │           integration_test.rs
    └───shared
        │   Cargo.toml
        └───src
                lib.rs
```

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