This is a terminal user interface (TUI) application written in Go using the Bubble Tea framework.

## Project guidelines

- Use conventional commits with `katago` scope for all commits (e.g., `feat(katago):`, `fix(katago):`, `test(katago):`)
- Run `go build` to ensure the code compiles before committing
- Format code with `go fmt` before committing
- Use existing patterns from the codebase when adding features
- All new features should eventually have corresponding tests (see TODO section)

### Go guidelines

- This project uses **Go 1.25+**
- **Always** use `go fmt` to format code
- **Always** handle errors explicitly - never ignore error return values
- Use meaningful variable names - avoid single-letter names except in short scopes (loops, closures)
- **Always** use `defer` for cleanup operations (like `defer resp.Body.Close()`)
- Prefer composition over inheritance - Go doesn't have classes
- **Never** use panic for normal error handling - return errors instead
- Use `log.Println()` for debugging (logs go to `debug.log` when `DEBUG` env var is set)
- Use constants for "magic" values and enums
- Follow Go naming conventions:
  - Exported (public) names start with uppercase: `FetchSessions`
  - Unexported (private) names start with lowercase: `checkServer`
  - Acronyms should be all uppercase: `ID`, `URL`, `API`

### Bubble Tea framework guidelines

- This project uses **Bubble Tea** for the TUI framework
- The Model-View-Update (MVU) architecture is the core pattern:
  - **Model**: Holds application state (see `Model` struct in `tui.go`)
  - **View**: Renders the current state to a string (see `View()` method)
  - **Update**: Handles messages and updates state (see `Update()` method)

#### Model guidelines

- The `Model` struct contains all application state
- **Always** return both a `tea.Model` and `tea.Cmd` from `Update()`
- **Never** mutate state directly in goroutines - use messages
- Use view types (enums) to track which view should be rendered: `StartView`, `ListView`, `CreateSessionView`

#### Message handling

- Messages drive all state changes in Bubble Tea
- Messages can be any Go type (structs, slices, errors)
- Use type switches to handle different message types in `Update()`
- **Always** wrap errors in a custom error type that implements `error` interface (see `errMsg`)
- Commands (`tea.Cmd`) are functions that return messages asynchronously
- Use `tea.Batch()` to run multiple commands together
- Use `tea.Sequence()` to run commands in order

#### Command patterns

```go
// Simple command that returns a message
func myCommand() tea.Msg {
    result := doSomething()
    return result
}

// Command that needs access to config/state - use closure
func myCommandWithConfig(config *Config) tea.Cmd {
    return func() tea.Msg {
        result := doSomethingWithConfig(config)
        return result
    }
}

// Multiple commands in parallel
return m, tea.Batch(cmd1, cmd2, cmd3)

// Commands in sequence
return m, tea.Sequence(cmd1, cmd2)
```

#### View rendering guidelines

- The `View()` method must return a single string
- Use Lip Gloss for styling (see styling guidelines below)
- Switch on view type to render different screens
- **Always** compose views using `lipgloss.Join*()` functions
- Keep view logic separate from update logic

### Bubble Tea components (Bubbles)

This project uses official Bubble Tea components from the `bubbles` package:

#### List component

- Used for displaying the sessions list (see `m.list`)
- **Always** implement `list.Item` interface for items (see `Session` methods: `Title()`, `Description()`, `FilterValue()`)
- Set size with `SetSize()` in response to `tea.WindowSizeMsg`
- Update and render: `m.list.Update(msg)` and `m.list.View()`
- **Always** add custom key bindings with `AdditionalShortHelpKeys` and `AdditionalFullHelpKeys`

#### Spinner component

- Used for loading states (see `m.spinner`)
- **Always** call `m.spinner.Tick` in `Init()` to start animation
- Update spinner in `Update()` method when in loading state
- Choose spinner style from `spinner` package (e.g., `spinner.Hamburger`)

#### Form component (huh)

- Used for creating session forms (see `buildCreateSessionForm()`)
- Build forms with `huh.NewForm()` and groups with `huh.NewGroup()`
- Available input types:
  - `huh.NewInput()` - Text input
  - `huh.NewSelect()` - Dropdown selection
  - `huh.NewText()` - Multi-line text area
- **Always** use `.Value()` to bind form fields to struct fields
- Use `.Validate()` for custom validation logic
- Check form completion with `m.form.State == huh.StateCompleted`
- **Always** call `m.form.Init()` after building form
- Update form: `m.form.Update(msg)`

### Styling with Lip Gloss

