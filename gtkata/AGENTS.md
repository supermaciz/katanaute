# GTKata – Agent Playbook

You are working on **GTKata**, a GTK4 + libadwaita GUI client that sits on top of the Katanaute backend. This crate is a **thin GNOME UI layer**; all HTTP, auth, configuration, and data-model logic lives in `katarustcore`.

If you need to change how the device flow works, how tokens are stored, or how JSON is shaped, update `katarustcore/` instead and only adjust the way GTKata calls into it.

---

## Scope & Responsibilities

- GTKata owns:
  - GNOME-facing UX (windows, navigation, forms, lists, detail views).
  - Visual presentation of sessions, belts, notes, and errors.
  - Wiring user interactions to `katarustcore` APIs.
- `katarustcore` owns:
  - `ApiClient` (HTTP client) and all REST calls.
  - Device-code authentication helpers (`initiate_device_flow`, `poll_for_authorization`).
  - `Config` + file persistence + `KATANAUTE_API_URL` handling.
  - `Kata`, `Session`, `SessionInput`, and related models.

Do **not** duplicate HTTP clients, config parsing, or models inside GTKata. Always reuse the primitives from `katarustcore`.

Relevant files:

- GTKata: `gtkata/src/main.rs`, `gtkata/src/markdown.rs`, `gtkata/Cargo.toml`
- Shared core: `katarustcore/src/*.rs`, `katarustcore/README.md`, `katarustcore/AGENTS.md`

---

## Local Development Rules

- Run commands from `gtkata/` when you touch this crate.
- Conventional commits should use a `gtkata` scope when your work is GTK-specific:
  - `feat(gtkata): add keyboard shortcuts`
  - `fix(gtkata): handle empty notes gracefully`
- Before committing GTKata changes:
  - `cargo build`
  - `cargo fmt`
  - `cargo clippy`
- If you add tests here, run them with `cargo test` (katarustcore already has its own test guidance).

Rust guidelines (GTKata-specific):

- Edition: Rust 2024.
- Avoid `.unwrap()` and `panic!` in user-facing code; propagate errors or show them in the UI.
- Prefer `&str` parameters where possible; avoid needless `String` cloning.
- Use clear names; single-letter identifiers are only acceptable in tiny scopes (e.g., iterators).
- For IO or async operations, prefer functions returning `Result<T, Box<dyn std::error::Error>>` and rely on `?`.

---

## GTK4 + libadwaita Guidelines

GTKata is a **pure GTK4/libadwaita app** — do not introduce Relm4 or other GUI frameworks.

Use libadwaita-first patterns:

- `adw::ApplicationWindow` as the main window.
- `adw::NavigationView` + `adw::NavigationPage` for screen transitions.
- `adw::ToolbarView` with `adw::HeaderBar` for chrome.
- `adw::PreferencesGroup` + `adw::ActionRow` for forms and detail sections.
- CSS classes:
  - `title-1/2/3`, `caption` for typography.
  - `suggested-action`, `destructive-action`, `pill`, `boxed-list`, `card` for styling.

Follow the GNOME Human Interface Guidelines (HIG):

- Keep layouts simple and adaptive.
- Make primary actions obvious and secondary actions quiet.
- Require confirmation before destructive actions (once editing/deletion exist).

Reference docs (via Context7 / upstream):

- GTK4 Rust bindings: https://gtk-rs.org/gtk4-rs/
- libadwaita Rust bindings: https://world.pages.gitlab.gnome.org/Rust/libadwaita-rs/
- GNOME HIG: https://developer.gnome.org/hig

---

## State, Async, and API Usage

All application state currently lives in `AppState` inside `src/main.rs`:

```rust
struct AppState {
    api_client: ApiClient,
    config: Config,
    sessions: Vec<Session>,
    katas: Vec<Kata>,
}
```

Key rules:

1. **Do not create another HTTP client.** Always call methods on `ApiClient` from `katarustcore`.
2. **Respect the existing token flow.**
   - `Config::load()` decides the base URL and token using env + config file.
   - `Config::save_token` / `Config::clear_token` are the only places that should write to disk.
   - Use `AppState::save_token` / `AppState::clear_token` helpers rather than editing `Config` directly.
