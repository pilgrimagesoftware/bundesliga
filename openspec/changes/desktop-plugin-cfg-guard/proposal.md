# Proposal: app-desktop-plugin-cfg-guard

## Why

`app/src-tauri/Cargo.toml` correctly gates `tauri-plugin-window-state` behind a desktop-only platform condition:

```Fussballergebnisse/app/src-tauri/Cargo.toml#L1-3
[target.'cfg(not(any(target_os = "android", target_os = "ios")))'.dependencies]
tauri-plugin-window-state = "2"
```

However, `app/src-tauri/src/lib.rs` calls `.plugin(tauri_plugin_window_state::Builder::new().build())` unconditionally in `run()`. On a mobile build, the `tauri_plugin_window_state` crate is not compiled in, so the call to `tauri_plugin_window_state::Builder` will fail to resolve and the mobile build will not compile.

## What Changes

- Wrap the `.plugin(tauri_plugin_window_state::Builder::new().build())` call in `run()` with `#[cfg(not(any(target_os = "android", target_os = "ios")))]` (or the `#[cfg(desktop)]` alias if the Tauri version supports it)

## Capabilities

No behaviour change on desktop. Enables correct mobile compilation.

## Impact

- `app/src-tauri/src/lib.rs`: add `cfg` attribute or `cfg!` block around the `window_state` plugin registration in the `run()` function
