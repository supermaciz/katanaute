# Katanaute - Kata Training Tracker

A multi-client (Uechi-Ryu) Karate kata training tracker application with a Phoenix backend, React web frontend, and multiple native clients (Rust GUI with Iced, Rust GUI with GTK4, Go GUI with Fyne, Go TUI with Bubble Tea).

> **Note**: This file contains comprehensive development guidelines for the entire monorepo. For quick-start instructions, see [README.md](./README.md). For component-specific guidelines, see the CLAUDE.md files in each subdirectory.

## Quick Reference

**Start Development**
```bash
# Backend + React (served from Phoenix)
cd katanaute && mix setup && mix react.build && mix phx.server
# React is now accessible at http://localhost:4000/
# LiveView admin UI at http://localhost:4000/admin

# React Development (separate dev server with hot reload)
cd katareact && npm install && npm run dev
# Dev server at http://localhost:3000 with API proxy

# Rust GUI (Iced)
cd katarouille && cargo build && cargo run

# Rust GUI (GTK4 + libadwaita)
cd gtkata && cargo build && cargo run

# Go GUI (Fyne)
cd katafyne && go build && ./katafyne

# Go TUI
cd katago && go build && ./katago
```

**Run Tests**
```bash
# Backend: mix test (in katanaute/)
# React: npm test (in katareact/)
# Rust GUI: Not yet implemented
# Go: Not yet implemented
```

**Component Documentation**
- Phoenix: [katanaute/CLAUDE.md](./katanaute/CLAUDE.md)
- React: [katareact/CLAUDE.md](./katareact/CLAUDE.md)
- Rust GUI (Iced): [katarouille/CLAUDE.md](./katarouille/CLAUDE.md)
- Rust GUI (GTK4): [gtkata/CLAUDE.md](./gtkata/CLAUDE.md)
- Go GUI: [katafyne/CLAUDE.md](./katafyne/CLAUDE.md)
- Go TUI: [katago/CLAUDE.md](./katago/CLAUDE.md)

## Table of Contents

