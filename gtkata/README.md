# GTKata - Kata Training Tracker Desktop Client

A beautiful, native desktop application for tracking your Karate kata training sessions.

## Features

- **Modern GTK4 Interface**: Clean, native desktop UI that integrates with your system
- **Secure Authentication**: Email/password login or device flow authorization
- **Session Management**: Create and view training sessions with notes
- **Kata Catalog**: Browse all katas from Yellow belt to Shodan
- **Markdown Notes**: Rich text support for session notes
- **Secure Storage**: Tokens stored securely in system keyring

## Installation

### Prerequisites

#### Linux

**Debian/Ubuntu:**
```bash
sudo apt install libgtk-4-dev build-essential libssl-dev pkg-config
```

**Fedora:**
```bash
sudo dnf install gtk4-devel gcc openssl-devel pkg-config
```

**Arch Linux:**
```bash
sudo pacman -S gtk4 base-devel openssl pkg-config
```

#### macOS

Install GTK4 via Homebrew:
```bash
brew install gtk4
```

#### Windows

Download and install GTK4 runtime from: https://github.com/wingtk/gvsbuild

### Building from Source

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository and build**:
   ```bash
   cd katanaute/gtkata
   cargo build --release
   ```

3. **Run**:
   ```bash
   cargo run --release
   ```

The binary will be located at `target/release/gtkata`.

## Usage

### First Run

1. Start GTKata
2. Choose authentication method:
   - **Email Login**: Enter your email and password, then click "Login" or "Register"
   - **Device Flow**: Click "Start Device Flow", then open the link in your browser to authorize

### Creating Sessions

1. Click "+ New Session" in the toolbar
2. Select a kata from the dropdown
3. Check "Part of structured learning path" if applicable
4. Add notes in Markdown format (optional)
5. Click "Create"

### Managing Sessions

- **View sessions**: Sessions are displayed in the main list
- **Refresh**: Click "Refresh" to update the session list
- **Logout**: Click "Logout" to sign out (clears stored token)

## Configuration

GTKata connects to the backend API at `http://localhost:4000/api` by default.

To use a different backend URL:

```bash
export KATANAUTE_API_URL=https://your-backend-url.com/api
gtkata
```

## Troubleshooting

### "Failed to connect to API"

- Ensure the backend server is running on `localhost:4000`
- Check your network connection
- Verify the API URL with `echo $KATANAUTE_API_URL`

### "Authentication failed"

- Check your email and password
- Ensure you have an account (use "Register" to create one)
- Try the device flow authentication method instead

### GTK4 not found (Linux)

Install GTK4 development libraries:
```bash
# Debian/Ubuntu
sudo apt install libgtk-4-dev

# Fedora
sudo dnf install gtk4-devel

# Arch
sudo pacman -S gtk4
```

### Blank window or crashes (macOS)

Ensure GTK4 is properly installed:
```bash
brew reinstall gtk4
```

## Development

See [CLAUDE.md](./CLAUDE.md) for detailed development documentation.

### Quick Development Setup

```bash
# Install dependencies (Linux - Debian/Ubuntu)
sudo apt install libgtk-4-dev build-essential libssl-dev pkg-config

# Run in development mode
cargo run

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Check for issues
cargo clippy
```

## License

Part of the Katanaute project. See root LICENSE file.

## Support

For issues and feature requests, please use the project's issue tracker.

## Screenshots

*(Coming soon)*

## Related Projects

- **katanaute**: Phoenix backend API
- **katareact**: React web frontend
- **katago**: Terminal UI client

See the root [README.md](../README.md) for more information about the Katanaute project.