3. **Async in GTK:**
   - Use `glib::spawn_future_local` for all async work.
   - Clone the `ApiClient` and any shared `Rc<RefCell<AppState>>` into the closure.
   - Use `#[weak]` for widgets and `#[strong]` for owned state when using `glib::clone!`.
   - Never call `std::thread::sleep`; rely on the async helpers inside `katarustcore` (device polling already respects the server interval).
4. **Tokio runtime:**
   - A `tokio::runtime::Runtime` is created once in `main()` and entered before GTK starts.
   - Do not create additional runtimes.

When adding new screens or flows that talk to the backend:

- Start from the patterns in `load_sessions`, `show_session_create`, and the device flow in `show_authentication`.
- Push that logic down into `katarustcore` where it makes sense, then keep the GTK side focused on UI updates.

---

## Markdown Rendering

`src/markdown.rs` turns Markdown into GTK widgets using:

- `markdown` for parsing (GitHub-flavored Markdown via `ParseOptions::gfm()`).
- `syntect` + `html2pango` for syntax-highlighted code blocks.
- GTK `Label`, `Box`, `Grid`, etc. for layout.

Guidelines:

- Reuse `RenderConfig` and `render_input` rather than building new Markdown renderers.
- If you need different themes or image behavior, extend `RenderConfig` with additional options instead of forking the logic.
- Keep CSS for the Markdown renderer in `load_css()` (at the bottom of `markdown.rs`).

---

## Device Flow & Auth (UI Layer)

The device flow is implemented in `katarustcore::auth`; GTKata only orchestrates it:

- `show_authentication` calls `initiate_device_flow(&api_client)` and displays the returned `user_code` and `verification_uri`.
- It then calls `poll_for_authorization(&api_client, device_code, interval)` and updates the UI based on success or failure.
- On success, it saves the token via `AppState::save_token` and navigates to the session list.

As an agent:

- Never hard-code polling intervals or error strings that conflict with `katarustcore`.
- If the backend contract changes, update `katarustcore` first, then adapt the GTK messages.
- Keep auth-related UI logic in `show_authentication`; do not spread it across multiple screens.

---

## Configuration & Environment

Configuration is shared across all Rust clients via `katarustcore::Config`:

- Config directory: determined by `directories::ProjectDirs` for app name `katanaute`.
- File name: `config.json`.
- Fields (`ConfigFile`): `api_token: Option<String>`, `base_url: Option<String>`.
- Runtime config (`Config`):
  - `base_url: String` (default `http://localhost:4000/api`, overridable via `KATANAUTE_API_URL` or config file).
  - `api_token: Option<String>`.

In GTKata:

- Use `Config::load()` to create `AppState`.
- Use `Config::save_token` / `Config::clear_token` via `AppState` helpers.
- Allow `KATANAUTE_API_URL` to override the base URL; do not add extra env vars.

---

## Testing & TODOs

GTKata currently has no dedicated tests. If you add them:

- Keep pure API/auth tests in `katarustcore`; GTKata should test its own UI-specific logic only.
- Use Rust's built-in `#[cfg(test)]` and `#[test]` modules.
- Prefer small, focused tests around:
  - belt CSS class selection,
  - time parsing (`parse_time`),
  - simple view-model helpers you may introduce (e.g., formatting labels).

Known UX gaps you may be asked to work on (check main README/roadmap to sync):

- Session editing and deletion.
- Search/filtering, keyboard shortcuts.
- Toast notifications and better error surfaces.
- More robust empty/loading states.

When implementing roadmap items, keep this split:

- Cross-client behavior → `katarustcore`.
- GTK-specific presentation and interaction → GTKata.

---

## When In Doubt

- Check `katarustcore/AGENTS.md` for API/auth rules before changing network-facing behavior.
- Check `gtkata/README.md` for human-facing expectations.
- Keep GTKata simple, idiomatic, and aligned with other clients: it is a GNOME skin over shared behavior, not its own backend.
