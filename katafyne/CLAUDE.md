# Katafyne Development Guidelines

Guidelines for developing and maintaining Katafyne, the Go + Fyne GUI client for Katanaute.

## Technology Stack

- **Language**: Go 1.18+
- **GUI Framework**: Fyne v2 (https://fyne.io/)
- **Architecture**: MVC-like with separate concerns
- **Configuration**: XDG Base Directory specification

## Project Structure

```
katafyne/
├── main.go       # Application entry point, UI logic, views
├── api.go        # API client for backend communication
├── auth.go       # Device flow authentication
├── config.go     # Configuration and token persistence
├── models.go     # Data structures (Session, Kata, User)
├── go.mod        # Go module definition
├── go.sum        # Dependency checksums
├── README.md     # User documentation
└── CLAUDE.md     # This file - development guidelines
```

## Fyne Framework Overview

### Key Concepts

**1. Application and Window**
```go
fyneApp := app.New()
mainWindow := fyneApp.NewWindow("Title")
mainWindow.ShowAndRun()
```

**2. Widgets** (UI components)
- `widget.NewLabel()` - Text display
- `widget.NewButton()` - Clickable button
- `widget.NewEntry()` - Text input
- `widget.NewList()` - Scrollable list
- `widget.NewSelect()` - Dropdown menu
- `widget.NewCheck()` - Checkbox

**3. Containers** (layout)
- `container.NewVBox()` - Vertical stack
- `container.NewHBox()` - Horizontal stack
- `container.NewBorder()` - Border layout (top, bottom, left, right, center)
- `container.NewHSplit()` - Horizontal split pane
- `container.NewScroll()` - Scrollable content

**4. Dialogs**
- `dialog.ShowInformation()` - Info popup
- `dialog.ShowError()` - Error popup
- `dialog.ShowForm()` - Form dialog

### Fyne Best Practices

1. **State Management**: Store app state in a struct
   ```go
   type App struct {
       config     *Config
       fyneApp    fyne.App
       mainWindow fyne.Window
       sessions   []*Session
       katas      []*Kata
   }
   ```

2. **Async Operations**: Use goroutines for network calls
   ```go
   go func() {
       result := FetchSessions(config)
       // Update UI on main thread if needed
   }()
   ```

3. **Updating UI**: Call `Refresh()` on widgets after data changes
   ```go
   sessionList.Refresh()
   ```

4. **Layout**: Use border layout for structured views
   ```go
   content := container.NewBorder(
       topWidget,    // Header
       bottomWidget, // Footer
       leftWidget,   // Sidebar
       rightWidget,  // Aside
       centerWidget, // Main content
   )
   ```

## Code Organization

### main.go

Contains the application entry point and all UI-related code:

- `main()` - Entry point, initializes app
- `App` struct - Application state
- `showLoginView()` - Authentication screen
- `showMainView()` - Main application screen
- `showCreateSessionDialog()` - Session creation form
- `refreshData()` - Fetch data from API
- `logout()` - Clear token and return to login

**UI Pattern**:
1. Create widgets
2. Define callbacks/handlers
3. Arrange in containers
4. Set as window content

### api.go

API client for communicating with the Katanaute backend:

- `createAuthRequest()` - Helper to create authenticated HTTP requests
- `FetchSessions()` - GET /sessions (requires auth)
- `CreateSession()` - POST /sessions (requires auth)
- `FetchKatas()` - GET /katas (requires auth)

**Request Pattern**:
```go
req, err := createAuthRequest("GET", url, token, nil)
client := &http.Client{}
resp, err := client.Do(req)
defer resp.Body.Close()
// Parse response
```

### auth.go

Device flow authentication implementation:

- `InitiateDeviceFlow()` - Start device flow, get user_code
- `PollForToken()` - Poll for authorization completion
- `AuthenticateWithDeviceFlow()` - Complete flow with progress callback

**Flow**:
1. POST /auth/device/code → get device_code, user_code, verification_uri
2. Display user_code and verification_uri to user
3. Poll POST /auth/device/token with device_code
4. Return access_token when authorized

### config.go

Configuration and token persistence:

- `GetConfigDir()` - XDG-compliant config directory
- `LoadConfig()` - Read config from disk
- `SaveConfig()` - Write config to disk
- `NewConfig()` - Initialize runtime config

**Config Priority**:
1. Environment variable `KATANAUTE_API_URL`
2. Saved config file
3. Default: `http://localhost:4000/api`

### models.go

Data structures matching backend JSON:

- `Kata` - Kata definition
- `Session` - Training session
- `SessionInput` - Session creation payload
- `User` - User information

## Authentication Flow

### Initial Launch (No Token)

1. Show login view
2. User clicks "Login with Device Flow"
3. Call `InitiateDeviceFlow()` → receive user_code
4. Display user_code and verification_uri
5. Poll `PollForToken()` every 5 seconds
6. On success: save token, show main view

### Authenticated Launch

1. Load config → has token
2. Show main view immediately
3. Fetch sessions and katas

### Token Persistence

Tokens are stored in `~/.config/katanaute/config.json`:
```json
{
  "api_token": "the-bearer-token",
  "base_url": "http://localhost:4000/api"
}
```

## API Integration

### Authentication Header

All authenticated requests include:
```
Authorization: Bearer <token>
```

### Response Format

All API responses use this format:
```json
{
  "data": [ ... ]
}
```

Use the generic `Data[T]` type:
```go
type Data[T Session | Kata] struct {
    Data []*T `json:"data"`
}
```

### Error Handling

1. Check HTTP status code
2. Return descriptive errors
3. Show error dialogs to user

```go
if resp.StatusCode == http.StatusUnauthorized {
    return nil, fmt.Errorf("unauthorized: please login first")
}
```

## UI Guidelines

### Layout Structure

**Login View**:
- Centered title
- Instructions
- Login button
- Status label

**Main View**:
- Split pane (50/50)
- Left: Session list with header and buttons
- Right: Session detail panel

### User Feedback

1. **Dialogs** for errors and confirmations
   ```go
   dialog.ShowError(err, window)
   dialog.ShowInformation("Success", "Session created!", window)
   ```

2. **Status labels** for async operations
   ```go
   statusLabel.SetText("Loading...")
   ```

3. **Disable buttons** during operations
   ```go
   button.Disable()
   // ... do work ...
   button.Enable()
   ```

### Data Presentation

**Session List Items**:
- Line 1: Kata name [level] (bold)
- Line 2: Date + course indicator (📚)

**Session Details**:
```
Kata: Name (level)
Date: YYYY-MM-DD HH:MM
In Course: true/false

Notes:
Markdown notes here...
```

## Development Workflow

### Setup

```bash
cd katafyne
go get fyne.io/fyne/v2
go mod tidy
```

### Running

```bash
# Direct run
go run .

# With custom API URL
KATANAUTE_API_URL=http://localhost:4000/api go run .

# Build and run
go build
./katafyne
```

### Code Quality

```bash
# Format code
go fmt ./...

# Vet for issues
go vet ./...

# Build
go build
```

## Common Development Tasks

### Adding a New API Endpoint

1. Add function to `api.go`
2. Define request/response types in `models.go`
3. Use `createAuthRequest()` helper
4. Handle errors appropriately

### Adding a New View/Screen

1. Add method to `App` struct in `main.go`
2. Create widgets and containers
3. Set as window content: `a.mainWindow.SetContent(content)`

### Adding a New Dialog

```go
func (a *App) showMyDialog() {
    // Create form items
    entry := widget.NewEntry()
    items := []*widget.FormItem{
        {Text: "Field", Widget: entry},
    }

    // Show dialog
    dialog.ShowForm("Title", "OK", "Cancel", items, func(ok bool) {
        if ok {
            // Handle submission
        }
    }, a.mainWindow)
}
```

## Testing Strategy (TODO)

Currently, Katafyne has no automated tests. Future testing should cover:

### Unit Tests
- `api.go` - Mock HTTP responses
- `auth.go` - Mock device flow
- `config.go` - Test file operations

### Integration Tests
- Full authentication flow
- API client with test server

### UI Tests
- Fyne provides `test.NewApp()` for UI testing

## Troubleshooting

### Compilation Issues

**Missing Fyne dependencies**:
```bash
go get fyne.io/fyne/v2
go mod tidy
```

**Platform-specific build issues**:
- Linux: Install `gcc`, `libgl1-mesa-dev`, `xorg-dev`
- macOS: Install Xcode command line tools
- Windows: Install GCC (e.g., TDM-GCC)

### Runtime Issues

**App won't start**:
- Check platform dependencies installed
- Run with `go run .` to see error output

**Network errors**:
- Verify backend is running
- Check `KATANAUTE_API_URL` configuration
- Test API manually: `curl http://localhost:4000/api/katas`

**Authentication fails**:
- Verify backend device flow is enabled
- Check you're entering the user code correctly
- Ensure you approve the request in time (expires in 15 min)

## Known Limitations

1. **No session editing** - Only create and view
2. **No session deletion** - Read-only list
3. **No offline mode** - Unlike Katarouille (Rust client)
4. **No search/filter** - All sessions shown
5. **No pagination** - Loads all sessions at once

## Future Enhancements

### High Priority
- [ ] Session editing
- [ ] Session deletion
- [ ] Better error recovery UI

### Medium Priority
- [ ] Search/filter sessions by kata
- [ ] Sort options (date, kata, course)
- [ ] Session statistics view
- [ ] Markdown preview for notes

### Low Priority
- [ ] Custom themes
- [ ] Keyboard shortcuts
- [ ] Export sessions to CSV/JSON
- [ ] Offline session caching

## Comparison with Katarouille

Both are native GUI clients, but:

| Aspect | Katafyne (Go + Fyne) | Katarouille (Rust + Iced) |
|--------|---------------------|--------------------------|
| Language | Go | Rust |
| Framework | Fyne (simpler) | Iced (Elm architecture) |
| Architecture | MVC-like | MVU (Model-View-Update) |
| State Management | Struct fields | Elm messages |
| Async | Goroutines | Tokio async/await |
| Complexity | Lower | Higher |
| Type Safety | Moderate | Strong |
| Offline Support | No | Yes |

**When to use Katafyne**:
- Prefer Go over Rust
- Want simpler codebase
- Don't need offline support
- Rapid prototyping

**When to use Katarouille**:
- Prefer Rust
- Need strong type safety
- Want offline capability
- Prefer Elm architecture

## Resources

- **Fyne Documentation**: https://docs.fyne.io/
- **Fyne Examples**: https://github.com/fyne-io/examples
- **Go Documentation**: https://go.dev/doc/
- **Katanaute API**: See `katanaute/CLAUDE.md`

## Commit Conventions

Use conventional commits with `katafyne` scope:

```
feat(katafyne): add session deletion
fix(katafyne): handle auth errors gracefully
docs(katafyne): update README with screenshots
test(katafyne): add API client tests
```

## Contributing

When modifying Katafyne:

1. **Keep it simple** - Fyne is meant to be straightforward
2. **Separate concerns** - UI in `main.go`, API in `api.go`, etc.
3. **Handle errors** - Show user-friendly dialogs
4. **Update docs** - Keep README.md and this file current
5. **Test manually** - Run against local backend
6. **Format code** - Always run `go fmt`

## Questions?

See main repository's `CLAUDE.md` for overall project guidelines and architecture.
