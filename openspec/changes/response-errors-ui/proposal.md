## Why

When API or command responses fail, the current UI only shows local inline error text. Users need a central place to inspect recent response errors, understand what failed, and retry or clear errors without losing context.

## What Changes

- Add a response error viewing UI that collects recent frontend/backend response failures.
- Provide a discoverable entry point from the app shell, such as a header indicator or status button.
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

- Frontend error state/store for recording response errors.
- Existing view fetch paths and Tauri `invoke` call sites should record errors through a shared mechanism.
- App shell/header UI gains a compact error indicator and a disclosable error panel or drawer.
- Optional retry hooks for table, matches, teams, team detail, and match detail data loads.
- No backend API change is required for the initial UI, but backend command errors should be captured with enough context for users to identify the failing operation.
