## ADDED Requirements

### Requirement: Activity Log
The system SHALL maintain an in-memory, session-only log of activity entries, each with a
label, a status (`InProgress`, `Complete`, or `Failed` with a message), and a timestamp,
capped at a fixed size with oldest entries evicted first.

#### Scenario: Recording a completed activity
- **WHEN** an operation (e.g. a plugin load) completes successfully
- **THEN** an activity entry with status `Complete` is appended to the log

#### Scenario: Recording a failed activity
- **WHEN** an operation fails
- **THEN** an activity entry with status `Failed` and the failure's message is appended to the
  log

#### Scenario: Log eviction at capacity
- **WHEN** the log is at its capacity and a new entry is recorded
- **THEN** the oldest entry is evicted so the log does not exceed capacity

### Requirement: Status Bar Activity Button
The status bar SHALL show an activity button reflecting the count of currently in-progress
entries, opening an anchored popover listing the activity log (most recent first) when clicked.

#### Scenario: No in-progress activity
- **WHEN** no activity entry has status `InProgress`
- **THEN** the activity button renders in its idle state

#### Scenario: Opening the activity panel
- **WHEN** a user clicks the activity button
- **THEN** a popover anchored to the button opens, listing activity log entries newest first

### Requirement: Status Bar Alerts Button
The status bar SHALL show an alerts button with an unread indicator, opening an anchored popover
listing only `Failed` activity entries (the alert history) when clicked.

#### Scenario: Unread alert indicator
- **WHEN** a `Failed` activity entry has been recorded since the alerts panel was last opened
- **THEN** the alerts button shows an unread indicator

#### Scenario: Opening the alerts panel clears the unread indicator
- **WHEN** a user opens the alerts panel
- **THEN** the unread indicator is cleared, even if no entries have been read individually

#### Scenario: Alerts panel shows only failures
- **WHEN** the activity log contains both `Complete` and `Failed` entries
- **THEN** the alerts panel lists only the `Failed` entries
