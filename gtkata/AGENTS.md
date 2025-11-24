# GTKata - GTK4 + libadwaita GUI Client

This is a graphical user interface (GUI) application for Linux written in Rust using GTK4 and libadwaita for managing kata training sessions.

## Project Guidelines

- Use conventional commits with `gtkata` scope for all commits (e.g., `feat(gtkata):`, `fix(gtkata):`, `test(gtkata):`)
- Run `cargo build` to ensure the code compiles before committing
- Format code with `cargo fmt` before committing
- Run `cargo clippy` to check for common mistakes
- Use existing patterns from the codebase when adding features
- Follow GNOME Human Interface Guidelines (HIG)
- All new features should eventually have corresponding tests (see TODO section)

## Rust Guidelines

- This project uses **Rust 2024 edition**
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

## GTK4 and libadwaita Guidelines

- This project uses **GTK4** and **libadwaita**
- **DO NOT** use relm4 - this is a pure GTK4/libadwaita application
- Follow GNOME Human Interface Guidelines: https://developer.gnome.org/hig

### Core Principles

1. **Use libadwaita widgets**: Prefer `adw::` widgets over `gtk4::` when available
   - `adw::ApplicationWindow` over `gtk4::ApplicationWindow`
   - `adw::HeaderBar` over `gtk4::HeaderBar`
   - `adw::ActionRow` for list items
   - `adw::PreferencesGroup` for grouped settings

2. **Navigation**: Use `adw::NavigationView` for screen transitions
   - Each screen is an `adw::NavigationPage`
   - Set appropriate titles and tags
   - Use `can_pop(false)` for root pages

3. **Layout**: Use `adw::ToolbarView` for pages with header bars
   - Header bar goes in `add_top_bar()`
   - Content goes in `set_content()`
   - Bottom bars go in `add_bottom_bar()`

4. **Styling**: Use CSS classes for visual hierarchy
   - `title-1`, `title-2`, `title-3` for headings
   - `caption` for small text
   - `suggested-action` for primary buttons
   - `destructive-action` for dangerous actions
   - `pill` for rounded buttons
   - `boxed-list` for card-style lists
   - `card` for content containers

## Application Architecture

### State Management

The application uses shared state via `Rc<RefCell<AppState>>`:

```rust
struct AppState {
    api_client: ApiClient,
    config: Config,
    sessions: Vec<Session>,
    katas: Vec<Kata>,
}
```

- `Rc` allows multiple ownership
- `RefCell` allows interior mutability
- State is shared across all views

### Async Operations

GTK4 uses GLib's async runtime:

```rust
glib::spawn_future_local(clone!(
    #[weak] widget1,
    #[strong] state,
    async move {
        // Async code here
        let result = api_client.fetch_sessions().await;
        // Update UI on main thread
    }
));
```

- Use `glib::spawn_future_local()` for async operations
- Use `clone!` macro to capture variables
- `#[weak]` for widgets (prevents reference cycles)
- `#[strong]` for owned data like `Rc<RefCell<T>>`

### Widget Creation Patterns

**AdwNavigationView Pattern:**
```rust
let nav_view = adw::NavigationView::new();

let page = adw::NavigationPage::builder()
    .title("Page Title")
    .tag("page-tag")
    .child(&content)
    .can_pop(false)  // Only for root pages
    .build();

nav_view.add(&page);  // For root page
nav_view.push(&page); // For stacked pages
```

**ToolbarView Pattern:**
```rust
let toolbar_view = adw::ToolbarView::new();

let header_bar = adw::HeaderBar::new();
toolbar_view.add_top_bar(&header_bar);

let content = gtk4::ScrolledWindow::new();
toolbar_view.set_content(Some(&content));
```

**List Pattern:**
```rust
let list_box = gtk4::ListBox::new();
list_box.add_css_class("boxed-list");
list_box.set_selection_mode(gtk4::SelectionMode::None);

let row = adw::ActionRow::new();
row.set_title("Title");
row.set_subtitle("Subtitle");
list_box.append(&row);
```

**Form Pattern:**
```rust
let group = adw::PreferencesGroup::new();
group.set_title("Group Title");

let row = adw::ActionRow::new();
row.set_title("Option");

let switch = gtk4::Switch::new();
row.add_suffix(&switch);
row.set_activatable_widget(Some(&switch));

group.add(&row);
```

## Code Organization

- **main.rs**: Application entry point and all UI logic
  - `main()` - Entry point
  - `build_ui()` - Application setup
  - `AppState` - Shared application state
  - `show_authentication()` - Authentication screen
  - `show_session_list()` - Session list screen
  - `show_session_create()` - Session creation screen
  - Helper functions for building UI components

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

## UI Component Guidelines

### Headers and Navigation

