## ADDED Requirements

### Requirement: Team grid
The teams view SHALL display all teams for the selected league and season as a grid of cards. Each card SHALL show the team logo and name. Clicking a card SHALL navigate to the team detail view.

#### Scenario: Team grid shows all teams in the league
- **WHEN** the teams view loads for bl1/2024
- **THEN** 18 team cards are displayed

#### Scenario: Clicking a team card navigates to team detail
- **WHEN** user clicks a team card
- **THEN** the view state transitions to `{ screen: 'team_detail', teamId: <id> }`

### Requirement: Team detail — identity and season stats
The team detail view SHALL display: team logo, full name, founded year, stadium name and capacity, and the team's current row from the league table (P, W, D, L, GF, GA, GD, Pts, position).

#### Scenario: Team detail shows table position and stats
- **WHEN** team detail loads
- **THEN** the team's current league position and season statistics are visible

#### Scenario: Missing TheSportsDB data shows graceful fallback
- **WHEN** TheSportsDB data is unavailable or the name match confidence is below threshold
- **THEN** founded year and stadium fields are omitted and a "details unavailable" note is shown; the rest of the view renders normally using OpenLigaDB data

### Requirement: Team detail — squad by position
The team detail view SHALL display the team's squad grouped by position (Goalkeepers, Defenders, Midfielders, Forwards). Each player entry SHALL show: name, nationality flag/code, and date of birth (or age).

#### Scenario: Squad grouped by position
- **WHEN** TheSportsDB squad data is available
- **THEN** players are shown under position group headings in the order: GK, DF, MF, FW

#### Scenario: Squad unavailable shows placeholder
- **WHEN** TheSportsDB squad data cannot be fetched or matched
- **THEN** a "Squad data unavailable" placeholder is shown in the squad section

### Requirement: Team detail — staff and coaches
The team detail view SHALL display coaching staff entries from TheSportsDB, showing name and role for each staff member.

#### Scenario: Coaching staff shown when available
- **WHEN** TheSportsDB returns staff data for the team
- **THEN** a "Staff" section lists each staff member's name and role

### Requirement: Team detail — recent matches
The team detail view SHALL display the team's five most recent matches (finished) and next upcoming match (if any) with opponent, date, score, and home/away indicator.

#### Scenario: Recent matches listed with results
- **WHEN** team detail loads
- **THEN** up to five most recently finished matches are shown with scores and dates

#### Scenario: Next match shown when scheduled
- **WHEN** the team has an upcoming match
- **THEN** the next match is shown at the top of the recent matches section with kick-off date but no score
