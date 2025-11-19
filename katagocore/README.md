# Katagocore - Shared Go Library

The shared code that powers both Katafyne (GUI) and Katago (terminal) clients.

## Why Does This Exist?

Before katagocore, Katafyne and Katago had almost identical code for authentication, configuration, and API calls - about 95% duplication! This library eliminates that duplication so:

- ✅ **One place to fix bugs** - Fix it once, both clients benefit
- ✅ **Easier to maintain** - Update auth logic in one spot
- ✅ **Consistent behavior** - Both clients work exactly the same way
- ✅ **Less code overall** - ~800 duplicated lines → ~350 shared lines

## What's Inside?

This library handles all the boring stuff so Katafyne and Katago can focus on being good UIs:

- 🔐 **Login/Authentication** - The whole device flow process
- 💾 **Saving Your Login** - Remembers you between sessions
- 🌐 **Talking to the Server** - Gets your sessions, creates new ones, fetches katas
- 📦 **Data Structures** - The Session, Kata, and User types both clients need
- ⚙️ **Configuration** - Handles settings and environment variables

## Package Structure

```
katagocore/
├── go.mod          # Module definition
├── auth.go         # Device flow authentication
├── config.go       # Configuration management
├── client.go       # API client functions
├── models.go       # Data structures
├── README.md       # This file
└── CLAUDE.md       # Development guidelines
```

## Using It in Your Code

If you're building another Go client for Katanaute, here's how to use this library:

**1. Add it to your `go.mod`:**

```go
require github.com/supermaciz/katanaute/katagocore v0.0.0

replace github.com/supermaciz/katanaute/katagocore => ../katagocore
```

**2. Import it:**

```go
import "github.com/supermaciz/katanaute/katagocore"
```

That's it! Now you have access to all the functions.

## Usage

### Configuration

```go
// Load configuration (from ~/.config/katanaute/config.json)
config, err := katagocore.NewConfig()
if err != nil {
    log.Fatal(err)
}

// Access values
fmt.Println(config.BaseURL)    // http://localhost:4000/api
fmt.Println(config.APIToken)   // the-bearer-token

// Override with environment variable
// KATANAUTE_API_URL=https://example.com/api
```

### Authentication

**Device Flow (synchronous)**

```go
token, err := katagocore.AuthenticateWithDeviceFlow(
    "http://localhost:4000/api",
    func(userCode, verificationURI string) {
        fmt.Printf("Visit: %s\n", verificationURI)
        fmt.Printf("Code: %s\n", userCode)
    },
)
if err != nil {
    log.Fatal(err)
}

// Save token
configFile := &katagocore.ConfigFile{
    APIToken: token,
    BaseURL:  "http://localhost:4000/api",
}
katagocore.SaveConfig(configFile)
```

**Device Flow (asynchronous with channels)**

```go
tokenChan, errChan := katagocore.AuthenticateWithDeviceFlowAsync(
    baseURL,
    func(message string) {
        fmt.Println(message)
    },
)

select {
case token := <-tokenChan:
    fmt.Println("Got token:", token)
case err := <-errChan:
    log.Fatal(err)
}
```

### API Client

**Fetch Sessions**

```go
sessions, err := katagocore.FetchSessions(config)
if err != nil {
    log.Fatal(err)
}

for _, session := range sessions {
    fmt.Printf("%s: %s\n", session.Kata.Name, session.PracticedAt)
}
```

**Create Session**

```go
session := &katagocore.SessionInput{
    Session: katagocore.Session{
        PracticedAt: time.Now(),
        InCourse:    true,
        Notes:       "Great training session!",
    },
    KataID: 1,
}

err := katagocore.CreateSession(config, session)
if err != nil {
    log.Fatal(err)
}
```

**Fetch Katas**

```go
katas, err := katagocore.FetchKatas(config)
if err != nil {
    log.Fatal(err)
}

for _, kata := range katas {
    fmt.Println(kata.String())  // "Sanchin (yellow)"
}
```

## Data Structures

### Config

```go
type Config struct {
    BaseURL  string  // API base URL
    APIToken string  // Bearer token
}
```

### ConfigFile

```go
type ConfigFile struct {
    APIToken string `json:"api_token"`
    BaseURL  string `json:"base_url"`
}
```

### Session

```go
type Session struct {
    ID          int       `json:"id,omitempty"`
    InCourse    bool      `json:"in_course"`
    Notes       string    `json:"notes,omitempty"`
    PracticedAt time.Time `json:"practiced_at"`
    Kata        *Kata     `json:"kata,omitempty"`
}
```

### Kata

```go
type Kata struct {
    ID    int    `json:"id"`
    Name  string `json:"name"`
    Level string `json:"level"`
}
```

### SessionInput

```go
type SessionInput struct {
    Session
    KataID int `json:"kata_id"`
}
```

### User

```go
type User struct {
    ID          int     `json:"id"`
    Email       string  `json:"email"`
    ConfirmedAt *string `json:"confirmed_at"`
}
```

## Client Adaptations

### Katago (Bubble Tea)

Katago needs to implement `list.Item` interface for sessions. Create an adapter:

```go
type SessionListItem struct {
    *katagocore.Session
}

func (s SessionListItem) FilterValue() string {
    return s.Kata.Name
}

func (s SessionListItem) Title() string {
    return s.Kata.Name
}

func (s SessionListItem) Description() string {
    return fmt.Sprintf("(%s)", s.PracticedAt.Format("2006-01-02"))
}
```

### Katafyne (Fyne)

Katafyne uses katagocore types directly without adaptation:

```go
sessions, err := katagocore.FetchSessions(config)
// Use sessions directly in Fyne widgets
```

## API Reference

### Configuration Functions

- `GetConfigDir() (string, error)` - Get XDG config directory
- `GetConfigPath() (string, error)` - Get config file path
- `LoadConfig() (*ConfigFile, error)` - Load config from disk
- `SaveConfig(*ConfigFile) error` - Save config to disk
- `NewConfig() (*Config, error)` - Create new runtime config
- `ClearToken() error` - Remove API token from config

### Authentication Functions

- `InitiateDeviceFlow(baseURL string) (*DeviceCodeResponse, error)`
- `PollForToken(baseURL, deviceCode string) (*DeviceTokenResponse, error)`
- `AuthenticateWithDeviceFlow(baseURL string, onCodeReceived func(string, string)) (string, error)`
- `AuthenticateWithDeviceFlowAsync(baseURL string, onProgress func(string)) (chan string, chan error)`

### API Client Functions

- `FetchSessions(config *Config) ([]*Session, error)`
- `CreateSession(config *Config, session *SessionInput) error`
- `FetchKatas(config *Config) ([]*Kata, error)`

## Testing

(TODO: Add unit tests)

```bash
go test ./...
```

## Development

See [CLAUDE.md](./CLAUDE.md) for detailed development guidelines.

## License

Part of the Katanaute monorepo. See main repository for license information.

## Related Projects

- **katafyne** - Go + Fyne GUI client (uses katagocore)
- **katago** - Go + Bubble Tea TUI client (uses katagocore)
- **katanaute** - Phoenix backend (Elixir)
- **katareact** - React web frontend (TypeScript)
- **katarouille** - Rust GUI client (doesn't use katagocore)
