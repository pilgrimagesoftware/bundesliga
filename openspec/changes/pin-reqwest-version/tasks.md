# Tasks: app-pin-reqwest-version

- [ ] 10.1 Inspect `app/src-tauri/Cargo.lock` to find the currently resolved `reqwest` version
- [ ] 10.2 Replace `reqwest = { version = "*", features = ["json"] }` in `app/src-tauri/Cargo.toml` with the pinned major version (e.g., `reqwest = { version = "0.13", features = ["json"] }`)
- [ ] 10.3 Run `cargo build` in `app/src-tauri/` to confirm the crate still compiles with the pinned version
- [ ] 10.4 Confirm `cargo update` no longer changes the resolved `reqwest` version by checking the lock file
