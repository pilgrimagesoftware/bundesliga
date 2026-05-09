## ADDED Requirements

### Requirement: Searchable season picker

The system SHALL provide a season picker that allows users to search available seasons by visible season label.

#### Scenario: Search narrows the season list

- **WHEN** the user opens the season picker and enters text that matches one or more season labels
- **THEN** the picker displays only seasons whose visible labels match the search text

#### Scenario: Search has no matches

- **WHEN** the user enters text that matches no available season label
- **THEN** the picker displays an empty state indicating that no seasons match

### Requirement: Filterable season picker

The system SHALL provide controls that reduce the visible season list to a desired subset.

#### Scenario: Preset filter is applied

- **WHEN** the user selects a preset season filter
- **THEN** the picker displays only seasons that satisfy that preset

#### Scenario: Custom year range filter is applied

- **WHEN** the user enters a custom year range
- **THEN** the picker displays only seasons whose parsed year or year range intersects the custom range

#### Scenario: Filters can be cleared

- **WHEN** the user clears the active filter
- **THEN** the picker returns to showing all seasons that match the current search text

### Requirement: Sortable season picker

The system SHALL provide season sorting modes for A-Z collation and intelligent chronological sorting.

#### Scenario: A-Z sort is selected

- **WHEN** the user selects A-Z sorting
- **THEN** the picker orders visible seasons by locale-aware collation of the visible season label

#### Scenario: Year-aware sort is selected

- **WHEN** the user selects year-aware sorting
- **THEN** the picker orders visible seasons using year or year ranges parsed from the visible season label

#### Scenario: Season label has no year

- **WHEN** a season label does not contain a parseable year
- **THEN** the picker keeps that season selectable and falls back to label collation for its relative order

### Requirement: Disclosable advanced controls

The system SHALL keep search, filter, and sort controls in a disclosable picker surface that is compact in the default header state and easy to discover when opened.

#### Scenario: Header remains compact

- **WHEN** the picker is closed
- **THEN** the header shows only a compact season trigger with the selected season and an affordance indicating more controls are available

#### Scenario: Advanced controls are discoverable

- **WHEN** the user opens the season picker
- **THEN** the picker shows a visible control or row for expanding sorting and filtering options without requiring hidden gestures

### Requirement: Season selection compatibility

The system SHALL preserve existing season selection behavior for table, matches, teams, and detail views.

#### Scenario: User selects a filtered season

- **WHEN** the user selects a season from a searched, filtered, or sorted list
- **THEN** the selected season becomes the app season and existing views refresh through the current season context

#### Scenario: Current selection is excluded by filter

- **WHEN** the active filter excludes the currently selected season
- **THEN** the app keeps the current selected season until the user explicitly selects another season
