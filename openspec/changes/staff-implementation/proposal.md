# app-staff-implementation

## Why

`bundesliga-sports-ui`'s team-data-cache capability plans a `TheSportsDbStaff` type and a `TeamDetail::staff` field as part of the TheSportsDB enrichment work, carried over from the original (Tauri-era) design intent. In that original implementation the field was scaffolded but never populated — dead code that always serialized as `[]`. Rather than repeat that mistake in the new data layer, this change resolves the staff-data question *before* `bundesliga-sports-ui`'s TheSportsDB integration tasks are implemented.

Two options:

- **Option A (Preferred)**: Implement the TheSportsDB staff lookup using the `/lookup_all_players.php` or equivalent endpoint, separating player (squad) and staff (manager/coach) data.
- **Option B (Fallback)**: Omit `TheSportsDbStaff` and the `staff` field from `TeamDetail` entirely if the API does not provide useful staff data.

## What Changes

- Investigate whether TheSportsDB's free tier API provides coaching staff data (`/lookup_all_players.php` returns players only; a separate endpoint may exist for staff/manager).
- If staff data is available: implement `fetch_thesportsdb_staff(tsdb_team_id)` in `crates/fulltime-ui/src/data/` and include a populated `staff` field on the team detail model from the start.
- If staff data is not available or requires a paid tier: do not add a `staff` field to the team detail model at all — no stub, no dead code, no removal step needed later.

## Capabilities

**Modified Capabilities**

- `team-data-cache` (owned by `bundesliga-sports-ui`): resolves whether `TeamDetail` carries real staff data (Option A) or no staff field (Option B) before that capability's implementation tasks are executed.

## Impact

- `crates/fulltime-ui/src/data/` (team detail / TheSportsDB module from `bundesliga-sports-ui`): include or omit the staff fetch function and field based on the investigation outcome — this decision should land before, or as part of, `bundesliga-sports-ui` task 10 (TheSportsDB integration).
- `crates/fulltime-ui/src/ui/views/team_detail_view.rs`: include or omit the staff section UI accordingly.
