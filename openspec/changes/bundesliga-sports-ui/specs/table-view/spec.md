## ADDED Requirements

### Requirement: League standings table
The app SHALL display a league standings table when the Table nav item is active. The table SHALL show: position, team logo, team name, matches played, wins, draws, losses, goals for, goals against, goal difference, and points. Rows SHALL be sorted by points descending (then goal difference, then goals for).

#### Scenario: Table renders with correct columns
- **WHEN** the table view is active and data has loaded
- **THEN** each row displays position number, team logo image, team name, P, W, D, L, GF, GA, GD, Pts in that order

#### Scenario: Table rows sorted by points
- **WHEN** table data is returned from the backend
- **THEN** rows are ordered by points descending; ties broken by goal difference then goals scored

### Requirement: Visual zone indicators
The table SHALL visually distinguish promotion zones, European competition zones, and relegation zones using colored left-border accents on rows, consistent with standard Bundesliga table presentation.

#### Scenario: Top two rows show Champions League accent
- **WHEN** the table is rendered
- **THEN** rows 1 and 2 display a distinct left-border color indicating Champions League qualification

#### Scenario: Bottom two rows show relegation accent
- **WHEN** the table is rendered
- **THEN** rows 17 and 18 display a red left-border color indicating relegation

### Requirement: Team row navigates to team detail
Clicking a team row in the table SHALL navigate to the team detail view for that team.

#### Scenario: Click team row opens team detail
- **WHEN** user clicks a team row in the standings table
- **THEN** the view state transitions to `{ screen: 'team_detail', teamId: <id> }`
