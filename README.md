# Katanaute

A multi-client Uechi-Ryu Karate kata training tracker with Phoenix backend, React web UI, and Go terminal client.

## Purpose

It's useless. I'm doing this for fun and to learn some stuff.

But if you really want to track your kata practice sessions across multiple interfaces (web, terminal, and LiveView), this might be for you.

## Project Structure

This is a monorepo with three integrated components:

- **katanaute/** - Phoenix 1.8 backend with REST API and LiveView UI (Elixir)
- **katareact/** - Modern React 18 web frontend with TypeScript and Tailwind CSS
- **katago/** - Terminal UI client with Bubble Tea framework (Go)

All clients share the same Phoenix backend and SQLite database.

## Quick Start

### 1. Backend (Phoenix)

```bash
cd katanaute
mix setup              # Install deps, create DB, run migrations, seed data
mix phx.server         # Start server on http://localhost:4000
```

Visit [http://localhost:4000](http://localhost:4000) for the LiveView UI.

### 2. React Web Frontend

```bash
cd katareact
npm install            # or: bun install
npm run dev           # Start dev server on http://localhost:3000
```

Visit [http://localhost:3000](http://localhost:3000) for the React UI.

### 3. Terminal Client (Go)

```bash
cd katago
go build
./katago              # Requires backend running on localhost:4000
```

Use arrow keys or j/k to navigate, 'a' to add sessions, Ctrl+C to quit.

## Features

- **Session Tracking**: Record kata practice sessions with date/time, notes (Markdown), and course tracking
- **User Authentication**: Secure login with email/password, plus device flow for terminal clients
- **Multiple Interfaces**: Web UI (React + LiveView) and terminal client (Go)
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

### API Endpoints

All under `/api`, returning JSON with `{ data: [...] }` format.

**Authentication** (public):
- `POST /api/auth/register` - Create account
- `POST /api/auth/token` - Login (get Bearer token)
- `POST /api/auth/device/code` - Start device flow
- `POST /api/auth/device/token` - Poll device authorization

**Sessions** (requires auth):
- `GET /api/sessions` - List all sessions
- `POST /api/sessions` - Create session
- `GET/PUT/DELETE /api/sessions/:id` - Manage session

**Katas** (public):
- `GET /api/katas` - List all katas

## Configuration

### Backend
- **Port**: 4000 (default)
- **Database**: SQLite in `katanaute/dev.db`
- **Config**: `katanaute/config/{dev,test,prod}.exs`

### React Frontend
- **API Proxy**: Vite proxies `/api` to `http://localhost:4000`
- **Environment**: Optional `.env` file:
  ```bash
  VITE_API_URL=http://localhost:4000/api
  ```

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
npm run build         # Production build
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
- ✅ Phoenix LiveView web UI
- ✅ React SPA with TypeScript
- ✅ Go terminal UI with device flow auth
- ✅ Comprehensive test coverage (Phoenix, React)
- ✅ Markdown notes support
- ✅ Color-coded kata level system

**Known Limitations**
- Session editing limited (available in LiveView only)
- SQLite only (not production-ready for large scale)
- No tests for Go TUI

**Future Enhancements**
- Session editing in React and Go TUI
- Session filtering and search
- Statistics and progress tracking
- PostgreSQL support for production
- Multi-factor authentication

## Documentation

For comprehensive development guidelines, see:
- **[CLAUDE.md](./CLAUDE.md)** - Overall project architecture and guidelines
- **[katanaute/CLAUDE.md](./katanaute/CLAUDE.md)** - Phoenix backend development
- **[katareact/CLAUDE.md](./katareact/CLAUDE.md)** - React frontend development
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
