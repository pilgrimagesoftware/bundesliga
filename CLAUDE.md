# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Tauri v2 desktop app for displaying Bundesliga football league tables and scores. Built with a Rust backend (via Tauri) and a SvelteKit/TypeScript frontend. Data comes from the free OpenLigaDB REST API via the `openligadb` Rust crate.

## Development Commands

```bash
# Full dev mode (starts Vite + Tauri together)
pnpm tauri dev

# Frontend only
pnpm dev

# Production build (Tauri binary)
pnpm tauri build

# TypeScript + Svelte type checking
pnpm check
pnpm check:watch
```

Rust-specific: `cargo test`, `cargo clippy`, `cargo fmt` run from `src-tauri/`.

## Architecture

### Frontend (SvelteKit + Svelte 5)
- **Entry**: `src/routes/+page.svelte` — single-page UI with league/season dropdowns and table
- **Config**: Static Site Generation (SSG) — no SSR; output goes to `../build` for Tauri to serve
- **Communication**: Calls Rust via `invoke()` from `@tauri-apps/api/core`
- **Types**: `src/types/` — `League`, `TableTeam`, `Sport` interfaces

### Backend (Rust / Tauri)
- **Entry**: `src-tauri/src/lib.rs` — all Tauri commands and `BundesligaState` struct
- **Commands exposed to frontend**: `get_leagues()`, `get_seasons()`, `get_table()`
- **Data source**: `openligadb` crate (wraps the OpenLigaDB API for German football)
- **Plugins**: `tauri-plugin-log`, `tauri-plugin-window-state` (desktop), `tauri-plugin-opener`

### Frontend ↔ Backend Flow
1. On mount: `get_leagues()` — populate league dropdown
2. On league select: `get_seasons()` — returns hardcoded `[2024]`
3. On season select: `get_table()` — fetch standings; auto-refreshes every 30s

## Key Configuration
- Vite dev server: port `1420` (required by Tauri)
- Window: 800×600, title "Football Scores"
- CSP: disabled (`null` in `tauri.conf.json`)
- Node: 22.9.0 (`.node-version`), Rust: 1.82 (`.rust-version`)
- Capabilities: `src-tauri/capabilities/` — controls Tauri API permissions
