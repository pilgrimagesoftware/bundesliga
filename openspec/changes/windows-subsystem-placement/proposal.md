# Proposal: app-windows-subsystem-placement

## Why

`app/src-tauri/src/lib.rs` opens with `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`. This attribute tells the Windows linker to build a GUI application (no console window). It belongs exclusively in `main.rs` — the binary entry point — because it only has meaning for executable targets. `lib.rs` is a library crate; the attribute is silently ignored there and creates confusion about where the canonical platform configuration lives. The attribute already appears correctly in `main.rs`.

## What Changes

- Remove `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` from the top of `app/src-tauri/src/lib.rs`
- Confirm the attribute is present and correct in `app/src-tauri/src/main.rs` (it already is)

## Capabilities

No behaviour change — the attribute in `lib.rs` was always a no-op for library targets.

## Impact

- `app/src-tauri/src/lib.rs`: remove first line
