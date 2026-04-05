## Context

The app is a Tauri v2 desktop application (Rust backend, SvelteKit/Svelte 5 frontend). The current state is a single broken page: the table never loads due to a command name mismatch, league/season selection is wired to a hardcoded Rust state struct, and the frontend uses Tailwind class names without Tailwind installed. This design covers the full rebuild.

## Goals / Non-Goals

**Goals:**
- Fix the two existing bugs blocking the table view
- Introduce a three-view layout (Table, Matches, Teams) with sidebar navigation
- Integrate TheSportsDB for rich team data (squad, staff) with local caching
- Add persistent last-viewed state with "current matchday fallback" after 2-day absence
- Add rate limiting in Rust to prevent API flooding
- Establish dark theme with Bundesliga red accent via Tailwind v4

**Non-Goals:**
- Live push/websocket updates (polling at fixed intervals is sufficient)
- User accounts, authentication, or sync across machines
- Player statistics beyond top scorers and goal events
- Historical seasons beyond current year − 3
- Mobile/web deployment

## Decisions

### 1. Stateless Rust commands (params over state)

**Decision**: Remove `BundesligaState` and pass `league`, `season`, and other params directly to each command.

**Rationale**: The current pattern of a single hardcoded state struct means the UI selection has no effect on the backend. Stateless commands are simpler to test and reason about. The only mutable state the backend needs is the cooldown tracker and the app data dir path — both held in a new `AppState` struct behind `Arc<Mutex<>>`.

**Alternative considered**: Keep `BundesligaState` but make it mutable and update it via a `set_context` command. Rejected — adds unnecessary synchronization complexity and an extra round-trip on every selection change.

### 2. In-page view state machine (not SvelteKit routing)

**Decision**: A single `$state` object in the root component drives which view is rendered. No SvelteKit file-based routes.

```typescript
type AppView =
  | { screen: 'table' }
  | { screen: 'matches'; matchday: number }
  | { screen: 'match_detail'; matchId: number; fromMatchday: number }
  | { screen: 'teams' }
  | { screen: 'team_detail'; teamId: number }
```

**Rationale**: Tauri desktop apps have no URL bar. SvelteKit routing adds a layer of indirection (hash routing or memory adapter) with no benefit for a single-window app. A plain state object is transparent, easy to persist, and trivially serializable for the last-viewed feature.

**Alternative considered**: SvelteKit `@sveltejs/adapter-static` with hash routing. Rejected — routing conventions (back button, link handling) are browser idioms that feel wrong in a desktop app.

### 3. Tailwind v4 with CSS custom properties for theming

**Decision**: Install `tailwindcss` + `@tailwindcss/vite`. Define the color palette via CSS custom properties in `app.css` under a `@theme` block.

```css
@import "tailwindcss";
@theme {
  --color-bundesliga-red: #d20515;
  --color-surface: #0d0d0d;
  --color-surface-elevated: #1a1a1a;
  --color-border: #2a2a2a;
}
```

**Rationale**: Tailwind v4 has zero config files; all theming is in CSS. The `@tailwindcss/vite` plugin handles the Vite integration. This removes the need for `tailwind.config.js` and keeps all styling concerns in one place.

### 4. TheSportsDB as secondary data source with name-fuzzy matching

**Decision**: When team detail is first requested, search TheSportsDB by team name (`/searchteams.php?t=`). Cache the `{openligadb_id → thesportsdb_id}` mapping alongside the team detail JSON. On subsequent requests, look up by cached ID directly.

**Fuzzy matching**: Use the `strsim` crate (Jaro-Winkler distance) to find the best match when the exact name search returns no results or multiple results. Accept the top candidate if score > 0.85.

**Rationale**: OpenLigaDB uses German names ("FC Bayern München"); TheSportsDB uses English names ("Bayern Munich"). Exact match will fail regularly. Fuzzy matching with a confidence threshold handles this automatically without a hardcoded name map.

**Alternative considered**: Hardcoded name mapping table. Rejected — brittle, requires maintenance on team promotions/relegations.

### 5. Local JSON cache in app data dir

**Decision**: Two cache concerns:
- `app_state.json` — last viewed state (view, league, season, matchday, team ID, timestamp)
- `team_cache/{openligadb_team_id}.json` — TheSportsDB team detail + squad + staff

Cache TTL: 30 days for team detail. On load, check `std::fs::metadata().modified()` against `SystemTime::now()`. If stale or missing, fetch fresh.

**Rationale**: `tauri::Manager::app_handle().path().app_data_dir()` gives the correct OS-specific path (e.g., `~/Library/Application Support/football-scores/` on macOS). Plain JSON files are transparent, debuggable, and require no additional dependencies beyond `serde_json`.

**Alternative considered**: `tauri-plugin-store`. Rejected — adds a dependency for functionality that `serde_json` + `std::fs` handles cleanly.

### 6. Rate limiting via per-category cooldowns in AppState

**Decision**: `AppState` holds a `HashMap<&'static str, Instant>` for last-fetched timestamps. Each command category has a minimum interval:

| Category | Minimum interval |
|---|---|
| `match_data` | 30 seconds |
| `table` | 60 seconds |
| `matchdays` | 5 minutes |
| `team_detail` | 5 minutes (manual; cache is 30 days) |

If a command is called within the cooldown window, return the cached/last value with an `{ cached: true, next_refresh_at: <epoch_ms> }` envelope. The frontend disables the refresh button and shows "last updated X ago".

**Rationale**: A simple HashMap<str, Instant> requires no external crate and is trivially correct. The cooldown is enforced on the backend so it cannot be bypassed by frontend bugs.

### 7. Season list derived from current year

**Decision**: `get_seasons` returns `[current_year, current_year-1, current_year-2, current_year-3]`. No API call. The backend uses `chrono::Local::now().year()`.

**Rationale**: OpenLigaDB has no "available seasons" endpoint. Deriving from the current year covers all realistic use cases without hardcoding. Seasons with no data will return empty arrays gracefully (the API returns `[]`, not an error).

## Risks / Trade-offs

**TheSportsDB free tier rate limits** → Mitigation: the 5-minute manual cooldown and 30-day cache mean the app makes at most a handful of TheSportsDB calls per session. Well within free tier limits.

**TheSportsDB name matching failure** → Mitigation: If fuzzy match score < 0.85, the team detail view shows only the data available from OpenLigaDB (table stats, matches) with a "squad data unavailable" placeholder. No crash.

**Season data gaps** → Mitigation: If `get_table` or `get_matchdays` returns empty for a derived season, the frontend shows an empty state ("No data for this season") rather than an error.

**Stale squad data during transfer window** → Mitigation: Manual refresh button (with 5-minute cooldown) lets the user force a cache invalidation. The 30-day TTL will also naturally expire.

**Large match dataset on initial load** → `Match::by_league(league, season)` returns all matches for a full season (~306 matches). This is loaded once for the matches view and grouped by matchday client-side. Consider lazy loading per matchday if performance is a concern (deferred to future change).

## Open Questions

- Should the live indicator in the header reflect *any* ongoing match in the current league, or only the selected matchday? (Suggested: any match in the league that is `!is_finished && within 2h of kick-off`)
- Should relegated/promoted teams from prior seasons (not in current OpenLigaDB season) show a "no longer in Bundesliga" badge on the team detail? (Probably nice, but not blocking.)
