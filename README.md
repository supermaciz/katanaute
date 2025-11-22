# Katanaute

[![CI](https://github.com/supermaciz/katanaute/actions/workflows/ci.yml/badge.svg)](https://github.com/supermaciz/katanaute/actions/workflows/ci.yml)
[![Coverage](https://github.com/supermaciz/katanaute/actions/workflows/coverage.yml/badge.svg)](https://github.com/supermaciz/katanaute/actions/workflows/coverage.yml)
[![Security](https://github.com/supermaciz/katanaute/actions/workflows/security.yml/badge.svg)](https://github.com/supermaciz/katanaute/actions/workflows/security.yml)

[![Elixir](https://img.shields.io/badge/Elixir-1.19-4B275F?logo=elixir&logoColor=white)](https://elixir-lang.org/)
[![Phoenix](https://img.shields.io/badge/Phoenix-1.8-FD4F00?logo=phoenixframework&logoColor=white)](https://www.phoenixframework.org/)
[![React](https://img.shields.io/badge/React-18.3-61DAFB?logo=react&logoColor=black)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.9-3178C6?logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Rust](https://img.shields.io/badge/Rust-2024-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/Go-1.25-00ADD8?logo=go&logoColor=white)](https://go.dev/)

A multi-client Uechi-Ryu Karate kata training tracker with Phoenix backend serving a React web UI, plus native GUI clients (Rust, Go) and terminal interfaces (Go).

## Purpose

It's useless. I'm doing this for fun and to learn some stuff.

## Project Structure

This is a monorepo with seven integrated clients plus shared libraries:

### Core
- **katanaute/** - Phoenix 1.8 backend with REST API and LiveView admin UI (Elixir)
- **katareact/** - Modern React 18 web frontend served by Phoenix (TypeScript + Tailwind CSS)

### Native Clients
- **katarouille/** - Native GUI client with device flow auth (Rust + Iced)
- **gtkata/** - Native GUI client with device flow auth (Rust + GTK4 + libadwaita)
- **gtkata/** - Native GUI client with device flow auth (Rust + GTK4 + libadwaita)
- **katafyne/** - Native GUI client with device flow auth (Go + Fyne)
- **katago/** - Terminal UI client with device flow auth (Go + Bubble Tea)

### Shared Libraries
- **katagocore/** - Shared Go library for katafyne and katago (auth, config, API client)
- **katarustcore/** - Shared Rust library for katarouille and gtkata (auth, config, API client, models)

All clients connect to the same Phoenix backend and SQLite database.

## Quick Start

### All-in-One: Backend + React Web UI

```bash
cd katanaute
mix setup              # Install deps, create DB, run migrations, seed data
mix react.build        # Build React frontend and copy to Phoenix static dir
mix phx.server         # Start server on http://localhost:4000
```

Now visit:
- [http://localhost:4000](http://localhost:4000) - **React UI** (main web interface)
- [http://localhost:4000/admin](http://localhost:4000/admin) - **LiveView Admin** (Phoenix admin interface)

### React Development (with hot reload)

For React development with Vite hot reload:

```bash
cd katareact
npm install            # or: bun install
npm run dev           # Start dev server on http://localhost:3000
```

Visit [http://localhost:3000](http://localhost:3000) for the React UI with hot reload.

### Native GUI Clients

**Rust GUI (Katarouille)**
```bash
cd katarouille
cargo build
cargo run              # Requires backend running on localhost:4000
```

**Go GUI (Katafyne)**
```bash
cd katafyne
go build
./katafyne            # Requires backend running on localhost:4000
```

Both GUIs will walk you through device authentication on first run.

### Terminal Client (Go TUI)

```bash
cd katago
go build
./katago              # Requires backend running on localhost:4000
```

Use arrow keys or j/k to navigate, 'a' to add sessions, Ctrl+C to quit.

## Features

- **Session Tracking**: Record kata practice sessions with date/time, notes (Markdown), and course tracking
- **User Authentication**: Secure login with email/password, plus device flow for GUI/terminal clients
- **Multiple Interfaces**:
  - **React Web UI** (served by Phoenix at `/`) - main web interface
  - **LiveView Admin** (Phoenix admin at `/admin`) - admin interface
  - **Rust GUI** (Katarouille) - native cross-platform desktop app
  - **Go GUI** (Katafyne) - native cross-platform desktop app
  - **Go TUI** (Katago) - terminal interface
- **Color-Coded Levels**: Visual badges for kata progression (Yellow → Shodan)
- **RESTful API**: JSON API with Bearer token authentication
- **Developer-Friendly**: Comprehensive tests, hot reload, SQLite for easy setup

## Architecture

### Data Model

**Kata** (curriculum items)
- Name (e.g., "Sanchin", "Seisan", "Seichin")
- Level: yellow, orange, green, blue, brown, shodan

**Session** (training records)
- Practiced at (UTC datetime)
- In course (boolean - part of structured learning)
- Notes (Markdown text)
- Associated kata

### Routes

The application has three main route prefixes:

**`/` - React SPA (Main Web UI)**
- Served from Phoenix static directory
- Client-side routing with React Router
- Modern TypeScript interface with Tailwind CSS

**`/admin` - LiveView Admin Interface**
- `/admin/sessions` - Session management
- `/admin/users/register` - User registration
- `/admin/users/log_in` - Admin login
- `/admin/device` - Device authorization flow

**`/api` - REST API**
All API endpoints return JSON with `{ data: [...] }` format.

- **Authentication** (public):
  - `POST /api/auth/register` - Create account
  - `POST /api/auth/token` - Login (get Bearer token)
  - `POST /api/auth/device/code` - Start device flow (for GUI/TUI clients)
  - `POST /api/auth/device/token` - Poll device authorization
- **Sessions** (requires auth):
  - `GET /api/sessions` - List all sessions
  - `POST /api/sessions` - Create session
  - `GET/PUT/DELETE /api/sessions/:id` - Manage session
- **Katas** (public):
  - `GET /api/katas` - List all katas

## Configuration

### Backend
- **Port**: 4000 (default)
- **Database**: SQLite in `katanaute/dev.db`
- **Config**: `katanaute/config/{dev,test,prod}.exs`

### React Frontend

**Production (served by Phoenix)**:
- Built with `mix react.build` from `katanaute/` directory
- Served from `priv/static/react/` at root path `/`
- Assets reference `/react/assets/` (configured in `vite.config.ts`)

**Development (standalone Vite server)**:
- Runs on port 3000 with hot reload
- API proxy: `/api` → `http://localhost:4000`
- Start with `npm run dev` from `katareact/`
- Optional `.env` file:
  ```bash
  VITE_API_URL=http://localhost:4000/api
  ```

### Native Clients

All native clients (Katarouille, Katafyne, Katago) use the same configuration:

- **API URL**: `http://localhost:4000/api` (default)
- **Override**:
  ```bash
  export KATANAUTE_API_URL=http://your-server:port/api
  ```
- **Config Storage**: XDG-compliant directory (`~/.config/katanaute/`)

**Go TUI Debug Mode**:
```bash
DEBUG=1 ./katago  # Logs to debug.log
```

## Development

### Running Tests

**Backend (Phoenix)**
```bash
cd katanaute
mix test              # Run all tests
mix precommit         # Format, compile, test
```

**Frontend (React)**
```bash
cd katareact
npm test              # Run tests in watch mode
npm run build         # Production build (to dist/)

# For Phoenix deployment:
cd ../katanaute
mix react.build       # Build React and copy to priv/static/react/
```

### Database Management

```bash
cd katanaute
mix ecto.reset        # Drop, create, migrate, seed
mix ecto.migrate      # Run pending migrations
mix run priv/repo/seeds.exs  # Seed data only
```

### Adding Katas

Edit `katanaute/priv/repo/seeds.exs` and run:
```bash
cd katanaute
mix ecto.reset
```

Or use the API/LiveView to add them dynamically.

## Docker Deployment

### Building the Docker Image

The Dockerfile is configured to build both the Phoenix backend and React frontend. It must be built from the **monorepo root** (not from inside `katanaute/`) so the build context includes both directories:

```bash
# From the monorepo root directory (katanaute/)
docker build -f katanaute/Dockerfile -t katanaute .
```

The build process:
1. Installs Elixir and Node.js dependencies
2. Compiles Phoenix application
3. Runs `mix react.build` to build the React frontend
4. Compiles Phoenix assets
5. Creates a production release

### Running the Container

```bash
docker run -p 4000:4000 \
  -e SECRET_KEY_BASE="your-secret-key" \
  -e DATABASE_PATH="/app/data/katanaute.db" \
  -v katanaute-data:/app/data \
  katanaute
```

**Required Environment Variables:**
- `SECRET_KEY_BASE` - Generate with `mix phx.gen.secret`
- `DATABASE_PATH` - Path to SQLite database file

**Optional Environment Variables:**
- `PORT` - Server port (default: 4000)
- `PHX_HOST` - Hostname for URL generation

### Production Considerations

- The current Dockerfile uses SQLite, which requires persistent volume for the database
- For production scale, consider migrating to PostgreSQL
- Ensure `SECRET_KEY_BASE` is securely generated and stored
- Configure proper reverse proxy (nginx, Caddy) with SSL/TLS

## Troubleshooting

### Backend won't start
- Check if port 4000 is in use: `lsof -i :4000`
- Ensure dependencies: `cd katanaute && mix deps.get`
- Reset corrupted database: `mix ecto.reset`

### React can't connect to API
- Verify backend is running on port 4000
- Check Vite proxy config in `katareact/vite.config.js`
- Inspect browser Network tab for failed requests

### Native clients can't connect
- Ensure backend is running: `cd katanaute && mix phx.server`
- Verify database has data: `mix run priv/repo/seeds.exs`
- Check API URL: `export KATANAUTE_API_URL=http://localhost:4000/api`
- For Go TUI, enable debug: `DEBUG=1 ./katago` and check `debug.log`

## Project Status

**Current Features**
- ✅ User authentication (email/password + device flow)
- ✅ Phoenix backend with authenticated REST API
- ✅ React SPA served by Phoenix at `/` (main web UI)
- ✅ Phoenix LiveView admin UI at `/admin`
- ✅ Three native clients:
  - Rust GUI (Katarouille) with offline capability
  - Go GUI (Katafyne) with clean, modern interface
  - Go TUI (Katago) for terminal lovers
- ✅ Shared Go library (katagocore) for DRY code
- ✅ Comprehensive test coverage (Phoenix, React)
- ✅ Markdown notes support
- ✅ Color-coded kata level system
- ✅ Dual-UI architecture (React for users, LiveView for admin)

**Known Limitations**
- Session editing limited (available in LiveView only)
- Session deletion not available in native clients
- SQLite only (not production-ready for large scale)
- No tests for native clients (Katarouille, Katafyne, Katago)

**Future Enhancements**
- Session editing in React and native clients
- Session deletion in native clients
- Session filtering and search
- Statistics and progress tracking
- PostgreSQL support for production
- Multi-factor authentication
- Email confirmation flow
- Unit tests for native clients

## Documentation

For comprehensive development guidelines, see:
- **[CLAUDE.md](./CLAUDE.md)** - Overall project architecture and guidelines
- **[katanaute/CLAUDE.md](./katanaute/CLAUDE.md)** - Phoenix backend development
- **[katareact/CLAUDE.md](./katareact/CLAUDE.md)** - React frontend development
- **[katarouille/CLAUDE.md](./katarouille/CLAUDE.md)** - Rust GUI development
- **[katafyne/CLAUDE.md](./katafyne/CLAUDE.md)** - Go GUI development
- **[katago/CLAUDE.md](./katago/CLAUDE.md)** - Go TUI development
- **[katagocore/CLAUDE.md](./katagocore/CLAUDE.md)** - Go shared library development

## Resources

- [Phoenix Framework](https://hexdocs.pm/phoenix/overview.html)
- [Ecto](https://hexdocs.pm/ecto/Ecto.html)
- [Phoenix LiveView](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html)
- [React](https://react.dev/)
- [Vite](https://vitejs.dev/)
- [Bubble Tea](https://github.com/charmbracelet/bubbletea)
- [Tailwind CSS](https://tailwindcss.com/)

## License

This is a personal learning project. Use it however you want.
