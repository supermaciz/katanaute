# Katanaute - Phoenix Backend

Backend server for the Katanaute kata training tracker. Serves the React frontend, provides REST API, and includes a LiveView admin interface.

## Quick Start

### Serve React + API + Admin

```bash
mix setup              # Install deps, create DB, run migrations, seed data
mix react.build        # Build React frontend and copy to static dir
mix phx.server         # Start server on http://localhost:4000
```

Now visit:
- `http://localhost:4000` - **React UI** (main web interface)
- `http://localhost:4000/admin` - **LiveView Admin** (Phoenix admin interface)
- `http://localhost:4000/api` - **REST API** endpoints

## Features

- **Dual-UI Architecture**:
  - React SPA at `/` (served from `priv/static/react/`)
  - LiveView admin at `/admin`
- **Authentication**: Email/password login, device flow for GUI/CLI clients
- **REST API**: JSON API with Bearer token authentication (`/api/*`)
- **Database**: SQLite with Ecto (migrations in `priv/repo/migrations/`)
- **Build Integration**: `mix react.build` task to build and deploy React frontend

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
mix react.build        # Build React frontend and copy to static dir
mix ecto.reset         # Reset database
iex -S mix phx.server  # Start with interactive shell
```

## Routes

The application has three main route prefixes:

- **`/` (root)**: React SPA (catch-all route serves `priv/static/react/index.html`)
- **`/admin`**: LiveView admin interface
  - `/admin/sessions` - Session management (LiveView)
  - `/admin/users/register` - User registration
  - `/admin/users/log_in` - Admin login
  - `/admin/device` - Device authorization flow
- **`/api`**: REST API endpoints (see API Endpoints section above)

## Docker Deployment

### Building the Image

The Dockerfile builds both Phoenix and React. **Must be run from the monorepo root** to access both `katanaute/` and `katareact/` directories:

```bash
# From the monorepo root (parent directory)
cd ..
docker build -f katanaute/Dockerfile -t katanaute .
```

The build process uses `mix react.build` internally, which:
1. Installs Node.js dependencies in `katareact/`
2. Builds React with Vite
3. Copies built assets to `priv/static/react/`

### Running

```bash
docker run -p 4000:4000 \
  -e SECRET_KEY_BASE="$(mix phx.gen.secret)" \
  -e DATABASE_PATH="/app/data/katanaute.db" \
  -v katanaute-data:/app/data \
  katanaute
```

**Environment Variables:**
- `SECRET_KEY_BASE` - Required for production (generate with `mix phx.gen.secret`)
- `DATABASE_PATH` - SQLite database path (default: in-memory)
- `PORT` - Server port (default: 4000)
- `PHX_HOST` - Host for URL generation

**Note:** For production, consider PostgreSQL instead of SQLite for better concurrency.

## Documentation

See [CLAUDE.md](./CLAUDE.md) for comprehensive Phoenix development guidelines.
