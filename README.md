# FullTime

A native desktop app for Bundesliga football league tables and scores, built with [GPUI](https://github.com/zed-industries/zed) (the UI framework behind Zed).

## Stack

- **UI**: [GPUI](https://github.com/zed-industries/zed) + [gpui-component](https://github.com/longbridge/gpui-component), in `crates/fulltime-ui`
- **App shell**: `crates/fulltime-core` — window/menu bootstrap, logging, the `FullTime` binary
- **Data**: [OpenLigaDB](https://www.openligadb.de/) REST API (planned; not yet wired into the UI)
- **i18n**: `rust-i18n`, locale catalogs under `crates/fulltime-ui/i18n/`

## Development

**Prerequisites**: Rust 1.95 (see `rust-toolchain.toml`)

```bash
# Run the app
cargo run -p fulltime-core

# Build the workspace
cargo build

# Lint and format
cargo clippy --all-targets
cargo +nightly fmt
```

## Architecture

`fulltime-ui` is the reusable GPUI application crate: theming (`data/theme.rs`), the header
nav (brand, league tabs, screen nav, light/dark toggle), and the five app screens (Standings,
Match, History, Player, Team) under `ui/views/`, built from shared shell components in
`ui/views/components/`. `fulltime-core` is a thin binary crate that boots the GPUI app and
opens the main window; its `[[bin]]` target is named `FullTime`.

The UI currently renders the Claude Design mockup's layout as an empty-state skeleton —
league/match/team data and the OpenLigaDB integration land in a later change.

**Binary entry**: `crates/fulltime-core/src/main.rs`
**UI root view**: `crates/fulltime-ui/src/ui/views/root_view.rs`
