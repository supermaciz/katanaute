# Katanaute

A multi-client Uechi-Ryu Karate kata training tracker with Phoenix backend serving a React web UI, plus Rust GUI and Go terminal clients.

## Purpose

It's useless. I'm doing this for fun and to learn some stuff.

## Project Structure

This is a monorepo with four integrated components:

- **katanaute/** - Phoenix 1.8 backend with REST API and LiveView admin UI (Elixir)
- **katareact/** - Modern React 18 web frontend served by Phoenix (TypeScript + Tailwind CSS)
- **katarouille/** - Native GUI client with device flow auth (Rust + Iced)
- **katago/** - Terminal UI client with device flow auth (Go + Bubble Tea)

All clients share the same Phoenix backend and SQLite database.

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

### Rust GUI Client

```bash
cd katarouille
cargo build
cargo run              # Requires backend running on localhost:4000
```

The GUI will walk you through device authentication on first run.

### Go Terminal Client

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
  - **React Web UI** (served by Phoenix at `/`)
  - **LiveView Admin** (Phoenix admin at `/admin`)
  - **Rust GUI** (native cross-platform desktop app)
  - **Go TUI** (terminal interface)
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

### Rust GUI
- **API URL**: `http://localhost:4000/api` (default)
- **Override**:
  ```bash
  export KATANAUTE_API_URL=http://your-server:port/api
  ```
- **Config Storage**: XDG-compliant config directory (`~/.config/katarouille/`)

### Go TUI
- **API URL**: `http://localhost:4000/api` (default)
- **Override**:
  ```bash
  export KATANAUTE_API_URL=http://your-server:port/api
  ```
- **Debug Mode**:
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

### Go TUI shows empty list
- Ensure backend is running: `cd katanaute && mix phx.server`
- Verify database has data: `mix run priv/repo/seeds.exs`
- Check API URL: `export KATANAUTE_API_URL=http://localhost:4000/api`
- Enable debug mode: `DEBUG=1 ./katago` and check `debug.log`

## Project Status

**Current Features**
- ✅ User authentication (email/password + device flow)
- ✅ Phoenix backend with authenticated REST API
- ✅ React SPA served by Phoenix at `/` (main web UI)
- ✅ Phoenix LiveView admin UI at `/admin`
- ✅ Rust GUI client with device flow auth
- ✅ Go terminal UI with device flow auth
- ✅ Comprehensive test coverage (Phoenix, React)
- ✅ Markdown notes support
- ✅ Color-coded kata level system
- ✅ Dual-UI architecture (React for users, LiveView for admin)

**Known Limitations**
- Session editing limited (available in LiveView only)
- Session deletion not available in Rust GUI or Go TUI
- SQLite only (not production-ready for large scale)
- No tests for Rust GUI or Go TUI

**Future Enhancements**
- Session editing in React, Rust GUI, and Go TUI
- Session deletion in Rust GUI and Go TUI
- Session filtering and search
- Statistics and progress tracking
- PostgreSQL support for production
- Multi-factor authentication
- Email confirmation flow

## Documentation

For comprehensive development guidelines, see:
- **[CLAUDE.md](./CLAUDE.md)** - Overall project architecture and guidelines
- **[katanaute/CLAUDE.md](./katanaute/CLAUDE.md)** - Phoenix backend development
- **[katareact/CLAUDE.md](./katareact/CLAUDE.md)** - React frontend development
- **[katarouille/CLAUDE.md](./katarouille/CLAUDE.md)** - Rust GUI development
- **[katago/CLAUDE.md](./katago/CLAUDE.md)** - Go TUI development

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
