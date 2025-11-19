# Katagocore Development Guidelines

Guidelines for developing and maintaining katagocore, the shared Go library for Katanaute clients.

## Purpose

Katagocore implements the DRY (Don't Repeat Yourself) principle by consolidating common code between:
- **Katafyne** (Go + Fyne GUI client)
- **Katago** (Go + Bubble Tea TUI client)

Before katagocore existed, these clients had ~95% code duplication. Now they share:
- Authentication logic
- Configuration management
- API client functions
- Data models

## Technology Stack

- **Language**: Go 1.18+
- **Dependencies**: None (stdlib only)
- **Architecture**: Stateless library functions

## Code Organization

```
katagocore/
├── go.mod          # Module definition (no external dependencies)
├── auth.go         # Device flow authentication (InitiateDeviceFlow, PollForToken, AuthenticateWithDeviceFlow)
├── config.go       # Configuration management (LoadConfig, SaveConfig, NewConfig)
├── client.go       # API client (FetchSessions, CreateSession, FetchKatas)
├── models.go       # Data structures (Session, Kata, User, SessionInput)
├── README.md       # User documentation
└── CLAUDE.md       # This file - development guidelines
```

## Design Principles

### 1. Stateless Functions

All functions are stateless and take `*Config` as a parameter:

```go
// Good
func FetchSessions(config *Config) ([]*Session, error) { ... }

// Bad (avoid package-level state)
var globalConfig *Config
func FetchSessions() ([]*Session, error) { ... }
```

### 2. No External Dependencies

Katagocore uses only Go standard library to minimize dependency bloat. Clients can add their own UI frameworks without conflicts.

### 3. Client Adaptability

Katagocore provides base types that clients can adapt:

**Katago (Bubble Tea)** wraps types to implement `list.Item`:
```go
type SessionListItem struct {
    *katagocore.Session
}

func (s SessionListItem) FilterValue() string {
    return s.Kata.Name
}
```

**Katafyne (Fyne)** uses types directly:
```go
sessions, err := katagocore.FetchSessions(config)
// Use in Fyne widgets directly
```

### 4. Sync and Async Variants

Provide both synchronous and asynchronous authentication methods:
- `AuthenticateWithDeviceFlow()` - blocks until complete (for TUI)
- `AuthenticateWithDeviceFlowAsync()` - returns channels (for GUI event loops)

## Module Structure

### auth.go

**Device Flow Authentication**

Implements OAuth2 device authorization flow:

1. `InitiateDeviceFlow()` - POST /auth/device/code
   - Returns device_code, user_code, verification_uri
2. `PollForToken()` - POST /auth/device/token
   - Poll with device_code until authorized
3. `AuthenticateWithDeviceFlow()` - Complete flow (sync)
4. `AuthenticateWithDeviceFlowAsync()` - Complete flow (async)

**Key Types**:
- `DeviceCodeResponse`
- `DeviceTokenResponse`

### config.go

**Configuration Management**

XDG-compliant configuration storage:

- `GetConfigDir()` - Returns `~/.config/katanaute/`
- `LoadConfig()` - Read config from disk
- `SaveConfig()` - Write config to disk
- `NewConfig()` - Create runtime config (env var override)
- `ClearToken()` - Remove API token

**Key Types**:
- `Config` - Runtime configuration
- `ConfigFile` - Persisted configuration (JSON)

**Config Priority**:
1. Environment variable `KATANAUTE_API_URL`
2. Saved config file
3. Default: `http://localhost:4000/api`

### client.go

**API Client Functions**

HTTP client for Katanaute REST API:

- `FetchSessions()` - GET /sessions (requires auth)
- `CreateSession()` - POST /sessions (requires auth)
- `FetchKatas()` - GET /katas (requires auth)

All endpoints:
- Use Bearer token authentication
- Return wrapped responses: `{ data: [...] }`
- Handle 401 Unauthorized errors

**Internal**:
- `createAuthRequest()` - Helper for authenticated requests
- `Data[T]` - Generic wrapper for API responses

### models.go

**Data Structures**

Shared types matching backend JSON:

- `Session` - Training session record
- `SessionInput` - Payload for creating sessions
- `Kata` - Kata definition
- `User` - User information

All types include JSON tags for serialization.

## Development Workflow

### Making Changes

1. **Update the library**
   ```bash
   cd katagocore
   # Edit files
   ```

2. **Test in clients**
   ```bash
   cd ../katafyne
   go mod tidy
   go build

   cd ../katago
   go mod tidy
   go build
   ```

3. **Format and vet**
   ```bash
   cd katagocore
   go fmt ./...
   go vet ./...
   ```

### Adding New Features

**When to add to katagocore**:
- ✅ Used by both Katafyne and Katago
- ✅ Doesn't depend on UI frameworks
- ✅ Pure logic or data structures

**When to keep in clients**:
- ❌ UI-specific code (Fyne widgets, Bubble Tea models)
- ❌ Client-specific adapters
- ❌ Application state management

### Breaking Changes

If you change katagocore APIs:

1. Update both katafyne and katago simultaneously
2. Document the breaking change in commit message
3. Test both clients thoroughly

## Common Patterns

### Error Handling

Return descriptive errors with context:

```go
if resp.StatusCode != http.StatusOK {
    body, _ := io.ReadAll(resp.Body)
    return nil, fmt.Errorf("unexpected status code: %d, body: %s", resp.StatusCode, string(body))
}
```

### Configuration Loading

Always use `NewConfig()` instead of `LoadConfig()` directly:

```go
// Good - handles env var override
config, err := katagocore.NewConfig()

// Bad - misses env var override
configFile, err := katagocore.LoadConfig()
```

### API Requests

Use `createAuthRequest()` helper:

```go
req, err := createAuthRequest("GET", config.BaseURL+"/endpoint", config.APIToken, nil)
client := &http.Client{}
resp, err := client.Do(req)
defer resp.Body.Close()
```

## Testing Strategy (TODO)

Currently no tests exist. Future testing should cover:

### Unit Tests

```go
// auth_test.go
func TestInitiateDeviceFlow(t *testing.T) {
    // Mock HTTP server
    // Test successful flow
    // Test error cases
}

// config_test.go
func TestNewConfig(t *testing.T) {
    // Test default values
    // Test env var override
    // Test file loading
}

// client_test.go
func TestFetchSessions(t *testing.T) {
    // Mock HTTP responses
    // Test authentication
    // Test error handling
}
```

### Integration Tests

Test against a real backend:

```bash
# Start backend
cd katanaute && mix phx.server

# Run integration tests
cd katagocore
go test -tags=integration ./...
```

## Troubleshooting

### Import Issues

If clients can't find katagocore:

```bash
# In client directory (katafyne or katago)
go mod tidy
go get github.com/supermaciz/katanaute/katagocore
```

### Replace Not Working

Ensure `replace` directive is in `go.mod`:

```go
replace github.com/supermaciz/katanaute/katagocore => ../katagocore
```

### Type Mismatches

If you get type errors after updating katagocore:

```bash
# Clean and rebuild
go clean
go mod tidy
go build
```

## Future Enhancements

### High Priority
- [ ] Unit tests for all functions
- [ ] Integration tests with test server
- [ ] Better error types (instead of strings)

### Medium Priority
- [ ] Session editing API
- [ ] Session deletion API
- [ ] Pagination support for large session lists
- [ ] Retry logic with exponential backoff

### Low Priority
- [ ] Offline caching (like Katarouille)
- [ ] GraphQL support
- [ ] Streaming API for real-time updates

## Commit Conventions

Use conventional commits with `katagocore` scope:

```
feat(katagocore): add session deletion API
fix(katagocore): handle network timeouts gracefully
test(katagocore): add unit tests for auth flow
docs(katagocore): update API documentation
refactor(katagocore): simplify error handling
```

## Contributing

When modifying katagocore:

1. **Keep it minimal** - Only shared code, no UI dependencies
2. **Test both clients** - Ensure katafyne and katago still work
3. **Document changes** - Update README.md and this file
4. **Format code** - Always run `go fmt`
5. **No external deps** - Stick to standard library

## Questions?

See main repository's `CLAUDE.md` for overall project guidelines.
