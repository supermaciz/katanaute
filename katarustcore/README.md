# Katarustcore

Shared Rust library for Katanaute clients, providing common API client, authentication, configuration, and data models.

## Features

- **API Client**: HTTP client for Katanaute backend with session and kata management
- **Authentication**: Device flow OAuth2-style authentication for headless/CLI clients  
- **Configuration**: XDG-compliant config file management with token persistence
- **Models**: Shared data structures (Kata, Session, etc.) with serialization

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
katarustcore = { path = "../katarustcore" }
```

```rust
use katarustcore::{ApiClient, Config, initiate_device_flow, poll_for_authorization};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let mut api_client = ApiClient::new(config.base_url, config.api_token);
    
    // Use API client...
    Ok(())
}
```

## Architecture

- `api.rs` - HTTP client with reqwest
- `auth.rs` - Device flow helpers using tokio::time::sleep
- `config.rs` - Config file persistence with directories crate
- `models.rs` - Serde structs for API data

## Dependencies

- `reqwest` - HTTP client
- `tokio` - Async runtime and utilities
- `serde`/`serde_json` - Serialization
- `directories` - XDG config paths
- `chrono` - Date/time handling