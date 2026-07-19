## Why

When data-layer fetches fail, the current plan only shows local inline error text per view. Users need a central place to inspect recent response errors, understand what failed, and retry or clear errors without losing context.

## What Changes

- Add a response error viewing UI that collects recent data-fetch failures across all views.
- Provide a discoverable entry point from the app shell, such as a toolbar indicator or status button.
- Show a list of response errors with source, affected operation, timestamp, and concise message.
- Provide a detail view for each error with the raw response/error payload when available.
- Support clearing individual errors and clearing all recorded errors.
- Support retrying failed operations when the originating operation exposes a retry handler.
- Avoid exposing sensitive values from request/response payloads in the UI.

## Capabilities

### New Capabilities

- `response-errors-ui`: Centralized viewing and management of recent response errors.

### Modified Capabilities

- None.

## Impact

- `crates/fulltime-ui/src/data/response_errors.rs` (new): `ResponseErrorRecord` type and a session-scoped GPUI global (or `Entity`) holding the recent-errors list, record/clear/select operations, and a sanitizer.
- Existing data-layer fetch call sites (`bundesliga-sports-ui`'s `fetch_table`, `fetch_matches_for_matchday`, etc.) should record errors through the shared mechanism on failure.
- `crates/fulltime-ui/src/ui/views/toolbar.rs` gains a compact error indicator.
- `crates/fulltime-ui/src/ui/views/response_errors_panel.rs` (new): a disclosable panel/drawer listing recorded errors with a detail pane.
- Optional retry hooks for table, matches, teams, team detail, and match detail data loads.
- No IPC boundary to cross — errors are recorded directly from the async fetch functions with enough context (operation name, view, sanitized payload) for users to identify the failing operation.

## Follow-up: superseded by `ui-skeleton`

The `ui-skeleton` change (implemented) removed `crates/fulltime-ui/src/ui/views/sidebar.rs` and
`toolbar.rs`, replacing them with a single persistent header (`header.rs`) and per-screen content
views. This proposal's references to `sidebar.rs`/`toolbar.rs` above are stale and need revision
against the new header-based shell (`AppScreen` enum, `header.rs`, `views/components/`) before
implementation starts.
