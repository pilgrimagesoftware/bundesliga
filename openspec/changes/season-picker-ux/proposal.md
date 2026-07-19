## Why

The current season picker is a basic select control, which becomes tedious as soon as the available season list grows beyond a few recent years. Users need a fast way to find, narrow, and order seasons without cluttering the toolbar.

## What Changes

- Replace the simple season select UX with a discoverable, disclosable picker surface.
- Add season search so users can quickly find a season by year or name fragment.
- Add season filtering so users can reduce the visible list to only desired season ranges or matching season types.
- Add configurable sorting with at least:
  - A-Z collation by display name.
  - Intelligent chronological sorting based on year or year ranges found in the season name.
- Preserve a compact default toolbar control while making advanced controls easy to discover.
- Keep the selected season behavior compatible with existing table/matches/teams flows.

## Capabilities

### New Capabilities

- `season-picker-ux`: Searchable, filterable, and sortable season selection with a disclosable advanced UI.

### Modified Capabilities

- None.

## Impact

- `crates/fulltime-ui/src/ui/views/toolbar.rs`: Replace the placeholder season control with a dedicated season-picker component.
- `crates/fulltime-ui/src/ui/views/season_picker.rs` (new): Picker trigger, popover, search, filter, and sort UI, built on `gpui-component`'s popover/list primitives.
- `crates/fulltime-ui/src/data/seasons.rs`: `available_seasons()` (see `bundesliga-sports-ui`) stays the data source; no data-layer changes are required unless season metadata grows beyond plain year values.
- Visual behavior in the toolbar and any future season selection surfaces.

## Follow-up: superseded by `ui-skeleton`

The `ui-skeleton` change (implemented) removed `crates/fulltime-ui/src/ui/views/sidebar.rs` and
`toolbar.rs`, replacing them with a single persistent header (`header.rs`) and per-screen content
views. This proposal's references to `sidebar.rs`/`toolbar.rs` above are stale and need revision
against the new header-based shell (`AppScreen` enum, `header.rs`, `views/components/`) before
implementation starts.
