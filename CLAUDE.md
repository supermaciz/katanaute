# Katanaute - Kata Training Tracker

A multi-client kata training tracker application with a Phoenix backend, React web frontend, and Go terminal UI client.

## Repository Structure

This is a monorepo containing three main components:

```
katanaute/
├── katanaute/          # Phoenix backend (Elixir/Phoenix 1.8)
│   ├── CLAUDE.md       # Phoenix-specific development guidelines
│   └── AGENTS.md       # Additional Phoenix guidelines
├── katanaute-react/    # React frontend (React 18 + Vite)
│   └── CLAUDE.md       # React-specific development guidelines
├── katago/             # Terminal UI client (Go + Bubble Tea)
│   ├── CLAUDE.md       # Go TUI development guidelines
│   └── README.md       # Go TUI documentation
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

### Frontend: React SPA (katanaute-react/)
- **Framework**: React 18 with Vite
- **Styling**: Tailwind CSS v3
- **Testing**: Vitest + React Testing Library
- **Purpose**: Modern web interface for managing training sessions
- **Key Features**:
  - View and manage practice sessions
  - Create sessions with Markdown notes
  - Color-coded kata level badges
  - Responsive design

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

### Sessions
- `GET /api/sessions` - List all sessions (includes preloaded kata data)
- `POST /api/sessions` - Create new session
- `GET /api/sessions/:id` - Get session details
- `PUT /api/sessions/:id` - Update session
- `DELETE /api/sessions/:id` - Delete session

### Katas
- `GET /api/katas` - List all available katas
- `GET /api/katas/:id` - Get kata details

## Development Workflows

### Initial Setup

**1. Backend Setup (katanaute/)**
```bash
cd katanaute
mix setup                    # Install deps, create DB, run migrations, seed data
mix phx.server              # Start server on http://localhost:4000
```

**2. React Frontend Setup (katanaute-react/)**
```bash
cd katanaute-react
npm install                  # or: bun install
npm run dev                 # Start dev server on http://localhost:3000
```

**3. Go TUI Setup (katago/)**
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
iex -S mix phx.server      # Start with interactive shell
```

#### Frontend (React)
```bash
npm test                    # Run tests in watch mode
npm run build              # Production build
npm run preview            # Preview production build
```

#### Go TUI
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

- **React Frontend**: See `katanaute-react/CLAUDE.md` for:
  - React hooks and functional components
  - Tailwind CSS styling conventions
  - Vitest testing practices
  - API integration patterns
  - Form handling and validation

- **Go TUI**: See `katago/CLAUDE.md` for:
  - Bubble Tea MVU (Model-View-Update) architecture
  - Bubble Tea components (list, spinner, forms)
  - Lip Gloss styling and layout
  - API client patterns and error handling
  - Markdown rendering with Glamour
  - Go-specific coding conventions
  - Debugging with DEBUG mode

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

### Frontend Configuration
- **API Proxy**: Vite proxies `/api` to `http://localhost:4000`
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
4. Update React API client: `katanaute-react/src/services/api.js`
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
- **Go TUI**: Quick terminal access for developers
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

### Go TUI
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
- ✅ Phoenix backend with REST API
- ✅ Phoenix LiveView web UI
- ✅ React SPA with full session management
- ✅ Go TUI for session viewing and creation
- ✅ Comprehensive test coverage (Phoenix, React)
- ✅ Color-coded kata level system
- ✅ Markdown notes support
- ✅ Session sorting by date
- ✅ Split-view layout in Go TUI

**Known Limitations**
- No session editing in Go TUI (only create and view)
- No session deletion in Go TUI
- SQLite database (not suitable for production scale)
- No user authentication/authorization
- No session editing in React (only create/delete)
- No tests for Go TUI

**Future Enhancements** (See component TODOs)
- Session editing in React frontend
- Session editing and deletion in Go TUI
- Unit tests for Go TUI
- User authentication
- Session filtering and search
- Statistics and progress tracking
- PostgreSQL support for production
- Error recovery UI in Go TUI
