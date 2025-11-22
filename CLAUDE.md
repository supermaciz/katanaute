# Katanaute – AI Operator Briefing

> This file is for AI assistants. Human-friendly onboarding lives in `README.md`.

Katanaute is a Phoenix 1.8 backend that exposes a JSON API, a LiveView-powered admin UI, and static React assets. Several native clients (Rust Iced, Rust GTK4, Go Fyne, Go Bubble Tea) sit on top of the same surface. Treat the repo as a learning sandbox: polish varies, and experimentation is encouraged.

---

## Prime Directives
1. **Keep README human-first** – only mirror developer-facing context in this file when it improves automation.
2. **Use Context7 for framework specifics** – pull Phoenix/LiveView/Channel details from the official docs (e.g. [Phoenix LiveView](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html), [Channels + PubSub](https://hexdocs.pm/phoenix/channels.html)) before making claims about their behaviour.
3. **Respect component CLAUDE.md files** – each subproject carries additional rules (coding style, TODOs, testing requirements). Read them before editing that component.
4. **Do not broaden scope on your own** – modify only what the user asks for; report external unexpected changes immediately.
5. **Prefer targeted commands** – run `mix`, `npm`, `cargo`, or `go` from the component root; avoid repo-wide blanket operations unless specifically requested.
6. **No destructive git operations** – never `reset --hard` or drop user work.

---

## Repo Map (for automation)

| Path | Role | Notes |
| --- | --- | --- |
| `katanaute/` | Phoenix backend + LiveView admin + serves React build | mix tasks (`setup`, `phx.server`, `react.build`, `ecto.*`) |
| `katareact/` | React 18 + Vite SPA | npm scripts (`dev`, `build`, `test`) |
| `katarouille/` | Native GUI (Iced Elm-style architecture) | depends on `katarustcore/` |
| `gtkata/` | GNOME/libadwaita GUI | depends on `katarustcore/` |
| `katafyne/` | Go + Fyne GUI | depends on `katagocore/` |
| `katago/` | Go Bubble Tea TUI | depends on `katagocore/` |
| `katagocore/` | Shared Go library | consumed by Go clients |
| `katarustcore/` | Shared Rust crate | consumed by Rust clients |

Other directories (`gtkata`, `katafyne`, etc.) may include `CLAUDE.md` or `README.md` with client-specific rituals.

---

## Architecture Notes (AI Edition)
- **Backend** – Phoenix 1.8 + LiveView. Routes are split between `/` (React SPA served from `priv/static/react`), `/admin` (LiveView admin, device approval UI), and `/api` (JSON). LiveView relies on PubSub/Channels; consult [Phoenix channel docs](https://hexdocs.pm/phoenix/channels.html) whenever you document or change real-time behaviour.
- **Data model** – two primary schemas: `Kata` (name, `Ecto.Enum` level) and `Session` (`practiced_at`, `in_course`, Markdown `notes`, belongs_to Kata). SQLite stores everything during dev.
- **Auth** – Email/password token issuance for API clients plus an OAuth2-style device flow (code + polling) for native apps.
- **Shared libraries** – `katagocore` and `katarustcore` hold auth/config/API code so GUI/TUI projects stay thin. Update them in tandem when backend contracts move.

---

## Workflows & Commands

### Phoenix backend (`katanaute/`)
- `mix setup` – install deps, create/seed DB.
- `mix phx.server` – serve API + LiveView + built React UI.
- `mix react.build` – compile the React SPA into `priv/static/react` (run inside Phoenix app, not inside `katareact/`).
- `mix ecto.reset` / `mix ecto.migrate` – DB maintenance.
- `mix test` / `mix precommit` – ExUnit + formatter + compiler.

### React SPA (`katareact/`)
- `npm install` then `npm run dev` for hot reload (proxy to Phoenix).
- `npm test` (Vitest + Testing Library) for unit/UI coverage.
- `npm run build` → copies to `dist/`; Phoenix still needs `mix react.build` afterwards.

### Rust clients (`katarouille/`, `gtkata/`)
- `cargo build`, `cargo run`, `cargo fmt`, `cargo clippy`.
- Depend on backend being reachable at `http://localhost:4000/api` or `KATANAUTE_API_URL` environment variable.

### Go clients (`katafyne/`, `katago/`)
- `go build`, `go test` (if/when tests exist), `go fmt ./...`.
- Each binary expects the device code flow and API at `KATANAUTE_API_URL` (defaults to localhost).

---

## API & Authentication Cheatsheet
- JSON shape uses `{ data: ... }` everywhere.
- Public endpoints: `POST /api/auth/register`, `POST /api/auth/token`, `POST /api/auth/device/code`, `POST /api/auth/device/token`, `GET /api/katas`.
- Authenticated endpoints: `GET/POST/PUT/DELETE /api/sessions`, `GET /api/auth/me`, `DELETE /api/auth/token`.
- Device flow stages: `device/code` (issue codes) → user visits `/admin/device` → `device/token` polling until success/denial.

When updating docs or code, be explicit about polling interval, expiry, and where verification happens (LiveView on `/admin/device`).

---

## Testing Matrix
- **Phoenix** – ExUnit, `test/support` helpers available (`ConnCase`, `DataCase`).
- **React** – Vitest + React Testing Library; tests live under `src/**/*.test.tsx` plus `src/test` helpers.
- **Native clients** – currently manual; TODOs described inside component docs. If you add automated tests, update this matrix and mention exact commands.

Always run the narrowest necessary suite before reporting success.

---

## Configuration & Env Vars
- Phoenix expects `SECRET_KEY_BASE`, optionally `DATABASE_PATH`, `PORT` (default 4000).
- React dev server uses `.env` with `VITE_API_URL`.
- All native apps respect `KATANAUTE_API_URL`; `katago` also uses `DEBUG=1` for log dumps (`debug.log`).
- React assets ship under `/react/assets/*` once `mix react.build` is run.

Document new configuration flags in both `README.md` (human audience) and `AGENTS.md` (automation cues).

---

## Editing Guidance for AI
- **Documentation split** – README stays conversational; mirror detailed procedural or automation-specific text here.
- **Comments** – only introduce code comments when logic is non-obvious. Prefer succinct, high-signal remarks.
- **Formatting** – run formatter/linter commands already configured per component (e.g., `mix format`, `npm run lint` if added later, `cargo fmt`).
- **Database state** – SQLite files live under `katanaute/`; never commit them.
- **Secrets** – do not leak generated tokens or `.env` contents.

---

## Conventional Commits
- Follow [`type(optional-scope): imperative summary`] e.g., `feat(api): add device polling jitter`.
- Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`.
- Scope lives in parentheses right after the type (e.g., `feat(phoenix)`, `fix(katago)`, `style(katarouille)`); keep it small and lowercase, or omit entirely if it would be forced.
- Use bodies for rationale, linking issues with `Refs:` or `Closes:` when useful.
- Breaking changes must use either `type!` or a `BREAKING CHANGE:` trailer that states the migration step.

---

## Troubleshooting Reference
- Backend fails to boot → check `mix deps.get`, port collisions, or corrupt DB (`mix ecto.reset`).
- React cannot reach API → ensure Phoenix is running, confirm Vite proxy configuration, inspect browser dev tools.
- Native client stuck on auth → make sure `/admin/device` LiveView is reachable and `KATANAUTE_API_URL` matches the backend.
- Bubble Tea layout corruption → logging must use `log.Println`; `fmt.Println` breaks the TUI.

Record additional recurring issues here so future agents can react faster.

---

## Project Status Snapshot
- ✅ Phoenix API + LiveView admin, React SPA, Rust + Go clients, shared libraries, CI workflows.
- ⚠️ Missing items: session editing/deletion in non-web clients, automated native client tests, email confirmation/MFA, filtering/search dashboards, PostgreSQL prod story.

When implementing roadmap items, update this section plus the component-specific CLAUDE.md.

---

## External Resources
- Phoenix & LiveView references: [LiveView docs](https://hexdocs.pm/phoenix_live_view/Phoenix.LiveView.html), [Channels/PubSub](https://hexdocs.pm/phoenix/channels.html). Use Context7 to pull the relevant sections before citing behaviour.
- React, Tailwind, Bubble Tea, Iced, GTK4, Fyne: consult upstream docs and mirror only the parts necessary for automation decisions.

Read everything with an AI-operator mindset: this file is your playbook; keep it tight, actionable, and up to date.
