## ADDED Requirements

### Requirement: Response error capture

The system SHALL capture response errors from user-visible data loading operations in a shared session-scoped error list.

#### Scenario: Data-layer fetch fails

- **WHEN** a user-visible data-layer fetch fails
- **THEN** the system records an error with operation name, source, timestamp, concise message, and available sanitized details

#### Scenario: View keeps inline error

- **WHEN** a view records a response error in the shared list
- **THEN** the view may still display its local inline error state for immediate context

### Requirement: Error indicator entry point

The system SHALL provide a compact and discoverable app-shell entry point for viewing recorded response errors.

#### Scenario: No recorded errors

- **WHEN** no response errors are recorded
- **THEN** the app shell does not distract the user with an active error warning

#### Scenario: One or more errors recorded

- **WHEN** one or more response errors are recorded
- **THEN** the app shell shows an error indicator with the current count

#### Scenario: User opens error viewer

- **WHEN** the user activates the error indicator
- **THEN** the system opens the response error viewer without navigating away from the current primary view

### Requirement: Error list and detail viewing

The system SHALL let users inspect recorded response errors as both a list and a detail view.

#### Scenario: Error list is shown

- **WHEN** the error viewer opens
- **THEN** it displays recent response errors with timestamp, operation, source, and concise message

#### Scenario: Error detail is selected

- **WHEN** the user selects an error from the list
- **THEN** the viewer displays detailed sanitized error information when available

#### Scenario: Error detail is unavailable

- **WHEN** an error has no detail payload
- **THEN** the viewer indicates that no additional response details are available

### Requirement: Error clearing

The system SHALL allow users to clear recorded response errors.

#### Scenario: User clears one error

- **WHEN** the user clears a single error
- **THEN** that error is removed from the shared error list and the indicator count updates

#### Scenario: User clears all errors

- **WHEN** the user clears all response errors
- **THEN** the shared error list becomes empty and the active error indicator is removed or subdued

### Requirement: Retry support

The system SHALL support retry actions for recorded errors when the originating operation provides a current retry handler.

#### Scenario: Retry is available

- **WHEN** the selected error has an available retry handler
- **THEN** the viewer displays a retry action that invokes the current handler

#### Scenario: Retry is unavailable

- **WHEN** the selected error does not have an available retry handler
- **THEN** the viewer keeps the error inspectable and does not offer an active retry action

### Requirement: Sensitive detail protection

The system SHALL avoid exposing obvious sensitive values in displayed error details.

#### Scenario: Sensitive keys appear in detail payload

- **WHEN** an error detail contains sensitive keys such as authorization, token, password, secret, api_key, or cookie
- **THEN** the displayed detail masks those values

#### Scenario: Non-sensitive detail appears

- **WHEN** an error detail contains non-sensitive diagnostic fields
- **THEN** the displayed detail preserves those fields for inspection
