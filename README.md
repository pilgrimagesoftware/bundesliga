# Bundesliga

A Tauri v2 desktop app for displaying Bundesliga football league tables and scores.

## Stack

- **Frontend**: SvelteKit + Svelte 5 + TypeScript (Static Site Generation, served by Tauri)
- **Backend**: Rust via Tauri v2
- **Data**: [OpenLigaDB](https://www.openligadb.de/) REST API via the `openligadb` Rust crate

## Development

**Prerequisites**: Node 22.9.0, Rust 1.82, pnpm

```bash
# Full dev mode (Vite + Tauri)
cargo tauri dev

# Frontend only
pnpm dev

# Type checking
pnpm check
pnpm check:watch

# Production build
cargo tauri build
```

Rust: `cargo test`, `cargo clippy`, `cargo fmt` — run from `src-tauri/`.

## Architecture

The frontend calls Rust via Tauri's `invoke()` API. On load, it fetches the league list, then seasons, then standings. The table auto-refreshes every 30 seconds.

**Tauri commands**: `get_leagues()`, `get_seasons()`, `get_table()`

**Frontend entry**: `src/routes/+page.svelte`
**Backend entry**: `src-tauri/src/lib.rs`
