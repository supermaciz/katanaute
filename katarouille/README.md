# Katarouille - Kata Training Tracker GUI

A graphical user interface application for tracking Uechi-Ryu Karate kata training sessions, built with Rust and Iced.

## Features

- **Device Flow Authentication** - Secure OAuth2-style authentication for easy login
- **Session Management** - View all training sessions with dates and kata information
- **Session Creation** - Create new training sessions with notes and kata selection
- **Color-Coded Levels** - Visual kata level badges (yellow, orange, green, blue, brown, shodan)
- **Dark Theme** - Modern dark UI theme
- **Persistent Configuration** - Automatic token storage in XDG-compliant config directory

## Screenshots

*(GUI screenshots would go here)*

## Prerequisites

- Rust 1.85+ (uses 2024 edition)
- Phoenix backend running on `http://localhost:4000` (see [katanaute/](../katanaute/))
- OpenGL or Vulkan support for GUI rendering

## Installation

### From Source

```bash
# Clone the repository (if not already cloned)
git clone <repository-url>
cd katanaute/katarouille

# Build the application
cargo build --release

# The binary will be at target/release/katarouille
```

## Usage

### Quick Start

```bash
# Run the application
cargo run

# Or run the compiled binary
./target/release/katarouille
```

### First Time Setup

1. Click the "Login" button
2. A user code and verification URL will be displayed
3. Open the verification URL in your browser
4. Log in with your account (or create one)
5. Enter the user code shown in the application
6. Once authorized, the application will automatically load your sessions

### Using the Application

**Session List View**
- View all training sessions sorted by date (newest first)
- Click on a session to expand and view notes
- Click "New Session" to create a session
- Click "Refresh" to reload sessions from server
- Click "Logout" to clear authentication

**Create Session View**
- Select a kata from the list
- Optionally add notes in Markdown format
- Toggle "Part of Course" to track structured learning
- Click "Create Session" to save
- Click "Cancel" to return to session list

## Configuration

### Environment Variables

- `KATANAUTE_API_URL` - Override the backend API URL (default: `http://localhost:4000/api`)

### Config File

Configuration is stored at:
- Linux: `~/.config/katanaute/config.json`
- macOS: `~/Library/Application Support/katanaute/config.json`
- Windows: `%APPDATA%\katanaute\config.json`

Example config:
```json
{
  "api_token": "your_token_here",
  "base_url": "http://localhost:4000/api"
}
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests (when implemented)
cargo test
```

### Project Structure

```
src/
├── main.rs      # Application entry point and UI logic
├── api.rs       # API client for backend communication
├── auth.rs      # Device flow authentication logic
├── config.rs    # Configuration management
└── models.rs    # Data models (Kata, Session, etc.)
```

## Architecture

Katarouille uses the Elm Architecture (Model-View-Update) pattern via Iced:

- **Model**: Application state (`AppState` enum)
- **View**: UI rendering based on current state
- **Update**: Message handling and state transitions

### Application States

1. **Loading** - Initial state and during data fetching
2. **Authentication** - Device flow login screen
3. **SessionList** - Main session list view
4. **SessionCreate** - Session creation form

## Troubleshooting

### Build Errors

**Missing OpenGL/Vulkan**
```bash
# On Ubuntu/Debian
sudo apt-get install libxkbcommon-dev libwayland-dev

# On Fedora
sudo dnf install libxkbcommon-devel wayland-devel
```

**Outdated Rust**
```bash
rustup update
```

### Runtime Errors

**Backend Connection Failed**
- Ensure Phoenix backend is running: `cd ../katanaute && mix phx.server`
- Check API URL in config or environment variable

**Authentication Failed**
- Remove config file to reset: `rm ~/.config/katanaute/config.json`
- Try logging in again

**GUI Won't Start**
- Check graphics driver support for OpenGL/Vulkan
- On Wayland, try X11: `WINIT_UNIX_BACKEND=x11 cargo run`

## Contributing

For development guidelines, see [CLAUDE.md](./CLAUDE.md).

Key points:
- Follow Rust conventions and best practices
- Use `cargo fmt` and `cargo clippy` before committing
- Use conventional commits with `katarouille` scope
- Write tests for new features

## Tech Stack

- **Rust** - Systems programming language
- **Iced** - Cross-platform GUI framework (Elm Architecture)
- **Reqwest** - HTTP client for API requests
- **Serde** - Serialization/deserialization
- **Tokio** - Async runtime
- **Chrono** - DateTime handling

## Comparison with Other Clients

- **React Frontend (katareact)** - Full-featured web interface with editing
- **Go TUI (katago)** - Terminal UI for quick command-line access
- **Katarouille** - Native GUI application with offline capability (token persistence)

## License

See the main repository [LICENSE](../LICENSE) file.

## Related Projects

- [katanaute](../katanaute/) - Phoenix backend API
- [katareact](../katareact/) - React web frontend
- [katago](../katago/) - Go terminal UI client
