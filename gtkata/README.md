# GTKata - GNOME Kata Training Tracker

GTKata is a native GNOME application for tracking kata training sessions. It is built with GTK4 and libadwaita in Rust and uses the shared `katarustcore` crate for all API, authentication, configuration, and data models. GTKata talks to the Katanaute Phoenix backend and shows the same sessions you see in the web UI and other clients.

## What You Can Do

- Secure login using the device-code flow implemented in `katarustcore`
- Browse your training sessions in a GNOME-style list view
- Inspect a session detail view (kata, belt level, date/time, in-course flag)
- Read session notes rendered as Markdown with syntax highlighting
- Create new sessions with calendar/date and time selection
- Store and reuse your API token via an XDG-compliant config file

## Prerequisites

- Linux desktop (GNOME is the primary target)
- GTK4 and libadwaita development packages
- Rust toolchain (via `rustup`, Rust 2024 edition)
- A running Katanaute backend (see `katanaute/README.md`)

### Installing system dependencies

The exact package names vary slightly by distro; these examples are known-good starting points:

**Arch Linux:**
```bash
sudo pacman -S gtk4 libadwaita rust
```

**Ubuntu/Debian:**
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Fedora:**
```bash
sudo dnf install gtk4-devel libadwaita-devel rust cargo
```

## Quick Start

From the repository root:

```bash
# 1. Start the backend (from katanaute/)
cd katanaute
mix setup   # only needed the first time
mix phx.server

# 2. In another terminal, run GTKata
cd ../gtkata
cargo run
```

On first launch you will see an authentication screen. Press "Login" to start the device flow.

## Usage

### First Run & Login

1. Ensure the Katanaute backend is running (default: `http://localhost:4000`).
2. Start GTKata with `cargo run` (or use the release binary from `target/release/gtkata`).
3. On the "Authentication" screen, click **Login**.
4. The app requests a device code via `katarustcore::initiate_device_flow` and shows:
   - A **user code** you need to type in the browser
   - A **verification URL** you can click or copy
5. Open the URL in your browser, enter the code, and approve the request.
6. GTKata polls the backend via `katarustcore::poll_for_authorization` until approval and then loads your sessions.

### Browsing Sessions

- The main screen shows your sessions, newest first.
- Each entry displays:
  - Kata name
  - Practice date
  - Belt level badge (color-coded)
  - An indicator if the session is part of a course
- Use the **Refresh** button in the header bar to reload from the backend.

### Session Details & Notes

- Click a session row to open the detail view.
- You will see:
  - Kata name and belt badge
  - Practice date and time (UTC)
  - Whether it was part of a course
  - Notes rendered as Markdown
- Markdown rendering is handled by `src/markdown.rs` using `markdown`, `syntect`, and `html2pango` to build GTK widgets.

### Creating a Session

1. From the sessions list, click the **"+"** button.
2. GTKata fetches the kata list via `ApiClient::fetch_katas` from `katarustcore`.
3. Pick a kata (radio-style check buttons).
4. Choose a date with the calendar, and a time in `HH:MM` format.
5. Optionally add Markdown notes.
6. Toggle **"Part of Course"** if this is structured training.
7. Click **Create Session** to send a `SessionInput` to the backend.

### Logout

- Use the menu in the header bar (three-dot menu) and select **Logout**.
- This clears the stored token via `Config::clear_token` in `katarustcore` and returns you to the login screen.

## Configuration

Configuration is shared between GTKata and other Rust clients via `katarustcore`:

- Config directory: `~/.config/katanaute/`
- Config file: `config.json`

Example file:

```json
{
  "api_token": "your_token_here",
  "base_url": "http://localhost:4000/api"
}
```

You normally do not need to edit this file by hand. GTKata calls `Config::load`, `Config::save_token`, and `Config::clear_token` from `katarustcore` to manage it.

### Environment variables

- `KATANAUTE_API_URL` – override the default API URL for all `katarustcore` consumers.

Example:

```bash
KATANAUTE_API_URL=https://example.com/api cargo run
```

