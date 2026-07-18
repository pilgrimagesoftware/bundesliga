## Context

The app is a native GPUI desktop app (`crates/fulltime-core` binary + `crates/fulltime-ui` library), scaffolded from the Libri template. Unlike the Tauri/SvelteKit predecessor, there is no IPC boundary and no separate frontend process — `fulltime-ui` renders views directly against in-process data. The current state is an empty themed shell: title bar, an empty sidebar stub, an empty toolbar stub, a status bar stub, and no data layer. This design covers the full build-out into the real sports app.

## Goals / Non-Goals

**Goals:**
- Introduce a three-view layout (Table, Matches, Teams) with sidebar navigation
- Integrate TheSportsDB for rich team data (squad, staff) with local caching
- Add persistent last-viewed state with "current matchday fallback" after 2-day absence
- Add rate limiting to prevent API flooding
- Extend the existing `FullTimeTheme`/`ColorTokens` system (see `crates/fulltime-ui/src/data/theme.rs`) with the zone-accent colors standings tables need

**Non-Goals:**
- Live push/websocket updates (polling at fixed intervals is sufficient)
- User accounts, authentication, or sync across machines
- Player statistics beyond top scorers and goal events
- Historical seasons beyond current year − 3
- Mobile/web deployment

## Decisions

### 1. In-process data layer, no command/IPC pattern

**Decision**: League/season/table/match/team data is fetched by async functions in `crates/fulltime-ui/src/data/`, called directly from GPUI views (via `cx.spawn` background tasks that update view state on completion). There is no Tauri-style "command" abstraction and no serialization boundary between fetch and render.

**Rationale**: GPUI is a single process; the Tauri app's `#[tauri::command]` functions existed only to cross the IPC boundary to the SvelteKit frontend. That boundary no longer exists, so the equivalent Rust logic (calls into the `openligadb` crate, TheSportsDB `reqwest` calls) can be plain async functions returning typed Rust structs directly to the view layer.

**Alternative considered**: Keep a command-dispatch abstraction for symmetry with the old design. Rejected — it would reintroduce serialization/deserialization and a dispatch layer with no boundary to justify it.

### 2. In-window view state machine via a GPUI global

**Decision**: A `NavState` GPUI global (or a field on `RootView`, see `crates/fulltime-ui/src/ui/views/root_view.rs`) drives which view renders in the main-content area:

```rust
enum NavScreen {
    Table,
    Matches { matchday: u32 },
    MatchDetail { match_id: u64, from_matchday: u32 },
    Teams,
    TeamDetail { team_id: u64 },
}
```

**Rationale**: There is no URL bar and no router in a native GPUI window. A plain Rust enum held in view/global state is transparent, easy to persist (serde), and mirrors the `AppView` union the Tauri design already used — same shape, no IPC framing needed.

**Alternative considered**: A trait-object-based view stack (push/pop navigation). Rejected for this change — a flat enum covers every screen this app needs today; a stack can be introduced later if nested drill-down grows deeper.

### 3. Theme carries the zone-accent and status colors

**Decision**: Extend `ColorTokens` (`crates/fulltime-ui/src/data/theme.rs`) with `zone_champions_league`, `zone_europa_league`, `zone_relegation`, and `live_indicator` fields, following the same `hex`/`hex_a` constructor pattern already used for `pitch_colors`/`pitch_night_colors`.

**Rationale**: The existing theme module is already the single source of truth gpui-component widgets sync against (`apply_theme_colors`). Adding a few semantic fields there keeps the standings table's zone accents theme-aware (light/dark) instead of hardcoding colors in the table view.

**Alternative considered**: Hardcode zone colors directly in the table view. Rejected — breaks dark-mode support and duplicates color decisions the theme module already owns.

### 4. Team detail renders from OpenLigaDB first, then enriches with TheSportsDB

**Decision**: The team detail screen must be able to render meaningfully from OpenLigaDB data alone. TheSportsDB is an enrichment layer for founded year, stadium metadata, squad, and staff, not a prerequisite for the existence of the view.

**Rationale**: This keeps the Teams vertical slice shippable even if third-party matching fails or TheSportsDB integration is deferred. It also separates a deterministic baseline experience (identity, table stats, recent matches) from the highest-risk part of the project (cross-provider name matching and external-cache behavior).

**Alternative considered**: Make team detail entirely dependent on TheSportsDB-backed aggregation before the screen exists. Rejected — it couples the riskiest external integration to a core navigation path and delays a usable Teams flow.

### 5. TheSportsDB as secondary data source with name-fuzzy matching

