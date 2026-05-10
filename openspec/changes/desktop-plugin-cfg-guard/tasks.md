# Tasks: app-desktop-plugin-cfg-guard

- [ ] 13.1 In `app/src-tauri/src/lib.rs` inside the `run()` function, locate the `.plugin(tauri_plugin_window_state::Builder::new().build())` line
- [ ] 13.2 Wrap the line with `#[cfg(not(any(target_os = "android", target_os = "ios")))]` or use a `cfg!`-gated block
- [ ] 13.3 Verify the correct attribute form by checking the Tauri v2 documentation for the `cfg(desktop)` alias availability in the project's pinned Tauri version
- [ ] 13.4 Run `cargo build` for the desktop target to confirm no regressions
- [ ] 13.5 If a mobile simulator is available, attempt `cargo build --target aarch64-apple-ios` (or Android equivalent) to confirm the mobile target now compiles
