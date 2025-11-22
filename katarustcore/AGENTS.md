# AGENTS.md
1. Build the crate with `cargo build`; use `cargo check` for fast editor feedback.
2. Format via `cargo fmt --all` before commits; rustfmt defaults are mandatory.
3. Lint using `cargo clippy --all-targets --all-features -D warnings` to keep CI quiet.
4. Run all tests with `cargo test --all`; add `-- --nocapture` when inspecting logs.
5. Single test example: `cargo test config::tests::load_prefers_env -- --nocapture` (swap names as needed).
6. No Cursor or Copilot rule files exist, so this page is the canonical automation policy.
7. Prefer async functions returning `Result<T, Box<dyn std::error::Error>>` for anything touching IO.
8. Import order: crate modules, third-party crates, then std, each group separated by a blank line.
9. Module/files expose minimal structs and impls; re-export public types in `lib.rs` for consumers.
10. Structs/enums use UpperCamelCase, methods/functions use snake_case, and constants use SCREAMING_SNAKE_CASE.
11. Derive Debug/Clone/Serialize/Deserialize for every model crossing threads, disks, or the wire.
12. Prefer typed payload structs (see `SessionInput`) instead of HashMaps; annotate Options with serde helpers.
13. Error handling should convert HTTP failures into descriptive strings before propagating with `?`.
14. Store tokens/base URLs as `Option<String>` inside Config and sync them via dedicated setters/clearers.
15. Device-flow polling must respect the server interval, defaulting to 5s, and only suppress authorization_pending.
16. HTTP helpers build URLs with `format!` once per call and source the base path exclusively from Config.
17. Keep async loops cancellable; never block the Tokio runtime with std::thread sleeps.
18. Match expressions (e.g., `Kata::level_color`) handle every belt level plus a safe fallback color.
19. Serialization prefers pretty JSON for human-edited files; create directories before writing configs.