**Decision**: When team detail is first requested, search TheSportsDB by team name (`/searchteams.php?t=`). Cache the `{openligadb_id → thesportsdb_id}` mapping alongside the team detail JSON. On subsequent requests, look up by cached ID directly.

**Fuzzy matching**: Use the `strsim` crate (Jaro-Winkler distance) to find the best match when the exact name search returns no results or multiple results. Accept the top candidate if score > 0.85.

**Rationale**: OpenLigaDB uses German names ("FC Bayern München"); TheSportsDB uses English names ("Bayern Munich"). Exact match will fail regularly. Fuzzy matching with a confidence threshold handles this automatically without a hardcoded name map.

**Alternative considered**: Hardcoded name mapping table. Rejected — brittle, requires maintenance on team promotions/relegations.

### 6. Local JSON cache under the platform app-data directory

**Decision**: Two cache concerns, both under the same platform-specific directory `fulltime-core::logging::platform_log_dir` already establishes for logs (`~/Library/Application Support/com.pilgrimagesoftware.fulltime/` on macOS, etc. — a sibling `data` directory rather than `logs`):
- `app_state.json` — last viewed state (view, league, season, matchday, team ID, timestamp)
- `team_cache/{openligadb_team_id}.json` — TheSportsDB team detail + squad + staff

Cache TTL: 30 days for team detail. On load, check `std::fs::metadata().modified()` against `SystemTime::now()`. If stale or missing, fetch fresh.

**Rationale**: `dirs::data_dir()` (already a workspace dependency) gives the correct OS-specific path. Plain JSON files are transparent, debuggable, and require no additional dependencies beyond `serde_json`.

**Alternative considered**: An embedded database (sqlite, sled). Rejected — the data volume (one JSON file per team, one small state file) doesn't need query capability; plain files stay debuggable by hand.

### 7. Rate limiting via per-category cooldowns in a shared data-layer state

**Decision**: A `DataCache` struct (owned by a GPUI global or an `Entity`) holds a `HashMap<&'static str, Instant>` for last-fetched timestamps. Each data category has a minimum interval:

| Category | Minimum interval |
|---|---|
| `match_data` | 30 seconds |
| `table` | 60 seconds |
| `matchdays` | 5 minutes |
| `team_detail` | 5 minutes (manual; cache is 30 days) |

If a fetch is requested within the cooldown window, the data layer returns the last cached value with a `{ cached: true, next_refresh_at: Instant }` envelope instead of making a network request. The refresh control disables and shows "last updated X ago".

**Rationale**: A simple `HashMap<&str, Instant>` requires no external crate and is trivially correct — same design as the original Tauri `AppState`, just held in a GPUI entity instead of behind `tauri::State`.

### 8. Season list derived from current year

**Decision**: The season list is `[current_year, current_year-1, current_year-2, current_year-3]`, computed via `chrono::Local::now().year()`. No API call.

**Rationale**: OpenLigaDB has no "available seasons" endpoint. Deriving from the current year covers all realistic use cases without hardcoding. Seasons with no data will return empty arrays gracefully (the API returns `[]`, not an error).

## Risks / Trade-offs

**TheSportsDB free tier rate limits** → Mitigation: the 5-minute manual cooldown and 30-day cache mean the app makes at most a handful of TheSportsDB calls per session. Well within free tier limits.

**TheSportsDB name matching failure** → Mitigation: If fuzzy match score < 0.85, the team detail view shows only the data available from OpenLigaDB (table stats, matches) with a "squad data unavailable" placeholder. No crash.

**Season data gaps** → Mitigation: If the table or matchday fetch returns empty for a derived season, the view shows an empty state ("No data for this season") rather than an error.

**Stale squad data during transfer window** → Mitigation: Manual refresh (with 5-minute cooldown) lets the user force a cache invalidation. The 30-day TTL will also naturally expire.

**Large match dataset on initial load** → Fetching all matches for a full season returns ~306 matches. This is loaded once for the matches view and grouped by matchday in `fulltime-ui`. Consider lazy loading per matchday if performance is a concern (deferred to future change).

**Background task lifetime vs. view lifetime** → GPUI's `cx.spawn` tasks must be dropped/cancelled if the owning view is dropped (e.g., user navigates away mid-fetch) to avoid updating a stale entity. Each view's fetch task should be held as a field and replaced (dropping the old one) on navigation.

## Open Questions

- Should the live indicator reflect *any* ongoing match in the current league, or only the selected matchday? (Suggested: any match in the league that is `!is_finished && within 2h of kick-off`)
- Should relegated/promoted teams from prior seasons (not in current OpenLigaDB season) show a "no longer in Bundesliga" badge on the team detail? (Probably nice, but not blocking.)
