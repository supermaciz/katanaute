# Katanaute - Phoenix Backend

Backend server for the Katanaute kata training tracker. Provides REST API and LiveView UI with user authentication.

## Quick Start

```bash
mix setup              # Install deps, create DB, run migrations
mix phx.server         # Start server on http://localhost:4000
```

Visit `http://localhost:4000` to access the LiveView UI.

## Features

- **Authentication**: Email/password login, device flow for CLI clients
- **REST API**: JSON API with Bearer token authentication (`/api/*`)
- **LiveView UI**: Real-time web interface for session management
- **Database**: SQLite with Ecto (migrations in `priv/repo/migrations/`)

## API Endpoints

**Auth** (public):
- `POST /api/auth/register` - Create account
- `POST /api/auth/token` - Login
- `POST /api/auth/device/code` - Device flow start
- `POST /api/auth/device/token` - Device flow poll

**Sessions** (requires auth):
- `GET/POST /api/sessions` - List/create sessions
- `GET/PUT/DELETE /api/sessions/:id` - Manage session

**Katas** (public):
- `GET /api/katas` - List katas

## Development

```bash
mix test               # Run tests
mix precommit          # Format, compile, test (run before commit)
mix ecto.reset         # Reset database
iex -S mix phx.server  # Start with interactive shell
```

## Documentation

See [CLAUDE.md](./CLAUDE.md) for comprehensive Phoenix development guidelines.
