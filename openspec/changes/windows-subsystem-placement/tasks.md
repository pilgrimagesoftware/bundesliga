# Tasks: app-windows-subsystem-placement

- [ ] 12.1 Remove the line `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` from the top of `app/src-tauri/src/lib.rs`
- [ ] 12.2 Confirm `app/src-tauri/src/main.rs` still has the attribute (do not remove it there)
- [ ] 12.3 Run `cargo build` in `app/src-tauri/` to confirm no compilation errors
