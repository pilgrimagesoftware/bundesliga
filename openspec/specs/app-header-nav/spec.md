# app-header-nav Specification

## Purpose

TBD - created by archiving change ui-skeleton. Update Purpose after archive.

## Requirements

### Requirement: Persistent header replaces sidebar and toolbar
The system SHALL render a single persistent header at the top of the window, present identically across all five app views, replacing the prior sidebar and toolbar scaffold. The system SHALL NOT render a left sidebar.

#### Scenario: Header renders on every view
- **WHEN** the app is showing any of Standings, Match, History, Player, or Team
- **THEN** the same header (brand, league tabs, screen nav, theme toggle) is rendered at the top of the window

#### Scenario: No sidebar is rendered
- **WHEN** the app window renders in any state
- **THEN** no sidebar element occupies horizontal space to the left of the content area

### Requirement: Brand mark and wordmark
The header SHALL display a brand mark (circular logo) and the app wordmark ("Fulltime"), left-aligned, matching the mockup's brand block.

#### Scenario: Brand block is present
- **WHEN** the header renders
- **THEN** a circular brand mark and the text "Fulltime" appear at the header's leading edge

### Requirement: League tab bar
The header SHALL display a horizontal tab bar listing five leagues - EPL, LaLiga, Serie A, Bundesliga, Ligue 1 - with one league marked active/selected at a time. Selecting a league SHALL update the active league state without fetching or displaying any league-specific data.

#### Scenario: Five leagues are listed
- **WHEN** the header renders
- **THEN** the league tab bar shows exactly five tabs labeled EPL, LaLiga, Serie A, Bundesliga, and Ligue 1

#### Scenario: Selecting a league updates active state
- **WHEN** a user clicks an inactive league tab
- **THEN** that tab becomes the active tab and the previously active tab becomes inactive

### Requirement: Screen navigation control
The header SHALL provide a control for switching between the five app views (Standings, Match, History, Player, Team). Selecting a screen SHALL update the app's active-screen state and cause the content area to render that screen's empty-state layout.

#### Scenario: Switching screens updates content area
- **WHEN** a user selects a different screen via the header's navigation control
- **THEN** the active-screen state changes to the selected screen and the content area re-renders to show that screen's layout

### Requirement: Light/dark theme toggle
The header SHALL provide a toggle button that switches the active theme between `Pitch` (light) and `PitchNight` (dark), matching the mockup's sun/moon toggle.

#### Scenario: Toggling switches theme
- **WHEN** a user activates the theme toggle while `Pitch` is active
- **THEN** the active theme becomes `PitchNight` and all header/content colors update to the dark token set

### Requirement: No Style A/B segmented control
The header SHALL NOT render a Style A/B segmented control, since only Style A is supported.

#### Scenario: No style toggle is present
- **WHEN** the header renders
- **THEN** no control for switching between visual style variants is present
