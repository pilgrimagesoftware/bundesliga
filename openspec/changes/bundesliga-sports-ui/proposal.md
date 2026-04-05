## Why

The app currently has a broken, unstyled proof-of-concept: the table never loads (command name mismatch), league/season selection does nothing (hardcoded backend state), and the UI has no visual design. This change rebuilds it into a real sports app — functional, fast, and visually consistent with how Bundesliga content is typically presented.

## What Changes

- **BREAKING** All Rust backend commands become stateless (league/season passed as params); `BundesligaState` hardcoding removed
- Fix `invoke("get_results")` → `invoke("get_table")` command name mismatch
- Install and configure Tailwind v4 with dark theme and Bundesliga red (`#d20515`) accent
- Resize window from 800×600 to 1200×800 (resizable, min 960×640)
- Replace Svelte 4 store pattern with Svelte 5 runes (`$state`, `$derived`, `$effect`)
- Add sidebar navigation: Table | Matches | Teams
- Add Matches view: matchday list with prev/next navigation, match cards with scores
- Add Match Detail view: goal timeline with scorer, minute, penalty/OG flags
- Add Teams view: team grid with click-through to team detail
- Add Team Detail view: table position, season stats, squad by position, staff/coaches, recent matches
- Integrate TheSportsDB free API for squad and staff data (team detail only)
- Add local JSON cache in Tauri app data dir for team data (30-day TTL) and app state
- Add rate limiting in Rust with per-category cooldowns; disable refresh button during cooldown
- Add persistent last-viewed state: restore on open if < 2 days elapsed, else default to current matchday
- Derive season list from current year (current + last 3); no API endpoint exists for this

## Capabilities

### New Capabilities

- `navigation`: Sidebar nav + in-page view state machine; header with league/season pickers
- `table-view`: League standings table with visual styling
- `matches-view`: Matchday browsing with match cards and prev/next navigation
- `match-detail`: Goal timeline drill-down for a single match
- `teams-view`: Team grid and team detail (squad, staff, stats, recent matches)
- `team-data-cache`: TheSportsDB integration with local JSON cache and TTL invalidation
- `app-state-persistence`: Last-viewed state saved to disk; restore-or-default-to-current logic
- `rate-limiting`: Per-category refresh cooldowns in Rust; UI feedback on cooldown state

### Modified Capabilities

*(none — no existing specs)*

## Impact

- `src-tauri/src/lib.rs`: Complete rewrite of commands; add TheSportsDB HTTP client; add cooldown tracker; add cache read/write
- `src-tauri/Cargo.toml`: Add `chrono`, `tokio`, `serde_json` (already present), `strsim` (fuzzy name matching)
- `src/routes/+page.svelte`: Full rewrite into new view components
- `src/types/`: Extend with new TypeScript interfaces (Match, Goal, Group, TeamDetail, etc.)
- `tauri.conf.json`: Window size, min dimensions
- `vite.config.js`, `src/app.css`: Tailwind v4 setup
- New dep: `pnpm add -D tailwindcss @tailwindcss/vite`
