We now have crates for all components
Crates are independent and in their own workspace
Added application and app component
Console adapters for console are now in the same crate

No anyhow or thiserror yet.

crates/                                                                                                                                                                                          shared/           <- Error/Result (aucune dépendance)
    domain/           <- dépend de shared uniquement
    application/      <- dépend de shared + domain
    adapter_console/  <- dépend de shared + domain
    app/              <- dépend de tous



## Usage

```powershell
cargo test -p domain --test domain_test
cargo test -p application --test application_test
cargo test -p application --test application_test greeting_service_processes # test containing greeting_service_processes

```