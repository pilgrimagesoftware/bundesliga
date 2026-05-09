## Why

The current season picker is a basic select control, which becomes tedious as soon as the available season list grows beyond a few recent years. Users need a fast way to find, narrow, and order seasons without cluttering the main header.

## What Changes

- Replace the simple season select UX with a discoverable, disclosable picker surface.
- Add season search so users can quickly find a season by year or name fragment.
- Add season filtering so users can reduce the visible list to only desired season ranges or matching season types.
- Add configurable sorting with at least:
  - A-Z collation by display name.
  - Intelligent chronological sorting based on year or year ranges found in the season name.
- Preserve a compact default header control while making advanced controls easy to discover.
- Keep the selected season behavior compatible with existing league/table/matches/teams flows.

## Capabilities

### New Capabilities

- `season-picker-ux`: Searchable, filterable, and sortable season selection with a disclosable advanced UI.

### Modified Capabilities

- None.

## Impact

- Frontend season picker UI in `src/lib/components/Header.svelte`, likely extracted into a dedicated component.
- Frontend state for season selection and optional picker preferences.
- Existing `get_seasons` command consumers; no backend API changes are required unless season metadata grows beyond plain year values.
- Visual behavior in the desktop app header and any future season selection surfaces.
