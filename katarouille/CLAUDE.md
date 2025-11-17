# Katarouille - Rust GUI Client

This is a graphical user interface (GUI) application written in Rust using the Iced framework for managing kata training sessions.

## Project Guidelines

- Use conventional commits with `katarouille` scope for all commits (e.g., `feat(katarouille):`, `fix(katarouille):`, `test(katarouille):`)
- Run `cargo build` to ensure the code compiles before committing
- Format code with `cargo fmt` before committing
- Run `cargo clippy` to check for common mistakes
- Use existing patterns from the codebase when adding features
- All new features should eventually have corresponding tests (see TODO section)

## Rust Guidelines

- This project uses **Rust 2021 edition**
- **Always** use `cargo fmt` to format code
- **Always** handle errors explicitly - avoid `.unwrap()` in production code
- Use meaningful variable names - avoid single-letter names except in short closures
- Prefer `&str` over `String` for function parameters when possible
- **Never** use `panic!` for normal error handling - return `Result` instead
- Use `eprintln!` for error messages
- Follow Rust naming conventions:
  - Types and traits: `PascalCase`
  - Functions and variables: `snake_case`
  - Constants: `SCREAMING_SNAKE_CASE`
  - Modules: `snake_case`

## Iced Framework Guidelines

- This project uses **Iced 0.13**
- The Elm Architecture (Model-View-Update) is the core pattern:
  - **Model**: Holds application state (see `KatarouillePage` struct)
  - **View**: Renders the current state to UI elements (see `view()` method)
  - **Update**: Handles messages and updates state (see `update()` method)

### Application Structure

```rust
struct KatarouillePage {
    config: Config,
    api_client: ApiClient,
    state: AppState,
}
```

- `config`: Application configuration (API URL, token)
- `api_client`: HTTP client for API communication
- `state`: Current application state (determines which view to show)

### State Management

The application uses an enum for state management:

```rust
enum AppState {
    Loading,
    Authentication { ... },
    SessionList { ... },
    SessionCreate { ... },
}
```

Each state contains the data needed for that specific view.

### Message Handling

- Messages drive all state changes in Iced
- Messages are defined as an enum with variants for different actions
- Use pattern matching in `update()` to handle different message types
- **Always** return a `Task<Message>` from `update()` (use `Task::none()` if no async work needed)
- Use `Task::perform()` for async operations (API calls)

#### Message Patterns

```rust
// Async operation
Task::perform(
    async move {
        api_client.fetch_sessions().await.map_err(|e| e.to_string())
    },
    Message::SessionsFetched,
)

// No operation
Task::none()
```

### View Rendering Guidelines

- The `view()` method returns `Element<Message>`
- Use pattern matching on `state` to render different views
- Compose UI using widget functions: `button`, `text`, `column`, `row`, `container`, etc.
- Use builders pattern for widget configuration: `.size()`, `.padding()`, `.spacing()`, etc.
- **Always** provide a message for interactive widgets (`.on_press()` for buttons)

### Widget Guidelines

#### Layout Widgets

- `column![]` - Vertical layout
- `row![]` - Horizontal layout
- `container()` - Wrapper with styling
- `scrollable()` - Scrollable container

#### Interactive Widgets

- `button()` - Clickable button
- `text_input()` - Text input field
  - Use `.on_input(Message::variant)` for change handling

#### Display Widgets

- `text()` - Static text
  - Use `.size()` for font size
  - Use `.color()` for text color

#### Styling

- Use `.style()` closure for custom styling
- Colors: `Color::from_rgb(r, g, b)` where r, g, b are 0.0-1.0
- Common properties: `.padding()`, `.spacing()`, `.align_x()`, `.align_y()`

### Alignment and Sizing

- `Length::Fill` - Fill available space
- `Length::Shrink` - Minimal size
- `Alignment::Center` - Center alignment
- Use `.center(Length::Fill)` to center a container

## Code Organization

- **main.rs**: Application entry point and main UI logic
  - `KatarouillePage` struct and impl
  - `AppState` enum
  - `Message` enum
  - View rendering functions
- **models.rs**: Data models and types
  - `Kata` - Kata curriculum data
  - `Session` - Training session data
  - `SessionInput` - Input format for creating sessions
  - API response types