**AdwHeaderBar:**
- Use for all pages via `adw::ToolbarView`
- Title is set on the `adw::NavigationPage`, not the header bar
- Add buttons with `pack_start()` and `pack_end()`
- Use icon names from the icon naming specification

```rust
let header_bar = adw::HeaderBar::new();

let new_button = gtk4::Button::from_icon_name("list-add-symbolic");
new_button.set_tooltip_text(Some("New Session"));
header_bar.pack_start(&new_button);
```

### Lists and Rows

**AdwActionRow:**
- Primary list item widget
- Set title and subtitle
- Add suffixes for badges, icons, switches
- Use `set_activatable(true)` for clickable rows

```rust
let row = adw::ActionRow::new();
row.set_title("Session Name");
row.set_subtitle("2024-01-15");

let badge = gtk4::Label::new(Some("badge"));
badge.add_css_class("caption");
row.add_suffix(&badge);

row.set_activatable(true);
```

### Buttons

**Primary Actions:**
```rust
let button = gtk4::Button::with_label("Create");
button.add_css_class("suggested-action");
button.add_css_class("pill");
```

**Destructive Actions:**
```rust
let button = gtk4::Button::with_label("Delete");
button.add_css_class("destructive-action");
```

**Icon Buttons:**
```rust
let button = gtk4::Button::from_icon_name("list-add-symbolic");
button.set_tooltip_text(Some("Add"));
```

### Dialogs and Status

**Loading State:**
```rust
let status_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
status_box.set_valign(gtk4::Align::Center);
status_box.set_halign(gtk4::Align::Center);

let label = gtk4::Label::new(Some("Loading..."));
label.add_css_class("title-3");
status_box.append(&label);
```

**Error Messages:**
```rust
let error_label = gtk4::Label::new(None);
error_label.add_css_class("error");
error_label.set_wrap(true);
error_label.set_visible(false);

// Show error
error_label.set_text("Error message");
error_label.set_visible(true);
```

### Forms and Input

**Text Entry:**
```rust
let entry = gtk4::Entry::new();
entry.set_placeholder_text(Some("Enter text..."));
```

**Text View (Multi-line):**
```rust
let text_view = gtk4::TextView::new();
text_view.set_wrap_mode(gtk4::WrapMode::Word);
text_view.add_css_class("card");

let scrolled = gtk4::ScrolledWindow::new();
scrolled.set_child(Some(&text_view));
```

**Switches:**
```rust
let switch = gtk4::Switch::new();
switch.set_valign(gtk4::Align::Center);

let row = adw::ActionRow::new();
row.add_suffix(&switch);
row.set_activatable_widget(Some(&switch));
```

## API Integration Guidelines

- All API calls go through the `ApiClient` in `api.rs`
- The backend API is a Phoenix application running on `http://localhost:4000/api` by default
- API responses follow the format: `{ "data": [...] }`
- Use the `ApiResponse<T>` generic type for unmarshaling API responses
- **Always** handle API errors gracefully
- **Always** wrap API calls in async blocks with `glib::spawn_future_local()`
- Use `reqwest` for HTTP requests
- Include Bearer token in `Authorization` header for authenticated endpoints

### API Client Pattern

```rust
let api_client = state.borrow().api_client.clone();

glib::spawn_future_local(clone!(
    #[strong] state,
    #[weak] widget,
    async move {
        match api_client.fetch_sessions().await {
            Ok(sessions) => {
                state.borrow_mut().sessions = sessions.clone();
                // Update UI
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                // Show error to user
            }
        }
    }
));
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
- Display errors in the UI with appropriate styling
- Log errors with `eprintln!` for debugging
- Use `error` CSS class for error labels

## Type Definitions

- Use `#[derive(Debug, Clone, Serialize, Deserialize)]` for data models
- Add JSON tags with `serde`: `#[serde(skip_serializing_if = "Option::is_none")]`
- Use `Option<T>` for optional fields
- Use `Rc<RefCell<T>>` for shared mutable state

## Data Handling Guidelines

- Sessions are sorted by `practiced_at` date in descending order (newest first)
- Use `chrono::DateTime<Utc>` for datetime fields
- Format datetime with `.format("%Y-%m-%d")`
- Sort with `.sort_by()` and custom comparison

## CSS Classes and Styling

### Typography
- `title-1` - Largest heading
- `title-2` - Section heading
- `title-3` - Subsection heading
- `caption` - Small supporting text

### Components
- `pill` - Rounded corners (buttons, badges)
- `card` - Content container with background
- `boxed-list` - Card-style list container

### Actions
- `suggested-action` - Primary action (blue)
- `destructive-action` - Dangerous action (red)
- `error` - Error text (red)
- `warning` - Warning (yellow/orange)
- `success` - Success (green)
- `accent` - Accent color (blue)

