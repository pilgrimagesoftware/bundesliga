## ADDED Requirements

### Requirement: User can set one favorite team
The system SHALL allow the user to designate exactly one team as their favorite. Setting a new favorite SHALL replace the previous one. The favorite team id SHALL be persisted across app restarts.

#### Scenario: Set favorite from table row
- **WHEN** user clicks the star icon on any team row in the league table
- **THEN** that team becomes the favorite and the star fills red (`#d20515`)

#### Scenario: Replace existing favorite
- **WHEN** user clicks the star icon on a different team while a favorite is already set
- **THEN** the previous team loses its favorite status and the new team becomes the favorite

#### Scenario: Clear favorite
- **WHEN** user clicks the star icon on the currently-favorited team
- **THEN** the favorite is cleared and no team has favorite status

#### Scenario: Set favorite from team detail
- **WHEN** user clicks the star/favorite button in the team detail identity section
- **THEN** that team becomes (or is cleared as) the favorite

#### Scenario: Set favorite from team card
- **WHEN** user clicks the star icon on a TeamCard in the teams grid
- **THEN** that team becomes (or is cleared as) the favorite

### Requirement: Favorite team is visually highlighted everywhere
The favorite team row/card/header SHALL be visually distinguished with a Bundesliga-red left border and a filled star icon (⭐) in all views where teams appear.

#### Scenario: Highlighted in league table
- **WHEN** a favorite team is set and the league table is displayed
- **THEN** the favorite team's row has a red left border and a ⭐ icon

#### Scenario: Highlighted in team grid
- **WHEN** a favorite team is set and the teams grid is displayed
- **THEN** the favorite team's card has a red border and ⭐ icon

#### Scenario: Highlighted in team detail
- **WHEN** viewing the team detail of the favorited team
- **THEN** the identity section shows a filled ⭐ button in red

### Requirement: Favorite team is pinned to the top of the league table
The system SHALL render the favorite team as the first row of the league table (above the "My Teams" separator), regardless of their league position.

#### Scenario: Pinned row renders first
- **WHEN** a favorite team is set and the table loads
- **THEN** the favorite team's row appears above all other rows, with their actual league position number still shown

#### Scenario: No pinning when no favorite
- **WHEN** no favorite team is set
- **THEN** the table renders in normal league-position order with no separator

### Requirement: Favorite team shown in the app header
When a favorite team is set, the app header SHALL display the team's logo and name next to the app title.

#### Scenario: Header displays favorite team
- **WHEN** a favorite team is set
- **THEN** the header shows the team's icon and name between the app title and the league picker

#### Scenario: Header is clean with no favorite
- **WHEN** no favorite team is set
- **THEN** the header shows only the app title, league picker, and season picker

### Requirement: Favorite team preference persists across restarts
The favorite team id SHALL be saved to `app_state.json` whenever it changes and restored on next app launch.

#### Scenario: Restored on startup
- **WHEN** the app launches and `app_state.json` contains a `favorite_team_id`
- **THEN** that team is immediately treated as the favorite in all views without user action