- **api.rs**: API client implementation
  - `ApiClient` struct
  - HTTP request methods
  - Authentication endpoints
- **auth.rs**: Authentication logic
  - Device flow functions
  - Token polling
- **config.rs**: Configuration management
  - `Config` struct
  - File persistence functions
  - XDG directory support

## API Integration Guidelines

- All API calls go through the `ApiClient` in `api.rs`
- The backend API is a Phoenix application running on `http://localhost:4000/api` by default
- API responses follow the format: `{ "data": [...] }`
- Use the `ApiResponse<T>` generic type for unmarshaling API responses
- **Always** handle API errors gracefully
- **Always** wrap API calls in async tasks that return messages
- Use `reqwest` for HTTP requests
- Include Bearer token in `Authorization` header for authenticated endpoints

### API Client Pattern

```rust
pub async fn fetch_sessions(&self) -> Result<Vec<Session>, Box<dyn std::error::Error>> {
    let url = format!("{}/sessions", self.base_url);

    let mut request = self.client.get(&url);

    if let Some(token) = &self.token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }

    let response = request.send().await?;

    if response.status() == 401 {
        return Err("Unauthorized: please login first".into());
    }

    let api_response: ApiResponse<Vec<Session>> = response.json().await?;

    Ok(api_response.data)
}
```

## Authentication Guidelines

This application uses **device flow authentication** (OAuth2-style) for secure authentication:

### Device Flow Process

1. **Initiate device flow**: `POST /api/auth/device/code`
   - Receives `device_code` (secret) and `user_code` (human-readable)
   - Receives `verification_uri` for user to visit
2. **Display user code** and verification URL in GUI
3. **Poll for authorization**: `POST /api/auth/device/token` with `device_code`
   - Poll every 5 seconds (as indicated by API response `interval`)
   - Returns `authorization_pending` while waiting
   - Returns access token when user approves in browser
   - Returns `access_denied` if user denies
4. **Store access token** and use for all subsequent API requests
5. Add token to requests: `Authorization: Bearer <token>` header

### Token Management

- Tokens are stored in `~/.config/katanaute/config.json`
- Tokens are included in all API requests to authenticated endpoints
- Sessions endpoint requires authentication: `GET/POST /api/sessions`
- Katas endpoint is public: `GET /api/katas`
- Handle `401 Unauthorized` responses by re-authenticating

### Authentication State

```rust
Authentication {
    user_code: Option<String>,      // Display to user
    verification_uri: Option<String>, // Display to user
    polling: bool,                   // Show "waiting" message
    error: Option<String>,           // Show error message
}
```

## Configuration Guidelines

- Configuration is in the `Config` struct
- Default API URL: `http://localhost:4000/api`
- Override with environment variable: `KATANAUTE_API_URL`
- Config file location: `~/.config/katanaute/config.json` (XDG-compliant)
- Use `directories` crate for cross-platform config directory

### Configuration File Format

```json
{
  "api_token": "token_here",
  "base_url": "http://localhost:4000/api"
}
```

## Error Handling Guidelines

- **Always** return `Result<T, Box<dyn std::error::Error>>` from fallible functions
- Use `?` operator for error propagation
- Convert errors to strings for display: `.map_err(|e| e.to_string())`
- Store error messages in state for display
- Display errors in red: `.color(Color::from_rgb(1.0, 0.0, 0.0))`

## Type Definitions

- Use `#[derive(Debug, Clone, Serialize, Deserialize)]` for data models
- Add JSON tags with `serde`: `#[serde(skip_serializing_if = "Option::is_none")]`
- Use `Option<T>` for optional fields
- Use enums for variants: `enum AppState { ... }`

## Data Handling Guidelines

- Sessions are sorted by `practiced_at` date in descending order (newest first)
- Use `chrono::DateTime<Utc>` for datetime fields
- Format datetime with `.format("%Y-%m-%d")`
- Sort with `.sort_by()` and custom comparison

## Color Coding

Kata levels are color-coded using RGB values:

