# AGENTS.md

This file provides guidance to Claude Code (claude.ai/code), Codex (openai.com/codex/), GitHub Copilot (copilot.github.com) when working with code in this repository.

## Project Overview

A native desktop app for Bundesliga football league tables and scores, built with GPUI (the UI
framework behind Zed) and `gpui-component`. Data comes from the OpenLigaDB REST API (planned,
not yet wired into the UI). See `README.md` for the workspace layout, dev commands, and
architecture overview.

## Gotchas

- `gpui`/`gpui_platform` are pinned directly against `zed-industries/zed` at a specific `rev` in
  the root `Cargo.toml`, not just pulled in transitively through `gpui-component`. `gpui-component`
  pins its own `gpui`/`gpui_platform` rev independently in its own `Cargo.toml` - the two revs are
  not the same and updating one doesn't update the other.
- `gpui-component` ships widgets that overlap in name but not behavior with hand-rolled
  fulltime-ui code: `gpui_component::badge::Badge` is a notification-count/dot overlay, not the
  same thing as an initials-based avatar badge (that maps to `gpui_component::avatar::Avatar`
  instead). Check the actual widget behavior, not just the name, before assuming a match.
- `gpui-component` widgets read colors from `cx.theme()` (`gpui_component::Theme`), a separate
  global from this app's own `FullTimeTheme`. The two are synced only via
  `data::theme::apply_theme_colors`, called on theme application - a `gpui-component` widget
  using a semantic color field that function doesn't map yet will silently render
  `gpui-component`'s built-in default instead of the active FullTime palette.
- The workspace denies `unwrap()`/`expect()` and `unsafe` via `[workspace.lints]` in the root
  `Cargo.toml` — this applies to every crate in the workspace, not just `fulltime-core`.
- `crates/fulltime-plugin-fixture` is excluded from `cargo build`/`cargo test`'s default set
  (workspace `default-members`) — it only builds for `wasm32-wasip2`, exercised by
  `fulltime-core`'s plugin host tests, not by a normal native build.
- `plugins/bundesliga` is a git submodule with its own `Cargo.lock`/toolchain, excluded from the
  workspace entirely (`exclude` in the root `Cargo.toml`) — it's vendored only so its source is
  available for `scripts/vendor-bundesliga-plugin.sh` to build to `wasm32-wasip2`.
