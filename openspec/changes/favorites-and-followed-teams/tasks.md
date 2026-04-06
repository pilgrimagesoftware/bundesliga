## 1. Rust Backend — Extend AppViewState and Add Commands

- [ ] 1.1 Add `favorite_team_id: Option<i32>` and `followed_team_ids: Vec<i32>` to `AppViewState` struct in `lib.rs`; add `#[serde(default)]` on both fields so existing JSON without them deserializes cleanly
- [ ] 1.2 Add `set_favorite_team(team_id: Option<i32>, state: State<Mutex<AppState>>)` command: read `app_state.json` (or default), set `favorite_team_id`, remove team from `followed_team_ids` if present, write back
- [ ] 1.3 Add `toggle_followed_team(team_id: i32, state: State<Mutex<AppState>>)` command: read `app_state.json` (or default), skip if `team_id == favorite_team_id`, toggle presence in `followed_team_ids`, write back
- [ ] 1.4 Register `set_favorite_team` and `toggle_followed_team` in `tauri::generate_handler![]`

## 2. TypeScript Types

- [ ] 2.1 Add `favorite_team_id: number | null` and `followed_team_ids: number[]` fields to `src/types/AppViewState.ts`

## 3. Frontend State — context.svelte.ts

- [ ] 3.1 Add `favoriteTeamId = $state<number | null>(null)` and `followedTeamIds = $state<Set<number>>(new Set())` to `context.svelte.ts`
- [ ] 3.2 Export `getFavoriteTeamId()` and `getFollowedTeamIds()` getter functions
- [ ] 3.3 Export `setFavoriteTeam(teamId: number | null)` helper: invoke `set_favorite_team`, update local `favoriteTeamId`, remove from `followedTeamIds` if present
- [ ] 3.4 Export `toggleFollowedTeam(teamId: number)` helper: invoke `toggle_followed_team`, update local `followedTeamIds` (skip if `teamId === favoriteTeamId`)
- [ ] 3.5 Load `favorite_team_id` and `followed_team_ids` from saved state in `+page.svelte` startup logic and call context setters to seed initial reactive state

## 4. Header — Favorite Team Display

- [ ] 4.1 Accept a `favoriteTeam: { id: number; name: string | null; icon_url: string | null } | null` prop in `Header.svelte`
- [ ] 4.2 Render the favorite team's icon + name between the app title and league picker when `favoriteTeam` is set; hide when null
- [ ] 4.3 Pass the current favorite team object from `+page.svelte` to `Header` (derived from the table data + `getFavoriteTeamId()`)

## 5. League Table — Pinned Rows and Toggle Icons

- [ ] 5.1 Import `getFavoriteTeamId`, `getFollowedTeamIds`, `setFavoriteTeam`, `toggleFollowedTeam` in `TableView.svelte`
- [ ] 5.2 Create `$derived` values: `pinnedFavorite` (the one matching `favoriteTeamId`), `pinnedFollowed` (those matching `followedTeamIds`, sorted by league position), `rest` (the remainder)
- [ ] 5.3 Render table in order: pinned favorite row → pinned followed rows → divider row (`colspan` spanning all columns) → rest rows; only show divider when at least one row is pinned
- [ ] 5.4 Add a star icon column (far right of each row): filled red ⭐ for favorite, unfilled ☆ for others; clicking toggles `setFavoriteTeam`
- [ ] 5.5 Add a heart icon column (right of star): filled amber 🧡 for followed, unfilled 🤍 for others (hidden if team is favorite); clicking toggles `toggleFollowedTeam`
- [ ] 5.6 Apply `border-l-4 border-[var(--color-bundesliga-red)]` to favorite row (overriding zone color)
- [ ] 5.7 Apply `border-l-4 border-amber-400` to followed rows (overriding zone color)

## 6. Team Card — Favorite/Followed Indicators

- [ ] 6.1 Import `getFavoriteTeamId`, `getFollowedTeamIds` in `TeamCard.svelte`
- [ ] 6.2 Show ⭐ badge (red) on card if `team.id === getFavoriteTeamId()`
- [ ] 6.3 Show 🧡 badge (amber) on card if `getFollowedTeamIds().has(team.id)`
- [ ] 6.4 Apply `border-[var(--color-bundesliga-red)]` to favorite card border; `border-amber-400` to followed card border (replacing default `border-[var(--color-border)]`)

## 7. Team Detail — Favorite/Followed Toggle Buttons

- [ ] 7.1 Import `getFavoriteTeamId`, `getFollowedTeamIds`, `setFavoriteTeam`, `toggleFollowedTeam` in `TeamDetailView.svelte`
- [ ] 7.2 Add ⭐/☆ button in the identity section header: filled red if favorite, unfilled otherwise; clicking calls `setFavoriteTeam(teamId)` or `setFavoriteTeam(null)` to toggle
- [ ] 7.3 Add 🧡/🤍 button next to the star (hidden if this team is the favorite): filled amber if followed; clicking calls `toggleFollowedTeam(teamId)`

## 8. Matches View — "My Teams" Filter Tab

- [ ] 8.1 Add a tab bar above match cards in `MatchesView.svelte` with two tabs: "All Matches" (default) and "My Teams"
- [ ] 8.2 Add `activeTab = $state<'all' | 'my_teams'>('all')` local state
- [ ] 8.3 When `activeTab === 'my_teams'`, filter `matches` to only those where `team1.id` or `team2.id` is `getFavoriteTeamId()` or is in `getFollowedTeamIds()`
- [ ] 8.4 Show "No followed teams yet — star or follow teams to track them here" empty state when `activeTab === 'my_teams'` and both `favoriteTeamId` is null and `followedTeamIds` is empty
- [ ] 8.5 Show "None of your teams play this matchday" empty state when `activeTab === 'my_teams'`, teams are set, but filtered matches list is empty
- [ ] 8.6 Persist `activeTab` across matchday navigation (keep "My Teams" selected when user navigates prev/next)
