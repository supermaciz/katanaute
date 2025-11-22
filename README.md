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

A kata training tracker for the Uechi-Ryu curriculum. Phoenix powers the backend + LiveView admin, React delivers the main web UI, and a whole family of native clients (Rust + Go GUIs and a Go TUI) talk to the same API. I built it to learn, experiment, and have fun—it is not meant to be "production ready".

---

## Highlights
- **One backend, many faces** – Phoenix 1.8 serves the REST API, LiveView admin, and static React build. The same services fuel every native client.
- **Device flow authentication** – GUI/TUI clients use an OAuth2-style device code flow so you never type a password into a terminal window.
- **Markdown-friendly session notes** – every practice entry stores belt level, timestamp, structured flag, and free-form Markdown notes.
- **SQLite everywhere** – simple local dev storage that you can reset in a single command.
- **Learning playground** – expect duplicated features, rough edges, and some unfinished ideas. That's intentional.

---

## Repository Map

| Path | What lives there? | Language |
| --- | --- | --- |
| `katanaute/` | Phoenix backend, REST API, LiveView admin, React asset build pipeline | Elixir |
| `katareact/` | React 18 SPA served out of Phoenix' `/` route | TypeScript |
| `katarouille/` | Native GUI (Iced) with offline-friendly flow | Rust |
| `gtkata/` | GNOME/GTK4 GUI using libadwaita patterns | Rust |
| `katafyne/` | Cross-platform desktop GUI using Fyne | Go |
| `katago/` | Bubble Tea terminal client | Go |
| `katagocore/` | Shared Go auth/API helper lib | Go |
| `katarustcore/` | Shared Rust auth/API helper crate | Rust |

Each component ships with its own `CLAUDE.md` that explains conventions and TODOs.

---

## Quick Start

### Backend + React (served together)
```bash
cd katanaute
mix setup           # deps, DB, migrations, seeds
mix react.build     # build the React SPA into priv/static/react
mix phx.server      # http://localhost:4000 (React) + /admin (LiveView)
```

Visit:
- `http://localhost:4000` – React SPA (primary interface)
- `http://localhost:4000/admin` – LiveView admin UI with Phoenix LiveDashboard-style ergonomics

### React dev server (Vite)
```bash
cd katareact
npm install         # or bun install
npm run dev         # http://localhost:3000 with API proxy
```

### Native clients
All of them expect the backend on `http://localhost:4000` by default.

```bash
# Rust GUI (Iced)
cd katarouille && cargo run

# Rust GUI (GTK4)
cd gtkata && cargo run

# Go GUI (Fyne)
cd katafyne && go build && ./katafyne

# Go TUI (Bubble Tea)
cd katago && go build && ./katago   # press 'a' to add a session, j/k to navigate
```

Each client guides you through the device code auth flow the first time it launches.

---

## Development Workflow

### Backend (Phoenix)
- `mix phx.server` (or `iex -S mix phx.server`) to boot the API + LiveView.
- `mix react.build` bundles the SPA into `priv/static/react`.
- `mix test` / `mix precommit` keeps formatting + tests honest.
- `mix ecto.reset` drops/creates/migrates/seeds the SQLite DB.

Phoenix' LiveView + PubSub toolset (see the [Phoenix docs](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html)) keeps the admin UI reactive without piling on extra JS.

### React
- `npm run dev` for hot reload.
- `npm test` (Vitest + RTL) in watch mode.
- To create the Phoenix-served build, run `npm run build` followed by `cd ../katanaute && mix react.build`.

### Database + Seeds
- SQLite lives under `katanaute/dev.db` (git-ignored).
- Seeds: edit `katanaute/priv/repo/seeds.exs`, then rerun `mix ecto.reset` or `mix run priv/repo/seeds.exs`.

### Tests Summary
- Backend: ExUnit under `katanaute/test`.
- React: Vitest.
- Native clients: manual testing only (future TODOs live in each component doc).

