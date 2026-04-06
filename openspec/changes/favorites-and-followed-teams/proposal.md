## Why

The app currently treats all teams equally, with no way to personalize the experience. Users who care about a specific team have to hunt for it every time — there's no "my team" concept. Adding a favorite team and followed teams lets the app surface what matters most to the user immediately.

## What Changes

- Add a **favorite team** concept: one team per user, persistently stored, visually highlighted with the Bundesliga red accent everywhere it appears
- Add a **followed teams** concept: a set of additional teams the user cares about, highlighted in a secondary accent (gold/amber) throughout the app
- Favorite/followed status is togglable directly from the league table, team grid, and team detail views
- A **My Teams** section appears at the top of the league table, pinning favorite + followed teams above the full table
- The **Matches view** gains a "My Teams" filter tab that shows only matches involving the user's favorite or followed teams
- Favorites and followed teams are persisted in the Rust app state (saved to `app_state.json` alongside existing view state)
- The **Header** shows a star icon for the current favorite team (if any)

## Capabilities

### New Capabilities

- `team-favorites`: Persistent favorite team selection (one team); Bundesliga-red highlight; star icon in header; pinned row in table; filter in matches view
- `team-following`: Persistent followed-teams set (multiple teams); gold highlight; pinned rows in table (below favorite); included in matches filter

### Modified Capabilities

- `app-state-persistence`: `AppViewState` struct gains `favorite_team_id: Option<i32>` and `followed_team_ids: Vec<i32>` fields; new Tauri commands to set/clear favorite and toggle followed status

## Impact

- `src-tauri/src/lib.rs`: Extend `AppViewState` struct; add `set_favorite_team`, `clear_favorite_team`, `toggle_followed_team` commands
- `src/types/AppViewState.ts`: Add `favorite_team_id`, `followed_team_ids` fields
- `src/lib/views/TableView.svelte`: Add pinned "My Teams" section above full table; highlight favorite (red) and followed (amber) rows
- `src/lib/views/MatchesView.svelte`: Add "My Teams" filter tab; persist filter state
- `src/lib/views/TeamDetailView.svelte`: Add star/follow toggle buttons in the identity section
- `src/lib/components/TeamCard.svelte`: Show favorite/followed indicators on team cards
- `src/lib/components/Header.svelte`: Show favorite team icon/name when set
- `src/lib/stores/context.svelte.ts`: Add reactive `favoriteTeamId` and `followedTeamIds` state; helpers to mutate and persist them