- **Always** use Lip Gloss for terminal styling
- Define reusable styles at package level (see `listStyle`, `headerStyle`, `textStyle`, `formStyle`)
- Apply styles with `.Render(content)`
- Common style properties:
  - `.Margin(vertical, horizontal)` - Add spacing
  - `.Foreground(color)` - Text color
  - `.Border(borderType)` - Add borders (e.g., `lipgloss.RoundedBorder()`)
  - `.Padding()`, `.Width()`, `.Height()` - Sizing

#### Layout composition

```go
// Join views horizontally (side by side)
lipgloss.JoinHorizontal(lipgloss.Top, leftView, rightView)

// Join views vertically (stacked)
lipgloss.JoinVertical(lipgloss.Left, topView, bottomView)
```

### Markdown rendering with Glamour

- Use `glamour.Render()` to render Markdown to terminal
- **Always** handle rendering errors gracefully
- Use "dark" theme for terminal compatibility: `glamour.Render(markdown, "dark")`
- See `renderSessionNotes()` for example usage

### Code organization

- **main.go**: Application entry point and configuration
  - `Config` struct holds configuration (API URL)
  - `NewConfig()` creates config with defaults
  - `main()` initializes and runs Bubble Tea program
- **models.go**: Data models and types
  - `Kata` - Kata curriculum data
  - `Session` - Training session data
  - `SessionInput` - Input format for creating sessions
  - Interface implementations (`list.Item` methods)
- **katanaute_api.go**: API client functions
  - `FetchSessions()` - GET sessions from API
  - `FetchKatas()` - GET katas from API
  - `CreateSession()` - POST new session to API
  - `Data[T]` generic type for API responses
- **tui.go**: TUI implementation (Bubble Tea MVU)
  - `Model` struct and methods
  - View rendering functions
  - Message handlers
  - Form builders

### API integration guidelines

- All API calls go through `katanaute_api.go`
- The backend API is a Phoenix application running on `http://localhost:4000/api` by default
- API responses follow the format: `{ "data": [...] }`
- Use the generic `Data[T]` type for unmarshaling API responses
- **Always** defer `resp.Body.Close()` after HTTP requests
- **Always** check HTTP status codes and handle errors
- **Always** wrap API calls in commands that return messages
- Use `io.ReadAll()` to read response bodies
- Use `json.Marshal()` and `json.Unmarshal()` for JSON handling
- **Always** include the Bearer token in the `Authorization` header for authenticated endpoints

### Authentication guidelines

This application uses **device flow authentication** (OAuth2-style) for secure headless authentication:

**Device Flow Process:**
1. Request device code: `POST /api/auth/device/code`
   - Receives `device_code` (secret) and `user_code` (human-readable)
   - Receives `verification_uri` for user to visit
2. Display user code and verification URL to user in TUI
3. Poll for authorization: `POST /api/auth/device/token` with `device_code`
   - Poll every 5 seconds (as indicated by API response `interval`)
   - Returns `authorization_pending` while waiting
   - Returns access token when user approves in browser
   - Returns `access_denied` if user denies
4. Store access token and use for all subsequent API requests
5. Add token to requests: `Authorization: Bearer <token>` header

**Token Management:**
- Tokens are stored persistently (implementation-specific)
- Tokens are included in all API requests to authenticated endpoints
- Sessions endpoint requires authentication: `GET/POST/PUT/DELETE /api/sessions`
- Katas endpoint is public: `GET /api/katas`
- Handle `401 Unauthorized` responses by re-authenticating

**Authentication State:**
- Track authentication state in Model struct
- Show authentication flow in UI before main content
- Handle authentication errors gracefully
- Allow re-authentication if token expires or is invalid

#### API client pattern

```go
func FetchSomething(config *Config) ([]*Thing, error) {
    resp, err := http.Get(config.katanauteBaseURL + "/things")
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()

    data, err := io.ReadAll(resp.Body)
    if err != nil {
        return nil, err
    }

    var result Data[Thing]
    err = json.Unmarshal(data, &result)
    if err != nil {
        return nil, err
    }

    return result.Data, nil
}
```

### Configuration guidelines

