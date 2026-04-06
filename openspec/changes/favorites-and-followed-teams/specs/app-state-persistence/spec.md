## ADDED Requirements

### Requirement: AppViewState includes favorite and followed team ids
The `AppViewState` struct (Rust) and `AppViewState` type (TypeScript) SHALL include `favorite_team_id: Option<i32>` and `followed_team_ids: Vec<i32>` fields. Both fields SHALL default gracefully when absent from the JSON file (null / empty array respectively).

#### Scenario: Existing state file without new fields loads without error
- **WHEN** `app_state.json` exists but does not contain `favorite_team_id` or `followed_team_ids`
- **THEN** the app loads with `favorite_team_id = null` and `followed_team_ids = []` without any error

#### Scenario: New fields are included in save
- **WHEN** `save_last_viewed` is called with a state object
- **THEN** the written JSON includes both `favorite_team_id` and `followed_team_ids`

### Requirement: Dedicated Tauri commands to mutate favorite and followed state
The system SHALL expose `set_favorite_team(team_id: Option<i32>)` and `toggle_followed_team(team_id: i32)` Tauri commands that perform atomic read-modify-write on `app_state.json`.

#### Scenario: set_favorite_team persists change
- **WHEN** `set_favorite_team` is invoked with a team id
- **THEN** `app_state.json` is updated with the new `favorite_team_id` and all other fields remain unchanged

#### Scenario: set_favorite_team with null clears favorite
- **WHEN** `set_favorite_team` is invoked with null
- **THEN** `app_state.json` is updated with `favorite_team_id = null`

#### Scenario: toggle_followed_team adds team if not present
- **WHEN** `toggle_followed_team` is invoked with a team id not in `followed_team_ids`
- **THEN** the team id is appended to `followed_team_ids` in `app_state.json`

#### Scenario: toggle_followed_team removes team if present
- **WHEN** `toggle_followed_team` is invoked with a team id already in `followed_team_ids`
- **THEN** the team id is removed from `followed_team_ids` in `app_state.json`

#### Scenario: toggle_followed_team ignores the favorite team
- **WHEN** `toggle_followed_team` is invoked with the current `favorite_team_id`
- **THEN** `followed_team_ids` is not modified
