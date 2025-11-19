# Katafyne - GUI Client for Katanaute

A native cross-platform GUI client for the Katanaute kata training tracker, built with Go and Fyne.

## Features

- 🔐 **Device Flow Authentication** - Secure OAuth2-style authentication for desktop apps
- 📱 **Native GUI** - Cross-platform native interface using Fyne
- 📋 **Session Management** - View and create training sessions
- 🥋 **Kata Library** - Browse available katas with level indicators
- 💾 **Token Persistence** - Automatic token storage in XDG-compliant config directory
- 🎨 **Clean Interface** - Modern, intuitive split-pane layout

## Screenshots

### Login Screen
The device flow authentication presents a user code and verification URL.

### Main View
- Left pane: List of training sessions sorted by date
- Right pane: Detailed view of selected session
- Buttons: Refresh, Add Session, Logout

## Installation

### Prerequisites

- Go 1.18 or later
- Fyne dependencies (see below)

### Linux Dependencies

On Linux, you'll need the following packages:

```bash
# Debian/Ubuntu
sudo apt-get install gcc libgl1-mesa-dev xorg-dev

# Fedora
sudo dnf install gcc libXcursor-devel libXrandr-devel mesa-libGL-devel libXi-devel libXinerama-devel libXxf86vm-devel

# Arch
sudo pacman -S go gcc libxcursor libxrandr libxinerama libxi mesa
```

### Build

```bash
cd katafyne
go get fyne.io/fyne/v2
go mod tidy
go build
```

### Run

```bash
./katafyne
```

Or directly:

```bash
go run .
```

## Configuration

### API URL

By default, Katafyne connects to `http://localhost:4000/api`.

Override with environment variable:

```bash
export KATANAUTE_API_URL=https://your-server.com/api
./katafyne
```

### Token Storage

Authentication tokens are stored in:
- Linux: `~/.config/katanaute/config.json`
- macOS: `~/Library/Application Support/katanaute/config.json`
- Windows: `%APPDATA%\katanaute\config.json`

The config file contains:
```json
{
  "api_token": "your-token-here",
  "base_url": "http://localhost:4000/api"
}
```

## Usage

### First Launch

1. Click "Login with Device Flow"
2. A verification URL and user code will be displayed
3. Visit the URL in your browser
4. Enter the user code when prompted
5. Approve the authorization request
6. The app will automatically detect authorization and proceed

### Viewing Sessions

- Sessions are listed on the left, sorted by date (newest first)
- Click a session to view details on the right
- Sessions show:
  - Kata name and level badge
  - Practice date and time
  - Course indicator (📚 if part of structured course)
  - Notes (Markdown format)

### Creating a Session

1. Click "Add Session"
2. Select a kata from the dropdown
3. Enter training notes (Markdown supported)
4. Check "Part of structured course" if applicable
5. Click "Create"

### Logout

Click "Logout" to clear your token and return to the login screen.

## Architecture

Katafyne follows a simple MVC-like architecture:

```
main.go     - Application entry point and UI logic
api.go      - API client (FetchSessions, CreateSession, FetchKatas)
auth.go     - Device flow authentication
config.go   - Configuration and token persistence
models.go   - Data structures (Session, Kata, User)
```

### Device Flow Sequence

1. **Initiate**: `POST /api/auth/device/code`
   - Receive device_code and user_code
2. **Display**: Show user_code and verification_uri to user
3. **Poll**: `POST /api/auth/device/token` with device_code
   - Poll every 5 seconds until authorized
4. **Store**: Save access_token to config file
5. **Use**: Include token in all subsequent API requests

## Development

### Code Style

```bash
go fmt ./...
```

### Testing

(TODO: Add tests)

```bash
go test ./...
```

## Comparison with Other Clients

| Feature | Katafyne (Go + Fyne) | Katarouille (Rust + Iced) | Katago (Go + Bubble Tea) |
|---------|---------------------|--------------------------|-------------------------|
| Platform | GUI (cross-platform) | GUI (cross-platform) | TUI (terminal) |
| Language | Go | Rust | Go |
| Auth | Device Flow | Device Flow | Device Flow |
| View Sessions | ✅ | ✅ | ✅ |
| Create Sessions | ✅ | ✅ | ✅ |
| Edit Sessions | ❌ | ❌ | ❌ |
| Delete Sessions | ❌ | ❌ | ❌ |
| Offline Mode | ❌ | ✅ | ❌ |

## Troubleshooting

### App won't start - missing dependencies

Install platform-specific dependencies (see Installation section above).

### Authentication fails

- Ensure backend is running at the configured URL
- Check that you entered the correct user code
- Verify you're logged in to the web interface before authorizing

### Sessions not loading

- Verify backend is accessible
- Check token is valid (try logging out and back in)
- Look for error dialogs in the app

### Build errors

```bash
# Clean and rebuild
go clean
go mod tidy
go build
```

## Contributing

When contributing to Katafyne:

1. Follow Go conventions and `go fmt`
2. Keep UI logic in `main.go`
3. Keep API logic in `api.go`
4. Test with backend running locally
5. Update this README for new features

## License

Part of the Katanaute monorepo. See main repository for license information.

## Related Projects

- **katanaute** - Phoenix backend (Elixir)
- **katareact** - React web frontend
- **katarouille** - Rust GUI client (Iced)
- **katago** - Go terminal UI client (Bubble Tea)
