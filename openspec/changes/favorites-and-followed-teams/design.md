## Context

The app already has persistent app state stored in `app_state.json` via the `AppViewState` Rust struct and the `get_last_viewed` / `save_last_viewed` Tauri commands. The frontend holds league/season context in `context.svelte.ts`. There is no team personalization — teams are displayed uniformly everywhere.

The new feature needs to:
1. Store user preferences (favorite + followed) persistently
2. Make those preferences reactive so all views update when they change
3. Apply visual highlighting consistently across TableView, MatchesView, TeamsView, TeamDetailView, and TeamCard

## Goals / Non-Goals

**Goals:**
- One favorite team (star, Bundesliga red highlight, pinned first in table)
- Multiple followed teams (heart, amber/gold highlight, pinned below favorite in table)
- Toggle UI available in table rows, team cards, and team detail header
- "My Teams" filter tab in matches view (shows only matches with favorite or followed teams)
- Persisted in `app_state.json` (piggybacking on existing persistence layer)
- Reactive: changing favorite/followed updates all open views immediately

**Non-Goals:**
- Per-league favorites (favorites are global across all leagues)
- Notifications or push alerts for favorite team matches
- Syncing preferences across devices
- More than one "favorite" (one star, many followed is the model)
- Custom color choices per team

## Decisions

### 1. Extend `AppViewState` rather than a separate file

**Decision**: Add `favorite_team_id: Option<i32>` and `followed_team_ids: Vec<i32>` to the existing `AppViewState` struct. Read/write via the existing `save_last_viewed` command — no new Rust commands needed for persistence.

**Rationale**: The persistence infrastructure already exists and works. A second JSON file adds complexity with no benefit. `AppViewState` is loaded at startup and the preferences will arrive at the same time as the last-viewed state.

**However**: The existing `save_last_viewed` command takes the full state. The frontend needs to read current state before updating it (to avoid wiping the view fields when only the favorite changes). Add two thin Tauri commands — `set_favorite_team(team_id: Option<i32>)` and `toggle_followed_team(team_id: i32)` — that perform a read-modify-write internally. This avoids the frontend having to reconstruct the full state struct on every toggle.

**Alternative considered**: Separate `prefs.json` file. Rejected — extra file, extra commands, no structural benefit.

### 2. Frontend state in `context.svelte.ts` (not a new store)

**Decision**: Add `favoriteTeamId: number | null` and `followedTeamIds: Set<number>` as `$state` values in the existing `context.svelte.ts` module. Expose `setFavoriteTeam`, `clearFavoriteTeam`, and `toggleFollowedTeam` helpers that call the Tauri commands and update local state.

**Rationale**: The context store is already the shared reactive state for league/season. Team preferences are another piece of "global context" — they belong there. Using a `Set<number>` for followed teams makes `has()` checks O(1) everywhere.

**Alternative considered**: Separate `favorites.svelte.ts` store. Rejected — creates an extra import everywhere for what is essentially one more field in the existing context.

### 3. Pinned rows via client-side sort (no new API calls)

**Decision**: In `TableView`, partition the table array into three buckets: `[favorite]`, `[followed]`, `[rest]` and render them in order. Visual separator between pinned and rest sections. No backend involvement.

**Rationale**: The table data is already fetched. Sorting/partitioning client-side is trivial with `$derived`. Adding a backend sort would require the backend to know about user preferences, coupling two concerns.

### 4. Matches filter as a tab (not a toggle)

**Decision**: Add a "My Teams" tab alongside the existing matchday view in `MatchesView`. The tab shows all matchdays but filters match cards to only those where `team1.id` or `team2.id` is in the user's favorite/followed set.

**Rationale**: A tab is the clearest UI affordance. Tabs preserve the existing matchday navigation while scoping the content. An inline toggle could be missed; a separate tab makes the feature discoverable.

### 5. Visual treatment

| State | Color | Element |
|---|---|---|
| Favorite | Bundesliga red (`#d20515`) | Left border + star icon (⭐) |
| Followed | Amber (`#f59e0b`) | Left border + heart icon (🤍 / 🧡) |
| Neither | Transparent border | — |

The same border + icon treatment applies in: table rows, team cards, team detail header.

## Risks / Trade-offs

**Migration of existing saved state** → `followed_team_ids` defaults to `[]` and `favorite_team_id` to `null` when not present in JSON (serde default). No migration needed.

**`followed_team_ids` vs `favorite_team_id` in a Set** → If a user somehow marks the same team as both favorite and followed (race condition between tabs), favorite takes priority in display. The backend `toggle_followed_team` command should silently no-op if the team is the current favorite.

**Table re-render on toggle** → Pinned rows change position when a team is favorited from the table. This is expected and acceptable; the row moves to the top immediately.

## Open Questions

- Should the favorite team's logo appear in the Header, or just a generic star? (Suggested: show the team name + small logo if a favorite is set — a nice personal touch.)
- Should the "My Teams" tab be visible even when no teams are favorited/followed? (Suggested: yes, with a "No followed teams yet" empty state — helps discovery.)
