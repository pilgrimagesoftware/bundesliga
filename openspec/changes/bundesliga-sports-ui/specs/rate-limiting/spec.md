## ADDED Requirements

### Requirement: Per-category cooldown enforcement in backend
The system SHALL track the last successful fetch time per data category in Rust app state. If a command is invoked within the cooldown window, it SHALL return the most recently cached result with a metadata envelope indicating `{ cached: true, next_refresh_at: <epoch_ms> }`. It SHALL NOT make a network request.

Cooldown windows by category:
- `match_data`: 30 seconds
- `table`: 60 seconds
- `matchdays`: 5 minutes
- `team_detail`: 5 minutes

#### Scenario: Table command called within cooldown returns cached data
- **WHEN** `get_table` is invoked and the last successful fetch was 45 seconds ago (cooldown: 60s)
- **THEN** the backend returns the cached table data with `cached: true` and `next_refresh_at` set to 15 seconds from now

#### Scenario: Command called after cooldown makes a fresh request
- **WHEN** `get_table` is invoked and the last successful fetch was 90 seconds ago (cooldown: 60s)
- **THEN** the backend makes a fresh network request and updates the last-fetch timestamp on success

### Requirement: Refresh button disabled during cooldown
The frontend refresh button SHALL be disabled when the current view's data category is within its cooldown window. A "last updated X ago" label SHALL be displayed. The button SHALL re-enable automatically when the cooldown expires without requiring user interaction.

#### Scenario: Refresh button disabled immediately after refresh
- **WHEN** the user clicks the refresh button and data is fetched
- **THEN** the button becomes disabled and shows "last updated just now"

#### Scenario: Refresh button re-enables when cooldown expires
- **WHEN** the cooldown window elapses
- **THEN** the refresh button becomes active without requiring any user action (reactive to elapsed time)

### Requirement: Auto-refresh for match data
The app SHALL automatically re-fetch match data every 30 seconds when the matches view or match detail view is active. This auto-refresh SHALL respect the same cooldown as manual refresh; if the cooldown has not elapsed, the auto-refresh is silently skipped.

#### Scenario: Auto-refresh fires on schedule
- **WHEN** the matches view is active and 30 seconds have elapsed since last fetch
- **THEN** match data is automatically re-fetched in the background and the view updates

#### Scenario: Auto-refresh skipped during cooldown
- **WHEN** the auto-refresh timer fires but the cooldown has not yet elapsed
- **THEN** no network request is made and no visible update occurs