### Icon Names

Use standard icon names from the icon naming specification:
- `list-add-symbolic` - Add/new
- `view-refresh-symbolic` - Refresh
- `go-next-symbolic` - Navigate forward
- `go-previous-symbolic` - Navigate back
- `open-menu-symbolic` - Menu
- `emblem-default-symbolic` - Checkmark

## Testing Guidelines (TODO)

This project currently has no tests. When adding tests:

- Use Rust's built-in `#[cfg(test)]` and `#[test]`
- Name test modules `mod tests`
- Use `assert_eq!`, `assert!`, etc. for assertions
- Mock HTTP calls for API client tests
- Run tests with `cargo test`
- Use `cargo test --verbose` for detailed output

## Dependencies

- **gtk4** - GTK4 bindings for Rust
- **libadwaita** - Modern GNOME widgets
- **glib** - GLib bindings for async/signals
- **reqwest** - HTTP client for API requests
- **serde** / **serde_json** - JSON serialization/deserialization
- **tokio** - Async runtime (used by reqwest)
- **directories** - XDG config directory support
- **chrono** - DateTime handling

## GNOME HIG Compliance

This application follows GNOME Human Interface Guidelines:

- **Responsive**: Uses libadwaita adaptive widgets
- **Consistent**: Uses standard GNOME patterns and widgets
- **Accessible**: Proper labels, tooltips, and keyboard navigation
- **Modern**: Uses latest libadwaita patterns (NavigationView, etc.)

Key HIG principles applied:
- Clear visual hierarchy with typography classes
- Consistent spacing and margins
- Primary actions are visually emphasized
- Destructive actions require confirmation (TODO)
- Loading states provide feedback
- Error messages are clear and actionable

## Current Features

- ✅ Device flow authentication with OAuth2-style flow
- ✅ Bearer token API authentication
- ✅ Token persistence in config file
- ✅ View list of training sessions
- ✅ Session creation with form UI
- ✅ Color-coded kata level badges
- ✅ Loading states
- ✅ Error display
- ✅ Logout functionality
- ✅ GNOME HIG-compliant UI
- ✅ Modern libadwaita design

## Known Limitations

- No session editing (only create and view)
- No session deletion
- No session detail view (expandable rows not yet implemented)
- No session filtering or search
- No offline mode or caching
- Config file is plain JSON (token not encrypted)
- No session statistics or analytics
- No markdown rendering for notes
- Session list doesn't auto-refresh after creation

## TODO

- [ ] Add unit tests for API client functions
- [ ] Add integration tests for authentication flow
- [ ] Implement session detail view (expandable rows or separate page)
- [ ] Implement session editing
- [ ] Implement session deletion with confirmation dialog
- [ ] Add search/filter functionality
- [ ] Add keyboard shortcuts
- [ ] Improve error messages and recovery
- [ ] Add session statistics view
- [ ] Add markdown rendering for notes in detail view
- [ ] Add session export functionality
- [ ] Add preferences/settings screen
- [ ] Consider token encryption in config file
- [ ] Add toast notifications for actions
- [ ] Implement proper error dialogs
- [ ] Add loading spinners for long operations
- [ ] Auto-refresh session list after creation
- [ ] Add placeholder page for empty session list

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

# Binary will be at: target/release/gtkata
./target/release/gtkata
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

## System Requirements

- Linux (primary target)
- GTK4 4.12 or later
- libadwaita 1.5 or later
- Rust 2024 edition toolchain

### Installing GTK4 and libadwaita on Linux

**Arch Linux:**
```bash
sudo pacman -S gtk4 libadwaita
```

**Ubuntu/Debian:**
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
```

**Fedora:**
```bash
sudo dnf install gtk4-devel libadwaita-devel
```

## Troubleshooting

### Build Issues

- Ensure GTK4 and libadwaita development packages are installed
- Ensure Rust toolchain is up to date: `rustup update`
- Clean build artifacts: `cargo clean && cargo build`
- Check for conflicting dependencies: `cargo tree`

### Runtime Issues

- Verify backend is running on port 4000
- Check API URL configuration
- Check config file: `~/.config/katanaute/config.json`
- Remove config file to reset authentication

### GTK/Display Issues

- GTK4 requires a display server (X11 or Wayland)
- On Wayland, ensure `libadwaita` is properly installed
- Check GTK theme is compatible with libadwaita
- Try running with `GTK_DEBUG=interactive` for inspector

## Resources

- GTK4 Documentation: https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/
- libadwaita Documentation: https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/stable/latest/libadwaita/
- GNOME HIG: https://developer.gnome.org/hig
- Icon Naming: https://specifications.freedesktop.org/icon-naming-spec/latest/