- [Repository Structure](#repository-structure)
- [Architecture Overview](#architecture-overview)
- [Data Model](#data-model)
- [API Endpoints](#api-endpoints)
- [Authentication System](#authentication-system)
- [Development Workflows](#development-workflows)
- [Commit Conventions](#commit-conventions)
- [Configuration](#configuration)
- [Testing Strategy](#testing-strategy)
- [Common Development Tasks](#common-development-tasks)
- [Debugging](#debugging)
- [Key Technical Decisions](#key-technical-decisions)
- [Environment Variables](#environment-variables)
- [Security Considerations](#security-considerations)
- [Performance Considerations](#performance-considerations)
- [Deployment](#deployment-future)
- [Git Workflow](#git-workflow)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)
- [Resources](#resources)
- [Project Status](#project-status)

## Repository Structure

This is a monorepo containing seven main components and a shared Go library:

```
katanaute/
├── katanaute/          # Phoenix backend (Elixir/Phoenix 1.8)
│   ├── CLAUDE.md       # Phoenix-specific development guidelines
│   └── AGENTS.md       # Additional Phoenix guidelines
├── katareact/          # React frontend (React 18 + Vite)
│   └── CLAUDE.md       # React-specific development guidelines
├── katarouille/        # GUI client (Rust + Iced)
│   ├── CLAUDE.md       # Rust GUI development guidelines
│   └── README.md       # Rust GUI documentation
├── gtkata/             # GUI client (Rust + GTK4 + libadwaita)
│   ├── CLAUDE.md       # GTK4 GUI development guidelines
│   └── README.md       # GTK4 GUI documentation
├── katafyne/           # GUI client (Go + Fyne)
│   ├── CLAUDE.md       # Go GUI development guidelines
│   └── README.md       # Go GUI documentation
├── katago/             # Terminal UI client (Go + Bubble Tea)
│   ├── CLAUDE.md       # Go TUI development guidelines
│   └── README.md       # Go TUI documentation
├── katagocore/         # Shared Go library for katafyne and katago
│   ├── CLAUDE.md       # Library development guidelines
│   └── README.md       # Library documentation
└── CLAUDE.md           # This file - overall project guidelines
```

## Architecture Overview

### Backend: Phoenix Application (katanaute/)
- **Framework**: Phoenix 1.8 with LiveView
- **Database**: SQLite (via Ecto)
- **Purpose**: REST API server and optional LiveView web interface
- **Key Features**:
  - RESTful JSON API for sessions and katas
  - LiveView-based web UI
  - Database migrations and seeding
  - LiveDashboard for monitoring (dev only)

### Frontend: React SPA (katareact/)
- **Framework**: React 18 with Vite
- **Language**: TypeScript
- **Styling**: Tailwind CSS v3
- **Testing**: Vitest + React Testing Library
- **Purpose**: Modern web interface for managing training sessions
- **Deployment**: Served by Phoenix at `/` (root path)
- **Key Features**:
  - View and manage practice sessions
  - Create sessions with Markdown notes
  - Color-coded kata level badges
  - Responsive design
  - Full TypeScript type safety
  - Built and served from Phoenix static directory
  - LiveView admin UI accessible at `/admin`

### GUI Client: Rust with Iced (katarouille/)
- **Framework**: Iced (Elm Architecture GUI framework)
- **Language**: Rust 2024 edition
- **Purpose**: Native cross-platform GUI application for session management
- **Key Features**:
  - Device flow authentication with OAuth2-style flow
  - Native GUI with dark theme
  - View and create training sessions
  - Color-coded kata level badges
  - Token persistence in XDG-compliant config directory
  - Offline-capable (once authenticated)
  - Cross-platform (Linux, macOS, Windows)

### GUI Client: Rust with GTK4 (gtkata/)
- **Framework**: GTK4 + libadwaita (GNOME native widgets)
- **Language**: Rust 2024 edition
- **Purpose**: Native Linux GUI application for session management
- **Key Features**:
  - Device flow authentication with OAuth2-style flow
  - Modern GNOME design following Human Interface Guidelines
  - View and create training sessions
  - AdwNavigationView for screen transitions
  - Color-coded kata level badges
  - Token persistence in XDG-compliant config directory
  - Linux-native (GNOME/GTK ecosystem)
  - libadwaita adaptive layouts

### GUI Client: Go (katafyne/)
- **Framework**: Fyne (declarative GUI framework)
- **Language**: Go 1.18+
- **Purpose**: Native cross-platform GUI application for session management
- **Key Features**:
  - Device flow authentication with OAuth2-style flow
  - Native GUI with clean, modern interface
  - View and create training sessions
  - Split-pane layout for session list and details
  - Token persistence in XDG-compliant config directory
  - Cross-platform (Linux, macOS, Windows)
  - Simple MVC-like architecture

### CLI Client: Go TUI (katago/)
- **Framework**: Bubble Tea (terminal UI framework)
- **Language**: Go 1.25+
- **Purpose**: Terminal-based session viewer and creator
- **Key Features**:
  - Interactive list-based session browser
  - Create new training sessions with form UI
  - Keyboard navigation (arrow keys, j/k, 'a' to add, Ctrl+C to quit)
  - Markdown rendering for session notes
  - API integration with Phoenix backend

## Data Model

### Core Entities

#### Kata (Curriculum)
```elixir
schema "katas" do
  field :name, :string                    # e.g., "Sanchin", "Seisan"
  field :level, Ecto.Enum                 # yellow, orange, green, blue, brown, shodan
  has_many :sessions, Session
  timestamps()
end
```

Kata levels represent martial arts belt progression:
1. **Yellow** (Beginner)
2. **Orange**
3. **Green**
4. **Blue**
5. **Brown**
6. **Shodan** (Black belt)

#### Session (Training Record)
```elixir
schema "sessions" do
  field :practiced_at, :utc_datetime      # When the training occurred
  field :in_course, :boolean              # Part of structured learning path
  field :notes, :string                   # Markdown-formatted notes
  belongs_to :kata, Kata
  timestamps()
end
```

### Database
- **Type**: SQLite (ecto_sqlite3)
- **Migrations**: Located in `katanaute/priv/repo/migrations/`
- **Seeds**: Sample data in `katanaute/priv/repo/seeds.exs`

## API Endpoints

All API endpoints are under `/api` and return JSON with format: `{ data: [...] }`

### Authentication Endpoints (Public)

**User Registration and Login**
- `POST /api/auth/register` - Register a new user
  - Body: `{ email, password }`
  - Returns: `{ data: { access_token, token_type: "Bearer", user: { id, email } } }`
- `POST /api/auth/token` - Login with email/password
  - Body: `{ email, password }`
  - Returns: `{ data: { access_token, token_type: "Bearer", user: { id, email } } }`
- `DELETE /api/auth/token` - Logout (revoke current token)
  - Headers: `Authorization: Bearer <token>`
  - Returns: 204 No Content

**Device Flow (for headless/CLI clients)**
- `POST /api/auth/device/code` - Initiate device authorization flow
  - Returns: `{ device_code, user_code, verification_uri, verification_uri_complete, expires_in, interval }`
- `POST /api/auth/device/token` - Poll for authorization completion
  - Body: `{ device_code }`
  - Returns: Token when approved, or `authorization_pending`/`access_denied` errors

**Current User**
- `GET /api/auth/me` - Get current authenticated user info (requires auth)
  - Headers: `Authorization: Bearer <token>`
  - Returns: `{ data: { id, email, confirmed_at } }`

### Sessions (Requires Authentication)
- `GET /api/sessions` - List all sessions (includes preloaded kata data)
- `POST /api/sessions` - Create new session
- `GET /api/sessions/:id` - Get session details
- `PUT /api/sessions/:id` - Update session
- `DELETE /api/sessions/:id` - Delete session

**Authentication**: All session endpoints require a valid Bearer token in the `Authorization` header.

### Katas (Public Access)
- `GET /api/katas` - List all available katas
- `GET /api/katas/:id` - Get kata details

**Note**: Kata endpoints are publicly accessible to allow browsing the curriculum before registration.

## Authentication System

The application implements a comprehensive authentication system with support for both web and headless clients.

### Authentication Methods

**1. Web Authentication (Session-based)**
- Used by Phoenix LiveView UI
- Traditional email/password login
- Session cookies for persistence
- Routes: `/users/register`, `/users/log_in`, `/users/log_out`

**2. API Authentication (Token-based)**
- Used by React frontend and Go TUI
- Bearer tokens in `Authorization` header
- Tokens stored in `user_tokens` table with context "api"
- Register/Login returns access token

**3. Device Flow Authentication**
- OAuth2-style device authorization flow
- Designed for headless/CLI clients (Go TUI)
- User approves device via web interface
- No password exposure in terminal

### Device Flow Process

The device flow allows CLI applications to authenticate without exposing passwords in the terminal:

1. **Client initiates flow**: `POST /api/auth/device/code`
   - Server generates device_code (secret) and user_code (human-readable, e.g., "ABCD-EFGH")
   - Returns verification URI and codes
   - Device code expires in 15 minutes

2. **Client displays instructions**: Show user_code and verification_uri to user
   - User visits verification URI in browser
   - User logs in (if not already logged in)
   - User enters user_code or clicks pre-filled link

3. **User authorizes device**: Web interface at `/device/authorize`
   - Shows device code request details
   - User approves or denies the request
   - Server updates device_code status in database

4. **Client polls for completion**: `POST /api/auth/device/token` with device_code
   - Poll every 5 seconds (as indicated by `interval`)
   - Returns `authorization_pending` while waiting
   - Returns access token when approved
   - Returns `access_denied` if user denies

5. **Client uses token**: Store token and include in subsequent API requests
   - Header: `Authorization: Bearer <token>`
   - Token persists until revoked or deleted

### User Model

Users are managed by the `Katanaute.Accounts` context:

```elixir
schema "users" do
  field :email, :string
  field :password, :string, virtual: true
  field :hashed_password, :string
  field :confirmed_at, :naive_datetime
  has_many :sessions, Session
  timestamps()
end
```

### Token Management

Tokens are stored in the `user_tokens` table:
- **Context**: "api" for API tokens, "session" for web sessions
- **Hashed**: Token values are hashed before storage
- **Expiration**: Tokens can be explicitly revoked via `DELETE /api/auth/token`
- **Generation**: Uses Phoenix.Token with 32-byte random values

### Authentication Plugs

**`KatanauteWeb.Plugs.ApiAuth`**
- Extracts Bearer token from Authorization header
- Validates token and loads user into `conn.assigns.current_user`
- Options: `:require_authenticated_user` fails request if not authenticated
- Used in API pipeline

**`KatanauteWeb.Plugs.WebAuth`**
- Session-based authentication for browser requests
- Functions: `:fetch_current_user`, `:require_authenticated_user`, `:redirect_if_user_is_authenticated`
- Used in browser pipeline and LiveView

### Security Considerations

- **Password Hashing**: Uses Bcrypt via `Bcrypt.hash_pwd_salt/1`
- **Token Security**: Tokens are hashed before database storage
- **CSRF Protection**: Enabled for browser requests, disabled for API
- **Validation**: Email uniqueness enforced, password minimum length
- **SQL Injection**: Prevented via Ecto parameterized queries
- **XSS**: React and LiveView escape output by default

## Development Workflows

### Initial Setup

**1. Backend Setup (katanaute/)**
```bash
cd katanaute
mix setup                    # Install deps, create DB, run migrations, seed data
```

**2. Build and Serve React from Phoenix**
```bash
cd katanaute
mix react.build              # Build React and copy to priv/static/react/
mix phx.server              # Start server on http://localhost:4000
# React accessible at http://localhost:4000/
# LiveView admin at http://localhost:4000/admin
```

**3. React Development (with hot reload)**
```bash
cd katareact
npm install                  # or: bun install
npm run dev                 # Start dev server on http://localhost:3000
# Independent dev server with Vite hot reload and API proxy
```

**4. Rust GUI (GTK4) Setup (gtkata/)**
```bash
cd gtkata
cargo build
./target/debug/gtkata      # Requires backend running on localhost:4000
```

**5. Go TUI Setup (katago/)**
```bash
cd katago
go build
./katago                    # Requires backend running on localhost:4000
```

### Development Commands

#### Backend (Phoenix)
```bash
mix precommit               # Run before committing: compile, format, test
mix test                    # Run test suite
mix ecto.reset              # Drop DB, recreate, migrate, seed
mix react.build            # Build React frontend and copy to static dir
iex -S mix phx.server      # Start with interactive shell
```

#### Frontend (React)
```bash
npm test                    # Run tests in watch mode
npm run build              # Production build (outputs to dist/)
npm run preview            # Preview production build
# Note: For Phoenix deployment, use `mix react.build` from katanaute/
```

#### Rust GUI (Iced - Katarouille)
```bash
cargo build                 # Compile binary
cargo fmt                   # Format code
cargo clippy                # Lint code
cargo run                   # Run GUI client
```

#### Rust GUI (GTK4 - GTKata)
```bash
cargo build                 # Compile binary
cargo fmt                   # Format code
cargo clippy                # Lint code
cargo run                   # Run GUI client
```

#### Go GUI (Katafyne)
```bash
go build                    # Compile binary
go fmt ./...               # Format code
./katafyne                 # Run GUI client
go run .                   # Run without building
```

#### Go TUI (Katago)
```bash
go build                    # Compile binary
go fmt ./...               # Format code
./katago                   # Run TUI client
DEBUG=1 ./katago          # Run with debug logging to debug.log
```

### Component-Specific Guidelines

When working on a specific component, **ALWAYS** refer to its CLAUDE.md file:

- **Phoenix Backend**: See `katanaute/CLAUDE.md` for:
  - Phoenix 1.8 LiveView patterns
  - Ecto schema and changeset guidelines
  - Form handling and validation
  - Testing with Phoenix.LiveViewTest
  - Elixir-specific coding patterns

- **React Frontend**: See `katareact/CLAUDE.md` for:
  - React hooks and functional components
  - Tailwind CSS styling conventions
  - Vitest testing practices
  - API integration patterns
  - Form handling and validation

- **Rust GUI (Iced)**: See `katarouille/CLAUDE.md` for:
  - Iced Elm Architecture patterns
  - MVU (Model-View-Update) design
  - Device flow authentication
  - Token persistence
  - Rust-specific coding patterns

- **Rust GUI (GTK4)**: See `gtkata/CLAUDE.md` for:
  - GTK4 + libadwaita patterns
  - GNOME Human Interface Guidelines
  - AdwNavigationView for screen transitions
  - GLib async operations
  - Device flow authentication
  - Rust-specific coding patterns

- **Go GUI**: See `katafyne/CLAUDE.md` for:
  - Fyne GUI framework patterns
  - MVC-like architecture
  - Device flow authentication
  - Widget and container layout
  - Go-specific coding conventions

- **Go TUI**: See `katago/CLAUDE.md` for:
  - Bubble Tea MVU (Model-View-Update) architecture
  - Bubble Tea components (list, spinner, forms)
  - Lip Gloss styling and layout
  - API client patterns and error handling
  - Markdown rendering with Glamour
  - Go-specific coding conventions
  - Debugging with DEBUG mode

- **Go Shared Library**: See `katagocore/CLAUDE.md` for:
  - Shared library design principles
  - Device flow authentication implementation
  - Configuration management patterns
  - API client functions
  - Stateless function design
  - Client adaptation strategies

## Commit Conventions

Use **conventional commits** with appropriate scope:

### Phoenix Backend
```
feat(phoenix): add kata filtering endpoint
fix(phoenix): correct session validation
test(phoenix): add session controller tests
docs(phoenix): update API documentation
```

### React Frontend
```
feat(react): add session search functionality
fix(react): correct date formatting in session list
test(react): add comprehensive test suite
style(react): improve responsive layout
```

### Rust GUI (Iced)
```
feat(katarouille): implement session editing
fix(katarouille): handle authentication errors
test(katarouille): add API client tests
docs(katarouille): update installation guide
```

### Rust GUI (GTK4)
```
feat(gtkata): implement session editing
fix(gtkata): handle authentication errors
test(gtkata): add API client tests
docs(gtkata): update installation guide
```

### Go GUI
```
feat(katafyne): add session deletion feature
fix(katafyne): handle network errors gracefully
docs(katafyne): update installation guide
```

### Go TUI
```
feat(katago): implement session detail view
fix(katago): handle API connection errors
docs(katago): update usage instructions
```

### Repository-wide
```
chore: update dependencies across all projects
docs: add comprehensive CLAUDE.md
ci: add GitHub Actions workflow
```

## Configuration

### Backend Configuration
- **Database**: SQLite database in `katanaute/dev.db` (gitignored)
- **Port**: 4000 (default)
- **Config files**: `katanaute/config/{dev,test,prod}.exs`

### Routing Structure
The application has a dual-UI setup with clean route separation:

- **`/` (root)**: React SPA (primary user interface)
  - Served from `priv/static/react/index.html`
  - Client-side routing handled by React Router
  - Assets served from `/react/assets/`

- **`/admin`**: LiveView admin interface
  - `/admin/sessions` - Session management (LiveView)
  - `/admin/users/register` - User registration
  - `/admin/users/log_in` - User login
  - `/admin/device` - Device authorization flow

- **`/api`**: REST API endpoints
  - `/api/auth/*` - Authentication endpoints
  - `/api/katas` - Kata management
  - `/api/sessions` - Session management (requires auth)

### Frontend Configuration
- **Production**: Served by Phoenix from `priv/static/react/`
  - Build with `mix react.build` from `katanaute/` directory
  - Assets use `/react/` base path (configured in vite.config.ts)
- **Development**: Independent Vite dev server on port 3000
  - Hot reload enabled
  - API proxy: `/api` → `http://localhost:4000`
  - Run with `npm run dev` from `katareact/`
- **Environment**: `.env` file for custom API URL (optional)
  ```
  VITE_API_URL=http://localhost:4000/api
  ```

### Go TUI Configuration
- **API Base URL**: `http://localhost:4000/api` (default in Config struct)
- **Override**: Set `KATANAUTE_API_URL` environment variable
- **Debug Mode**: Set `DEBUG=1` to enable logging to `debug.log`

## Testing Strategy

### Backend Testing
- **Framework**: ExUnit
- **Coverage**: Controller tests, context tests, LiveView tests
- **Run**: `mix test` (auto-creates test DB)
- **Helpers**: `test/support/conn_case.ex`, `test/support/data_case.ex`

### Frontend Testing
- **Framework**: Vitest + React Testing Library
- **Coverage**: All pages and API client fully tested
- **Run**: `npm test` (watch mode)
- **Mocking**: Mock API responses in `src/test/utils.jsx`

### Go TUI Testing
- **Status**: Not yet implemented (see katago/CLAUDE.md TODO)
- **Planned Framework**: Go's built-in `testing` package
- **Coverage Goals**: API client tests, Bubble Tea Update logic tests

## Common Development Tasks

### Adding a New Kata
1. Add to seeds file: `katanaute/priv/repo/seeds.exs`
2. Run: `mix ecto.reset` (or insert via API/LiveView)

### Creating a Migration
```bash
cd katanaute
mix ecto.gen.migration add_new_field_to_sessions
# Edit the generated file in priv/repo/migrations/
mix ecto.migrate
```

### Adding a New API Endpoint
1. Update router: `katanaute/lib/katanaute_web/router.ex`
2. Create/update controller: `katanaute/lib/katanaute_web/controllers/`
3. Add JSON view: `katanaute/lib/katanaute_web/controllers/*_json.ex`
4. Update React API client: `katareact/src/services/api.js`
5. Update Go API client: `katago/katanaute_api.go`

### Debugging

**Backend**
- Use `IEx.pry` for breakpoints (requires `iex -S mix phx.server`)
- Check logs in terminal
- Visit `/dev/dashboard` for LiveDashboard

**Frontend**
- Browser DevTools
- React DevTools extension
- Vite dev server shows compilation errors

**Go TUI**
- Set `DEBUG=1` environment variable to enable logging to `debug.log`
- Use `log.Println()` for debug output (never `fmt.Println()` - it corrupts the TUI)
- Check `debug.log` file for error messages
- Verify backend API is running and accessible
- Check network requests with debug logs

## Key Technical Decisions

### Why SQLite?
- Simple setup for development
- Self-contained database file
- Sufficient for kata tracking use case
- Easy to reset and seed

### Why Multiple Clients?
- **React**: Modern web interface for full-featured management
- **Katarouille (Rust GUI)**: Native cross-platform desktop application with offline capability
- **GTKata (Rust GUI)**: Native Linux application with GNOME/GTK4 integration
- **Katafyne (Go GUI)**: Native cross-platform desktop application with simple, clean UI
- **Go TUI (Katago)**: Quick terminal access for developers
- **Phoenix LiveView**: Built-in real-time web option
- All share the same backend API

### Kata Level System
Based on martial arts belt progression, providing:
- Clear skill hierarchy
- Visual progression indicators (color-coded badges)
- Structured learning path (in_course flag)

## Environment Variables

### Backend (Phoenix)
```bash
SECRET_KEY_BASE=...         # Generated by Phoenix
DATABASE_PATH=...           # SQLite database path (optional)
PORT=4000                   # Server port
```

### Frontend (React)
```bash
VITE_API_URL=http://localhost:4000/api    # Backend API URL
```

### Rust GUI (Katarouille)
```bash
KATANAUTE_API_URL=http://localhost:4000/api    # Backend API URL
```

### Go GUI (Katafyne)
```bash
KATANAUTE_API_URL=http://localhost:4000/api    # Backend API URL
```

### Go TUI (Katago)
```bash
KATANAUTE_API_URL=http://localhost:4000/api    # Backend API URL
DEBUG=1                                         # Enable debug logging to debug.log
```

## Security Considerations

- **CSRF Protection**: Enabled for browser pipeline (Phoenix)
- **API Pipeline**: No CSRF (uses JSON, not sessions)
- **Input Validation**: All inputs validated via Ecto changesets
- **SQL Injection**: Prevented by Ecto's parameterized queries
- **XSS**: React escapes output by default; Markdown rendering in controlled contexts

## Performance Considerations

- **Sessions List**: React frontend sorts by date (newest first)
- **Database Queries**: Katas preloaded for sessions to avoid N+1 queries
- **LiveView**: Uses streams for efficient list rendering
- **React**: Minimal re-renders via proper state management

## Deployment (Future)

Currently development-focused. For production:

**Backend**
- Use PostgreSQL instead of SQLite
- Set `MIX_ENV=prod`
- Run `mix assets.deploy`
- Configure `SECRET_KEY_BASE`

**Frontend**
- Run `npm run build`
- Serve `dist/` directory via Nginx/CDN
- Update `VITE_API_URL` to production backend

**Rust GUI (Katarouille)**
- Build release binary: `cargo build --release`
- Binary located at `target/release/katarouille`
- Distribute to users for their platform
- Users configure `KATANAUTE_API_URL`

**Go TUI**
- Build binary: `go build`
- Distribute to users
- Users configure `KATANAUTE_API_URL`

## Git Workflow

1. **Branch Naming**: Use descriptive names (e.g., `feature/session-filtering`, `fix/kata-validation`)
2. **Commits**: Follow conventional commit format with appropriate scope
3. **Pre-commit**: Run component-specific precommit checks:
   - Phoenix: `mix precommit`
   - React: `npm test`
4. **Pull Requests**: Include tests for new features

## Troubleshooting

### Backend won't start
- Check if port 4000 is available: `lsof -i :4000`
- Ensure dependencies installed: `mix deps.get`
- Reset database if corrupted: `mix ecto.reset`

### React can't connect to API
- Verify backend is running on port 4000
- Check Vite proxy config in `vite.config.js`
- Inspect browser Network tab for failed requests

### Go TUI shows empty list or errors
- Verify backend is running on port 4000
- Check API URL configuration with `KATANAUTE_API_URL` env var
- Ensure database has seeded data: `cd katanaute && mix run priv/repo/seeds.exs`
- Enable debug mode (`DEBUG=1 ./katago`) and check `debug.log` for errors
- Verify network connectivity to backend

## Contributing

When making changes:

1. **Understand the component**: Read the component-specific CLAUDE.md
2. **Write tests**: All new features need tests
3. **Follow conventions**: Use existing patterns and naming
4. **Run precommit checks**: Ensure code quality before committing
5. **Update documentation**: Keep README and CLAUDE.md files current

## Resources

- **Phoenix**: https://hexdocs.pm/phoenix/overview.html
- **Ecto**: https://hexdocs.pm/ecto/Ecto.html
- **LiveView**: https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html
- **React**: https://react.dev/
- **Vite**: https://vitejs.dev/
- **Bubble Tea**: https://github.com/charmbracelet/bubbletea
- **Tailwind CSS**: https://tailwindcss.com/

## Project Status

**Current Features**
- ✅ User authentication with email/password
- ✅ Device flow authentication for CLI/GUI clients
- ✅ Bearer token API authentication
- ✅ Phoenix backend with authenticated REST API
- ✅ Phoenix LiveView web UI with session authentication
- ✅ React SPA with full session management and auth
- ✅ Rust GUI (Katarouille) with device flow authentication
- ✅ Rust GUI (GTKata) with GTK4/libadwaita and device flow authentication
- ✅ Go GUI (Katafyne) with device flow authentication
- ✅ Go TUI (Katago) with device flow authentication
- ✅ Comprehensive test coverage (Phoenix, React)
- ✅ Color-coded kata level system
- ✅ Markdown notes support
- ✅ Session sorting by date
- ✅ Split-view layout in Go clients
- ✅ Native cross-platform GUIs (Katarouille, Katafyne)
- ✅ Native GNOME GUI with AdwNavigationView (GTKata)

**Known Limitations**
- Session editing limited to LiveView (not in React or native clients)
- Session deletion available in LiveView and React only
- SQLite database (not suitable for production scale)
- No tests for native clients (Katarouille, GTKata, Katafyne, Katago)
- No email confirmation flow (confirmed_at field exists but not enforced)

**Future Enhancements** (See component TODOs)
- Session editing in React frontend and native clients
- Session deletion in native clients (Katarouille, GTKata, Katafyne, Katago)
- Unit tests for native clients
- Email confirmation and password reset flows
- Multi-factor authentication
- Session filtering and search
- Statistics and progress tracking
- PostgreSQL support for production
- Error recovery UI in native clients
- Markdown rendering in native GUI clients
- Session detail view in GTKata