- Configuration is in the `Config` struct
- Default API URL: `http://localhost:4000/api`
- Override with environment variable: `KATANAUTE_API_URL`
- Use `os.LookupEnv()` to check for environment variables
- Pass config to functions that need it (don't use global config)

### Debugging guidelines

- Set `DEBUG` environment variable to enable logging: `DEBUG=1 ./katago`
- Logs are written to `debug.log` file
- Use `log.Println()` for debug output (never `fmt.Println()` - it corrupts the TUI)
- Check `debug.log` file for error messages and debug output
- Use `tea.LogToFile()` to set up logging

### Error handling guidelines

- **Always** return errors from functions that can fail
- Wrap errors with context using `fmt.Errorf()` with `%w` verb: `fmt.Errorf("failed to fetch sessions: %w", err)`
- Create custom error message types for Bubble Tea (see `errMsg`)
- Handle errors in `Update()` by checking message type
- For fatal errors, return `tea.Quit` command
- For recoverable errors, show error state in UI

### Type definitions

- Use structs for data models
- Add JSON tags to struct fields for API marshaling: `json:"field_name"`
- Use `omitempty` for optional fields: `json:"id,omitempty"`
- Implement `String()` method for types that need text representation
- Use constants with `iota` for enums (see `viewType`)
- Use type aliases for clarity: `type viewType uint`

### Data handling guidelines

- Sessions are sorted by `practiced_at` date in descending order (newest first)
- Use Go's `time.Time` for datetime fields
- Parse datetime with `time.Parse()` - layout: `"2006-01-02 15:04"`
- Format datetime with `time.Format()` - use same layout pattern
- Sort slices with `sort.Slice()` and custom comparison function
- Check time ordering with `.After()` and `.Before()` methods

### Keyboard navigation

- Arrow keys and `j`/`k` - Navigate list (handled by bubbles/list)
- `Ctrl+C` - Quit application
- `a` - Add new session (custom key binding)
- **Always** check for `"ctrl+c"` in `Update()` to allow quitting
- Add custom key bindings with `key.NewBinding()`

### Testing guidelines (TODO)

This project currently has no tests. When adding tests:

- Use Go's built-in `testing` package
- Name test files with `_test.go` suffix
- Name test functions `TestFunctionName(t *testing.T)`
- Use table-driven tests for multiple test cases
- Mock HTTP calls for API client tests
- Test Bubble Tea Update logic by sending messages
- Run tests with `go test ./...`
- Use `go test -v` for verbose output

### Dependencies

- **bubbletea** - Terminal UI framework (MVU architecture)
- **bubbles** - Official Bubble Tea components (list, spinner, etc.)
- **lipgloss** - Terminal styling and layout
- **huh** - Form components for Bubble Tea
- **glamour** - Markdown rendering for terminals

### Common patterns

#### Switching views

```go
case []*Session:
    m.viewType = ListView
    m.sessions = msg
    // ... initialize view-specific state
```

#### Building and using forms

```go
// Build form
m.form = huh.NewForm(
    huh.NewGroup(
        huh.NewInput().Title("Field").Value(&m.data.Field),
    ),
)

// Check completion
if m.form.State == huh.StateCompleted {
    // Process form data
}
```

#### Fetching data

```go
// In Update()
case tea.KeyMsg:
    if msg.String() == "a" {
        return m, func() tea.Msg {
            return fetchDataCmd(m.config)
        }
    }

// Command function
func fetchDataCmd(config *Config) tea.Msg {
    data, err := FetchData(config)
    if err != nil {
        return errMsg{err}
    }
    return data
}
```

### Best practices

- Keep `Update()` method organized with clear type switches
- Separate view rendering into focused functions
- Use descriptive names for message types
- Document exported functions and types
- Avoid deeply nested if/else - use early returns
- Use `tea.Sequence()` for operations that must happen in order
- Use `tea.Batch()` for independent operations
- Handle window resize messages to adapt layout
- Initialize Bubble Tea program with `tea.WithAltScreen()` for full-screen mode

### Current features

- ✅ Device flow authentication with OAuth2-style flow
- ✅ Bearer token API authentication
- ✅ View list of training sessions
- ✅ Display session details with rendered Markdown notes
- ✅ Session creation with form UI
- ✅ Color-coded kata level display
- ✅ Loading states with spinner
- ✅ Split-view layout (list + details/form)

### Known limitations

- No session editing (only create and view)
- No session deletion
- No error recovery UI (errors cause quit)
- No offline mode or caching
- No configuration file (only environment variables)
- Token persistence implementation varies (check current implementation)

### TODO

- [ ] Add unit tests for API client functions
- [ ] Add unit tests for Bubble Tea Update logic
- [ ] Implement session editing
- [ ] Implement session deletion
- [ ] Add error recovery and display in UI
- [ ] Add loading state between form submission and refresh
- [ ] Add keyboard shortcuts help screen
- [ ] Add search/filter functionality
- [ ] Improve error messages for network failures
- [ ] Add configuration file support