```rust
match level {
    "yellow" => [1.0, 0.9, 0.0],    // Yellow
    "orange" => [1.0, 0.6, 0.0],    // Orange
    "green" => [0.0, 0.8, 0.0],     // Green
    "blue" => [0.0, 0.5, 1.0],      // Blue
    "brown" => [0.6, 0.4, 0.2],     // Brown
    "shodan" => [0.1, 0.1, 0.1],    // Black
    _ => [0.5, 0.5, 0.5],           // Gray (fallback)
}
```

## Testing Guidelines (TODO)

This project currently has no tests. When adding tests:

- Use Rust's built-in `#[cfg(test)]` and `#[test]`
- Name test modules `mod tests`
- Use `assert_eq!`, `assert!`, etc. for assertions
- Mock HTTP calls for API client tests
- Test state transitions in Update logic
- Run tests with `cargo test`
- Use `cargo test --verbose` for detailed output

## Dependencies

- **iced** - GUI framework (Elm Architecture)
- **reqwest** - HTTP client for API requests
- **serde** / **serde_json** - JSON serialization/deserialization
- **tokio** - Async runtime
- **directories** - XDG config directory support
- **chrono** - DateTime handling

## Common Patterns

### Switching Views

```rust
Message::SessionsFetched(result) => match result {
    Ok(sessions) => {
        self.state = AppState::SessionList {
            sessions,
            selected_session: None,
            error: None,
        };
        Task::none()
    }
    Err(e) => {
        // Handle error
    }
}
```

### Updating State Fields

```rust
Message::SelectKata(kata_id) => {
    if let AppState::SessionCreate { ref mut selected_kata_id, .. } = self.state {
        *selected_kata_id = Some(kata_id);
    }
    Task::none()
}
```

### Async API Calls

```rust
Message::Refresh => {
    let api_client = self.api_client.clone();
    self.state = AppState::Loading;
    Task::perform(
        async move { api_client.fetch_sessions().await.map_err(|e| e.to_string()) },
        Message::SessionsFetched,
    )
}
```

## Best Practices

- Keep `update()` method organized with clear pattern matching
- Separate view rendering into focused match arms
- Use descriptive names for message variants
- Document public functions and types
- Use early returns to avoid deep nesting
- Clone data when moving into async blocks
- Handle all error cases explicitly
- Use `eprintln!` for debugging (logs to stderr)

## Current Features

- ✅ Device flow authentication with OAuth2-style flow
- ✅ Bearer token API authentication
- ✅ Token persistence in config file
- ✅ View list of training sessions
- ✅ Display session details (expandable notes)
- ✅ Session creation with form UI
- ✅ Color-coded kata level badges
- ✅ Loading states
- ✅ Error display
- ✅ Logout functionality
- ✅ Responsive GUI with dark theme

## Known Limitations

- No session editing (only create and view)
- No session deletion
- No session filtering or search
- No offline mode or caching
- Config file is plain JSON (token not encrypted)
- No session statistics or analytics

## TODO

- [ ] Add unit tests for API client functions
- [ ] Add integration tests for authentication flow
- [ ] Implement session editing
- [ ] Implement session deletion
- [ ] Add search/filter functionality
- [ ] Add keyboard shortcuts
- [ ] Improve error messages and recovery
- [ ] Add session statistics view
- [ ] Add markdown rendering for notes
- [ ] Add session export functionality
- [ ] Improve UI styling and layout
- [ ] Add preferences/settings screen
- [ ] Consider token encryption in config file

## Building and Running

### Development

```bash
# Build the project
cargo build

# Run the application
cargo run

# Run with custom API URL
KATANAUTE_API_URL=http://localhost:4000/api cargo run
```

### Release

```bash
# Build optimized release binary
cargo build --release

# Binary will be at: target/release/katarouille
./target/release/katarouille
```

### Code Quality

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Run tests (when implemented)
cargo test
```

## Troubleshooting

### Build Issues

- Ensure Rust toolchain is up to date: `rustup update`
- Clean build artifacts: `cargo clean && cargo build`
- Check for conflicting dependencies: `cargo tree`

### Runtime Issues

- Verify backend is running on port 4000
- Check API URL configuration
- Check config file: `~/.config/katanaute/config.json`
- Remove config file to reset authentication

### GUI Issues

- Iced requires OpenGL/Vulkan support
- On Linux, ensure graphics drivers are installed
- On Wayland, may need to set `WINIT_UNIX_BACKEND=x11`
