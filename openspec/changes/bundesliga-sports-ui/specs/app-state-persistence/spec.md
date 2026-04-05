## ADDED Requirements

### Requirement: Last-viewed state saved on navigation
The system SHALL write the current view state to `<app_data_dir>/app_state.json` whenever the user navigates to a new view or changes the league/season selection. The saved state SHALL include: `last_opened` (UTC ISO 8601), `league`, `season`, `view`, `matchday` (nullable), `selected_team_id` (nullable).

#### Scenario: State saved on view change
- **WHEN** user navigates from the table view to the matches view
- **THEN** `app_state.json` is updated with `view: "matches"` and the current `matchday`

#### Scenario: State saved on league change
- **WHEN** user changes the league picker to a different league
- **THEN** `app_state.json` is updated with the new `league` value

### Requirement: Restore last-viewed state on startup
On startup the system SHALL read `app_state.json`. If `last_opened` is within 48 hours of the current time, the app SHALL restore the saved view state. If `last_opened` is older than 48 hours or the file does not exist, the app SHALL navigate to the current matchday of the default league.

#### Scenario: Recent session restored
- **WHEN** `app_state.json` exists and `last_opened` is 3 hours ago
- **THEN** the app opens to the saved view, league, season, and matchday without any redirect

#### Scenario: Stale session defaults to current matchday
- **WHEN** `app_state.json` exists and `last_opened` is 3 days ago
- **THEN** the app opens to the matches view for the current matchday of the default league (bl1)

#### Scenario: No saved state defaults to current matchday
- **WHEN** `app_state.json` does not exist (first launch)
- **THEN** the app opens to the matches view for the current matchday of bl1
