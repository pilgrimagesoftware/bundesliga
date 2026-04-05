## ADDED Requirements

### Requirement: TheSportsDB team lookup with name matching
The system SHALL fetch team detail from TheSportsDB when team detail is first requested. The lookup SHALL first attempt an exact name search; if no match is found or the top result's name similarity score is below 0.85 (Jaro-Winkler), the system SHALL return a cache-miss result and the team detail view SHALL render without squad/staff data.

#### Scenario: Exact name match succeeds
- **WHEN** TheSportsDB search returns a team with name similarity ≥ 0.85
- **THEN** that team's data is used and cached

#### Scenario: No match above threshold
- **WHEN** no TheSportsDB result has similarity ≥ 0.85
- **THEN** the backend returns a response with `squad: null, staff: null` and `source: "openligadb_only"`

### Requirement: Local JSON cache for team detail
The system SHALL store TheSportsDB responses as JSON files in `<app_data_dir>/team_cache/<openligadb_team_id>.json`. Each cache file SHALL include the fetched data and a `cached_at` ISO 8601 timestamp.

#### Scenario: Cache file written on first fetch
- **WHEN** a TheSportsDB response is successfully fetched
- **THEN** a JSON file is written to the team_cache directory with the response and a `cached_at` field

#### Scenario: Cache file read on subsequent requests within TTL
- **WHEN** a team detail is requested and a cache file exists with `cached_at` within 30 days
- **THEN** the cached data is returned without making a network request to TheSportsDB

#### Scenario: Cache file invalidated after TTL
- **WHEN** a team detail is requested and the cache file's `cached_at` is older than 30 days
- **THEN** a fresh TheSportsDB request is made and the cache file is overwritten

### Requirement: ID mapping cache
The system SHALL cache the `{openligadb_team_id → thesportsdb_team_id}` mapping within the team cache file to avoid repeated search queries on subsequent requests within TTL.

#### Scenario: Subsequent requests use cached ID
- **WHEN** a team's TheSportsDB ID is already in the cache file
- **THEN** the system uses the lookup-by-ID endpoint directly, not the search endpoint