This affects both GTKata and any other binaries using `katarustcore::Config::load`.

## Architecture

GTKata is intentionally thin:

- **Backend contract** (endpoints, device flow, JSON shapes) lives in `katarustcore` and the Phoenix backend.
- **Shared logic** (API client, authentication helpers, configuration, models) is implemented in `katarustcore`.
- **GTKata** focuses on GNOME UX: windows, navigation, forms, lists, and Markdown rendering.

### Code layout

```text
gtkata/
├── Cargo.toml        # GTK4/libadwaita app depending on katarustcore
└── src/
    ├── main.rs       # libadwaita UI, AppState, wiring to katarustcore
    └── markdown.rs   # Markdown -> GTK widgets renderer for session notes
```

There is no local `api.rs`, `auth.rs`, `config.rs`, or `models.rs` anymore; if you need to change API behavior, update `katarustcore/` instead.

### GNOME patterns

GTKata follows standard libadwaita patterns:

- `adw::ApplicationWindow` as the main window
- `adw::NavigationView` with `adw::NavigationPage` for screen transitions
- `adw::ToolbarView` + `adw::HeaderBar` for top-level chrome
- `adw::PreferencesGroup` and `adw::ActionRow` for forms and details
- CSS classes such as `boxed-list`, `pill`, and custom belt classes for styling

## Development

From `gtkata/`:

```bash
# Build in debug mode
cargo build

# Run the application
cargo run

# Optimized release build
cargo build --release
./target/release/gtkata

# Format code
cargo fmt

# Lint
cargo clippy

# Tests (none yet, but this will run them once they exist)
cargo test
```

## Key Dependencies

GTKata itself depends on:

- `gtk4`, `libadwaita`, `glib` – Rust bindings from the gtk-rs project
- `katarustcore` – shared API/auth/config/models crate (`../katarustcore`)
- `tokio` – async runtime used by `katarustcore` and Markdown rendering helpers
- `chrono` – date/time handling for session timestamps
- `markdown`, `syntect`, `html2pango` – Markdown parsing and syntax highlighting
- `anyhow`, `log` – error reporting and logging

For the full list and versions, see `gtkata/Cargo.toml` and `katarustcore/Cargo.toml`.

## Roadmap

Current state (based on the code in this repository):

- [x] Device flow authentication and token persistence
- [x] Session list with belt badges and course indicator
- [x] Session detail view
- [x] Markdown rendering for session notes
- [ ] Session editing
- [ ] Session deletion with confirmation
- [ ] Search and filter
- [ ] Keyboard shortcuts
- [ ] Statistics dashboard
- [ ] Preferences dialog
- [ ] Session export
- [ ] Toast notifications

## Troubleshooting

### Application will not start

- Ensure GTK4 and libadwaita are installed (dev packages, not only runtime).
- Check that a display server (X11 or Wayland) is running.
- Try inspecting with:
  ```bash
  GTK_DEBUG=interactive cargo run
  ```

### Cannot connect to backend

- Make sure the Phoenix app in `katanaute/` is running.
- Confirm the API is reachable at the configured URL (default `http://localhost:4000/api`).
- Check `KATANAUTE_API_URL` and `~/.config/katanaute/config.json`.

### Authentication problems

- Ensure `/admin/device` on the backend is reachable in the browser.
- If you see repeated failures, remove the config file to reset authentication:
  ```bash
  rm ~/.config/katanaute/config.json
  ```
- Then restart GTKata and go through the login flow again.

### Build failures

- Update Rust:
  ```bash
  rustup update
  ```
- Clean and rebuild:
  ```bash
  cargo clean && cargo build
  ```
- Make sure all GTK4/libadwaita dev packages are installed.

## Resources

- GTK4 Rust bindings: https://gtk-rs.org/gtk4-rs/
- libadwaita Rust bindings: https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/
- GNOME Human Interface Guidelines: https://developer.gnome.org/hig
- Shared Rust core (`katarustcore`): ../katarustcore/README.md
- Katanaute backend: ../katanaute/README.md
