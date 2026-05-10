# Design: desktop-plugin-cfg-guard

## Context

Tauri v2 supports mobile targets (Android, iOS). The `tauri-plugin-window-state` plugin saves and restores window position/size, which is a concept that doesn't exist on mobile. The crate's `Cargo.toml` is correct in making the dependency conditional. The missing piece is the corresponding `cfg` guard in the source code.

## Goals / Non-Goals

**Goals:**
- Make the app compile correctly for mobile targets
- Keep the window state plugin active on desktop targets

**Non-Goals:**
- Adding mobile-specific UI or behaviour
- Replacing `tauri-plugin-window-state` with a different plugin

## Decisions

**Use `#[cfg(not(any(target_os = "android", target_os = "ios")))]`**: This is the explicit form that matches the `Cargo.toml` condition exactly. Tauri v2 also provides a `#[cfg(desktop)]` shorthand alias; use whichever form matches the Tauri version in use.

**Wrap only the plugin line**: The `setup` closure and other builder calls are platform-agnostic and should not be touched.

## Risks / Trade-offs

- None on desktop. This change only affects compilation for mobile targets, which are currently untested.

## Open Questions

- Does the project intend to support mobile targets? If not, this is a low-priority but still-correct improvement.