---

## Architecture & Data Model

### Data Model
- **Kata** – name + belt level (yellow → shodan).
- **Session** – `practiced_at`, `in_course` flag, Markdown `notes`, belongs to a Kata.

### Routing
- `/` – React SPA served from Phoenix static assets.
- `/admin` – LiveView admin, session auth, device approvals.
- `/api` – JSON API returning `{ data: ... }` payloads.

Key endpoints:
- `POST /api/auth/register` / `POST /api/auth/token` – email/password auth.
- `POST /api/auth/device/code` & `POST /api/auth/device/token` – OAuth2-style device flow used by native clients.
- `GET/POST/PUT/DELETE /api/sessions` – authenticated CRUD with kata preloads.
- `GET /api/katas` – browseable curriculum without signing in.

Phoenix' Channel + PubSub infrastructure ([docs](https://hexdocs.pm/phoenix/channels.html)) keeps the admin UI and other connected clients in sync when sessions change.

---

## Configuration

### Backend
- `SECRET_KEY_BASE` – generate via `mix phx.gen.secret`.
- `DATABASE_PATH` – optional custom SQLite location (defaults inside repo).
- `PORT` – default 4000.

### React
Create `.env` in `katareact/` to override API location:
```bash
VITE_API_URL=http://localhost:4000/api
```

### Native clients
All support `KATANAUTE_API_URL`:
```bash
export KATANAUTE_API_URL=http://your-host:4000/api
```

`katago` also respects `DEBUG=1` to dump logs into `debug.log` so you can see Bubble Tea state transitions.

---

## Docker (optional)

The Dockerfile lives under `katanaute/`. Build from repo root so the Phoenix build step can copy the React app:
```bash
docker build -f katanaute/Dockerfile -t katanaute .
```

Run it with the expected secrets + mounted volume for SQLite persistence:
```bash
docker run -p 4000:4000 \
  -e SECRET_KEY_BASE="$(mix phx.gen.secret)" \
  -e DATABASE_PATH="/app/data/katanaute.db" \
  -v katanaute-data:/app/data \
  katanaute
```

For anything beyond tinkering consider Postgres + a proper reverse proxy.

---

## Troubleshooting
- **Backend refuses to start** – check port 4000 usage (`lsof -i :4000`), run `mix deps.get`, or `mix ecto.reset` the DB.
- **React can't reach the API** – ensure Phoenix is running, verify Vite proxy rules, inspect browser network tab.
- **Native client stuck waiting** – confirm device auth screen at `/admin/device`, set `KATANAUTE_API_URL`, and seed the DB so something exists to fetch.
- **TUI formatting weirdness** – never use `fmt.Println`; the Bubble Tea app logs via `log.Println` when `DEBUG=1`.

---

## Project Status

**What already works**
- ✅ Phoenix backend with REST API + LiveView admin
- ✅ React SPA served through Phoenix
- ✅ Device flow across all native clients
- ✅ Shared Go/Rust libraries for duplicated logic
- ✅ Markdown session notes + belt levels
- ✅ GitHub Actions CI + coverage + security workflows

**Known gaps / future experiments**
- Session editing/deleting missing in native clients
- SQLite-only (no Postgres story yet)
- No automated tests for GUIs/TUI
- Email confirmation + MFA still on the whiteboard
- Filtering/search/statistics dashboards still ideas

---

## Documentation & Resources
- **Repo-wide architecture notes** – [CLAUDE.md](./CLAUDE.md)
- **Component guides** – each directory contains a tailored `CLAUDE.md` (or README for the native apps) with TODOs and helper commands.
- **Phoenix framework references** – [LiveView docs](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html), [Channel/PubSub overview](https://hexdocs.pm/phoenix/channels.html).
- **React, Tailwind, Bubble Tea, Iced, GTK4, Fyne** – follow the official docs linked from each component guide.

Use this repo however you like; it exists so I can keep leveling up just like the kata log it tracks.
