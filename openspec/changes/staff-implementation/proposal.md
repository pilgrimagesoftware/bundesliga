# app-staff-implementation

## Why

The `TheSportsDbStaff` struct and `TeamDetail::staff` field exist in `app/src-tauri/src/lib.rs` but `staff` is always initialised as `vec![]` — no code ever populates it. This is dead code: the struct definition is misleading (it implies staff data is fetched and available), and the field occupies space in the serialised JSON payload sent to the frontend without carrying any information.

Two options:

- **Option A (Preferred)**: Implement the TheSportsDB staff lookup using the `/lookup_all_players.php` or equivalent endpoint, separating player (squad) and staff (manager/coach) data
- **Option B (Fallback)**: Remove `TheSportsDbStaff`, the `staff` field from `TeamDetail`, and any related dead code if the API does not provide useful staff data

## What Changes

- Investigate whether TheSportsDB's free tier API provides coaching staff data (e.g., `/lookup_all_players.php` returns players only; a separate endpoint may exist for staff/manager)
- If staff data is available: implement `fetch_thesportsdb_staff(tsdb_team_id)` and populate `TeamDetail::staff`
- If staff data is not available or requires a paid tier: remove `TheSportsDbStaff`, remove the `staff` field from `TeamDetail`, remove the `staff: vec![]` initialisers

## Capabilities

**Modified Capabilities**

- `team-data-cache` (if Option A): `TeamDetail` now contains real staff data
- `team-data-cache` (if Option B): `TeamDetail` schema simplified; frontend `TeamDetail` TypeScript type must be updated

## Impact

- `app/src-tauri/src/lib.rs`: add staff fetch function (Option A) or remove struct/field (Option B)
- `app/src` TypeScript types: add or remove `staff` field on `TeamDetail` type accordingly
