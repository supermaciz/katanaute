# GTKata - GNOME Kata Training Tracker

A native Linux GUI application for tracking kata training sessions, built with GTK4 and libadwaita following GNOME Human Interface Guidelines.

![GTKata](https://img.shields.io/badge/GTK-4-blue)
![libadwaita](https://img.shields.io/badge/libadwaita-1.5-purple)
![Rust](https://img.shields.io/badge/Rust-2024-orange)

## Features

- 🔐 **Secure Authentication** - Device flow OAuth2-style authentication
- 📝 **Session Management** - Create and view training sessions
- 🎨 **Modern GNOME Design** - libadwaita widgets and styling
- 🎯 **Kata Tracking** - Track practice across different belt levels
- 💾 **Persistent Config** - XDG-compliant configuration storage
- 🌐 **API Integration** - RESTful API communication with Phoenix backend

## Screenshots

*Coming soon*

## Installation

### Prerequisites

#### System Dependencies

**Arch Linux:**
```bash
sudo pacman -S gtk4 libadwaita rust
```

**Ubuntu/Debian:**
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Fedora:**
```bash
sudo dnf install gtk4-devel libadwaita-devel rust cargo
```

### Building from Source

```bash
# Clone the repository
cd katanaute/gtkata

# Build the application
cargo build --release

# Run the application
./target/release/gtkata
```

## Usage

### First Run

1. Start the backend server (see main README)
2. Launch GTKata: `cargo run` or `./target/release/gtkata`
3. Click "Login" to start device flow authentication
4. Visit the verification URL in your browser
5. Enter the user code shown in the app
6. Approve the authorization request
7. GTKata will automatically log you in

### Creating a Session

1. Click the "+" button in the header bar
2. Select a kata from the list
3. Add optional notes (supports Markdown)
4. Toggle "Part of Course" if this is structured training
5. Click "Create Session"

### Viewing Sessions

- Sessions are displayed in a list, newest first
- Each session shows:
  - Kata name
  - Practice date
  - Belt level badge (color-coded)
  - Course indicator (if applicable)

### Logout

1. Click the menu button (⋮) in the header bar
2. Select "Logout"

## Configuration

Configuration is stored in `~/.config/katanaute/config.json`:

```json
{
  "api_token": "your_token_here",
  "base_url": "http://localhost:4000/api"
}
```

### Environment Variables

- `KATANAUTE_API_URL` - Override the default API URL

Example:
```bash
KATANAUTE_API_URL=http://localhost:4000/api cargo run
```

## Architecture

GTKata follows GNOME design patterns:

- **AdwNavigationView** - Screen navigation with transitions
- **AdwToolbarView** - Consistent header bar layout
- **AdwActionRow** - List items with accessories
- **AdwPreferencesGroup** - Grouped form controls

### Code Structure

```
gtkata/
├── src/
│   ├── main.rs       # UI and application logic
│   ├── api.rs        # HTTP API client
│   ├── auth.rs       # Device flow authentication
│   ├── config.rs     # Configuration management
│   └── models.rs     # Data models
├── Cargo.toml        # Dependencies
├── CLAUDE.md         # Development guidelines
└── README.md         # This file
```

## Development

### Building

```bash
cargo build
```

### Running

```bash
cargo run
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

### Development Guidelines

See [CLAUDE.md](CLAUDE.md) for comprehensive development guidelines including:
- GTK4 and libadwaita patterns
- GNOME HIG compliance
- Async operation handling
- Widget creation patterns
- API integration
- Error handling

## Dependencies

- **gtk4** (0.9) - GTK4 Rust bindings
- **libadwaita** (0.7) - Modern GNOME widgets
- **glib** (0.20) - GLib async/signals
- **reqwest** (0.12) - HTTP client
- **serde** (1.0) - JSON serialization
- **tokio** (1.0) - Async runtime
- **directories** (6.0) - XDG directories
- **chrono** (0.4) - Date/time handling

## GNOME HIG Compliance

GTKata follows GNOME Human Interface Guidelines:

✅ **Consistent** - Uses standard GNOME patterns and widgets  
✅ **Clear** - Visual hierarchy with typography classes  
✅ **Accessible** - Proper labels, tooltips, keyboard navigation  
✅ **Modern** - Latest libadwaita patterns (NavigationView, etc.)  
✅ **Responsive** - Adaptive layouts with proper spacing  

See: https://developer.gnome.org/hig

## Comparison with Other Clients

| Feature | GTKata | Katarouille | Katafyne | Katago |
|---------|--------|-------------|----------|--------|
| Framework | GTK4/libadwaita | Iced | Fyne | Bubble Tea |
| Platform | Linux (GNOME) | Cross-platform | Cross-platform | Terminal |
| Design | GNOME HIG | Custom dark | Clean modern | TUI |
| Auth | Device flow | Device flow | Device flow | Device flow |
| Sessions | View, Create | View, Create | View, Create | View, Create |

## Roadmap

- [ ] Session detail view
- [ ] Session editing
- [ ] Session deletion with confirmation
- [ ] Search and filter
- [ ] Keyboard shortcuts
- [ ] Statistics dashboard
- [ ] Markdown preview for notes
- [ ] Toast notifications
- [ ] Preferences dialog
- [ ] Session export

## Contributing

See [CLAUDE.md](CLAUDE.md) for development guidelines.

1. Follow Rust 2024 edition conventions
2. Use `cargo fmt` before committing
3. Run `cargo clippy` to check for issues
4. Follow GNOME HIG patterns
5. Use conventional commits: `feat(gtkata):`, `fix(gtkata):`, etc.

## License

Part of the Katanaute project. See main repository for license information.

## Resources

- [GTK4 Documentation](https://gtk-rs.org/gtk4-rs/)
- [libadwaita Documentation](https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/)
- [GNOME HIG](https://developer.gnome.org/hig)
- [Katanaute Repository](../)

## Troubleshooting

### Application won't start

- Ensure GTK4 and libadwaita are installed
- Check display server is running (X11 or Wayland)
- Try: `GTK_DEBUG=interactive cargo run`

### Can't connect to backend

- Verify backend is running: `http://localhost:4000`
- Check API URL in config or env var
- Check network connectivity

### Build fails

- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`
- Install system dependencies (see Installation)

### Authentication fails

- Check backend is running and accessible
- Verify you can access the verification URL
- Try removing config file: `rm ~/.config/katanaute/config.json`

## Support

For issues, questions, or contributions, see the main Katanaute repository.
