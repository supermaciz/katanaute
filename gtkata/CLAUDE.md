# GTKata - GTK4 GUI Client

A modern GTK4-based desktop GUI client for the Katanaute kata training tracker, written in Rust.

> **Note**: This document contains GTKata-specific development guidelines. For overall project information, see the root [CLAUDE.md](../CLAUDE.md).

## Overview

GTKata is a native desktop application that provides a rich graphical interface for managing kata training sessions. It features:

- **Modern GTK4 UI**: Clean, native desktop interface using GTK4 and Adwaita styling
- **Multiple Authentication Methods**:
  - Email/password login and registration
  - OAuth2-style device flow for secure authentication
- **Secure Token Storage**: Uses system keyring for secure credential storage
- **Session Management**: View, create, and manage training sessions
- **Kata Catalog**: Browse all available katas with color-coded level badges
- **Markdown Notes**: Rich text notes support with Markdown

## Quick Start

### Prerequisites

**System Dependencies** (Linux):
```bash
# Debian/Ubuntu
sudo apt install libgtk-4-dev build-essential libssl-dev pkg-config

# Fedora
sudo dnf install gtk4-devel gcc openssl-devel pkg-config

# Arch Linux
sudo pacman -S gtk4 base-devel openssl pkg-config
```

**Rust**:
- Rust 1.70+ (edition 2021)
- Install via [rustup](https://rustup.rs/)

### Build and Run

```bash
cd gtkata
cargo build --release
cargo run
```

Or run in debug mode:
```bash
cargo run
```

### Configuration

GTKata reads the API URL from the environment:

```bash
# Override default API URL
export KATANAUTE_API_URL=http://localhost:4000/api
cargo run
```

Default: `http://localhost:4000/api`

## Architecture

### Technology Stack

- **UI Framework**: GTK4 (gtk-rs bindings)
- **Styling**: Adwaita (GNOME's design system)
- **HTTP Client**: reqwest (blocking mode)
- **Serialization**: serde + serde_json
- **Async Runtime**: tokio (for non-blocking operations)
- **Secure Storage**: keyring (cross-platform credential storage)
- **Date/Time**: chrono

### Project Structure

```
gtkata/
├── src/
│   ├── main.rs              # Application entry point and setup
│   ├── models.rs            # Data models and types
│   ├── api.rs               # API client for backend communication
│   ├── config.rs            # Configuration and token storage
│   ├── auth_window.rs       # Authentication window (login/device flow)
│   ├── main_window.rs       # Main application window
│   └── session_dialog.rs    # New session creation dialog
├── Cargo.toml               # Dependencies and build configuration
├── CLAUDE.md                # This file
└── README.md                # User-facing documentation
```

### Module Responsibilities

#### `models.rs`
Defines all data structures that mirror the backend API:
- `Kata`: Kata information with level (Yellow to Shodan)
- `Session`: Training session with notes and metadata
- `User`: User account information
- `AuthResponse`: Authentication response with token
- Request/response types for all API endpoints

**Important**: The `KataLevel` enum must stay in sync with the Phoenix backend enum.

#### `api.rs`
HTTP client for all backend API communication:
- **Authentication**: login, register, logout, device flow
- **Sessions**: CRUD operations for training sessions
- **Katas**: Fetching kata catalog (public endpoint)

Uses `reqwest` in blocking mode with proper error handling via `anyhow::Result`.

**Token Management**: Tokens are set via `set_token()` and automatically included in `Authorization: Bearer <token>` headers.

#### `config.rs`
Secure storage for authentication tokens and user data:
- Uses system keyring (Keychain on macOS, Secret Service on Linux, Credential Manager on Windows)
- Functions: `save_token()`, `load_token()`, `clear_token()`, `clear_all()`

**Security**: Tokens are stored encrypted by the OS. Never log or expose tokens in debug output.

#### `auth_window.rs`
Authentication interface with two modes:

**Email/Password Tab**:
- Email and password entry fields
- Login and Register buttons
- Error display for failed authentication

**Device Flow Tab**:
- "Start Device Flow" button
- Displays user code and verification link
- Automatically polls for authorization completion
- Shows status updates during polling

Uses `glib::Sender` and `glib::Receiver` for thread-safe communication between background auth operations and UI updates.

#### `main_window.rs`
Main application window after authentication:

**Components**:
- Header with user email and logout button
- Toolbar with search, refresh, and "New Session" button
- Scrollable session list (ListBox)

**Data Loading**: Fetches sessions and katas on startup in background thread, updates UI via `glib::idle_add_once`.

#### `session_dialog.rs`
Modal dialog for creating new training sessions:

**Fields**:
- Kata selection (ComboBoxText populated with available katas)
- "In course" checkbox (default: checked)
- Notes TextView with Markdown support

Returns `CreateSessionRequest` with current timestamp when user clicks "Create".

## Development Guidelines

### GTK4 and Rust Patterns

**1. Widget Ownership**
GTK widgets use reference counting (`Rc<RefCell<T>>`). Clone widgets for closures:

```rust
let button = Button::new();
let button_clone = button.clone();
button.connect_clicked(move |_| {
    button_clone.set_label("Clicked!");
});
```

**2. Thread Safety with glib**
Never update UI from background threads directly. Use `glib::idle_add_once`:

```rust
thread::spawn(move || {
    let result = fetch_data();
    glib::idle_add_once(move || {
        // Update UI here
        label.set_text(&result);
    });
});
```

**3. Signal Handlers**
Use `connect_*` methods for event handling. Always use `move` closures and clone captured variables:

```rust
let entry = Entry::new();
let label = Label::new(None);
let label_clone = label.clone();

entry.connect_changed(move |entry| {
    label_clone.set_text(&entry.text());
});
```

**4. Error Handling**
Use `anyhow::Result` for operations that can fail. Display errors to users gracefully:

```rust
match api.get_sessions() {
    Ok(sessions) => { /* update UI */ },
    Err(e) => {
        error_label.set_text(&format!("Error: {}", e));
    }
}
```

### API Integration

**Blocking vs Async**:
- Currently uses `reqwest::blocking::Client`
- Background threads for all API calls
- UI updates via `glib::idle_add_once`

**Error Messages**:
Always provide user-friendly error messages. Extract meaningful info from API errors:

```rust
if !response.status().is_success() {
    let status = response.status();
    let text = response.text().unwrap_or_default();
    return Err(anyhow!("Operation failed: {} - {}", status, text));
}
```

### Styling with Adwaita

Use built-in CSS classes for consistent styling:

```rust
button.add_css_class("suggested-action");  // Blue action button
button.add_css_class("destructive-action"); // Red destructive button
label.add_css_class("title-1");            // Large title
label.add_css_class("dim-label");          // Dimmed secondary text
```

**Common Classes**:
- `suggested-action`: Primary action buttons (blue)
- `destructive-action`: Dangerous actions (red)
- `title-1`, `title-2`, `title-3`: Heading sizes
- `dim-label`: Secondary/dimmed text
- `error`: Error messages
- `boxed-list`: Rounded list container

### Authentication Flow

**1. Startup Check**:
```
App starts → Check keyring for token
            → If valid: Load main window
            → If invalid/missing: Show auth window
```

**2. Login Success**:
```
User logs in → Receive token
            → Save to keyring
            → Close auth window
            → Open main window
```

**3. Logout**:
```
User clicks logout → Call API logout endpoint
                  → Clear keyring
                  → Close main window
                  → Exit application
```

**4. Device Flow**:
```
User clicks "Start Device Flow"
→ API returns device_code and user_code
→ Display user_code and verification link
→ Poll API every {interval} seconds
→ On approval: Save token and load main window
→ On deny: Show error
→ On timeout: Show timeout error
```

## Testing

### Manual Testing Checklist

**Authentication**:
- [ ] Login with valid credentials
- [ ] Login with invalid credentials (shows error)
- [ ] Register new account
- [ ] Device flow authorization
- [ ] Device flow denial
- [ ] Device flow timeout
- [ ] Token persists across app restarts
- [ ] Logout clears token

**Sessions**:
- [ ] View session list
- [ ] Create new session
- [ ] Session appears in list after creation
- [ ] Refresh button updates list
- [ ] Sessions show correct kata names and levels

**UI**:
- [ ] Window resizes properly
- [ ] Buttons respond to clicks
- [ ] Text entry works correctly
- [ ] Error messages display properly

### Unit Testing (TODO)

Future work: Add unit tests for:
- API client methods (with mock HTTP responses)
- Data model serialization/deserialization
- Config storage and retrieval

**Recommended Framework**: `cargo test` with `mockito` for HTTP mocking

## Common Development Tasks

### Adding a New API Endpoint

1. **Add model types** in `models.rs`:
   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct NewFeatureRequest {
       pub field: String,
   }
   ```

2. **Add API method** in `api.rs`:
   ```rust
   pub fn new_feature(&self, request: NewFeatureRequest) -> Result<Response> {
       let url = format!("{}/new-endpoint", self.base_url);
       let response = self.client.post(&url)
           .headers(self.headers())
           .json(&request)
           .send()?;
       // Handle response...
   }
   ```

3. **Call from UI** in appropriate window/dialog

### Adding a New Window

1. Create new module file (e.g., `src/new_window.rs`)
2. Define struct with `window: ApplicationWindow` field
3. Implement `new()`, `build_ui()`, and `show()` methods
4. Add `mod new_window;` to `main.rs`
5. Import and use: `use crate::new_window::NewWindow;`

### Debugging

**Enable Rust Backtraces**:
```bash
RUST_BACKTRACE=1 cargo run
```

**GTK Inspector** (live UI debugging):
```bash
GTK_DEBUG=interactive cargo run
```

**Print Debugging**:
Use `eprintln!()` for debug output (goes to stderr, doesn't interfere with GTK):
```rust
eprintln!("Debug: value = {:?}", value);
```

**Network Debugging**:
Check reqwest requests with `RUST_LOG`:
```bash
RUST_LOG=reqwest=debug cargo run
```

## Known Limitations

- **No Async UI**: Uses blocking HTTP client with threads
- **Limited Session Management**: View and create only (no edit/delete)
- **No Search**: Search entry is present but not functional
- **No Session Details View**: Sessions shown in list only
- **No Offline Mode**: Requires active backend connection
- **No Tests**: No unit or integration tests yet

## Future Enhancements

### High Priority
- [ ] Implement session editing
- [ ] Implement session deletion
- [ ] Make search functionality work
- [ ] Add session detail view with Markdown rendering
- [ ] Unit and integration tests

### Medium Priority
- [ ] Keyboard shortcuts (Ctrl+N for new session, etc.)
- [ ] Session filtering (by kata, date range, in_course)
- [ ] Statistics view (sessions per kata, progress tracking)
- [ ] Dark mode support
- [ ] Settings dialog (API URL, preferences)

### Low Priority
- [ ] Migrate to async/await with tokio
- [ ] Custom CSS theming
- [ ] Export sessions to PDF/CSV
- [ ] Offline mode with local caching
- [ ] Multi-window support

## Building for Distribution

### Linux

**AppImage** (recommended):
```bash
# Build release binary
cargo build --release

# Use linuxdeploy to create AppImage
# (requires linuxdeploy and linuxdeploy-plugin-gtk)
```

**Flatpak**:
Create a Flatpak manifest and build with `flatpak-builder`.

**Distribution Packages**:
- Debian: Create `.deb` with `cargo-deb`
- RPM: Use `cargo-rpm` for Fedora/RHEL
- AUR: Create PKGBUILD for Arch Linux

### macOS

**App Bundle**:
```bash
cargo build --release
# Create .app bundle structure
# Sign and notarize for distribution
```

### Windows

**Installer**:
```bash
cargo build --release
# Create installer with Inno Setup or WiX
```

**Note**: Windows requires GTK4 runtime to be installed or bundled with the application.

## Resources

- **GTK4 Rust Book**: https://gtk-rs.org/gtk4-rs/stable/latest/book/
- **GTK4 Documentation**: https://docs.gtk.org/gtk4/
- **Adwaita Guidelines**: https://developer.gnome.org/hig/
- **gtk-rs API Docs**: https://gtk-rs.org/gtk4-rs/stable/latest/docs/
- **Rust Book**: https://doc.rust-lang.org/book/

## Contributing to GTKata

When making changes:

1. **Follow Rust conventions**: Use `rustfmt` and `clippy`
   ```bash
   cargo fmt
   cargo clippy
   ```

2. **Test thoroughly**: Manual testing checklist above

3. **Update documentation**: Keep this CLAUDE.md current

4. **Use conventional commits**:
   ```
   feat(gtkata): add session editing
   fix(gtkata): correct authentication error handling
   docs(gtkata): update build instructions
   ```

5. **Error handling**: Always use `Result` and provide user-friendly messages

6. **Thread safety**: Never update UI from background threads directly
