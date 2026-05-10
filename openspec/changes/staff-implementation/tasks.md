# app-staff-implementation — Tasks

1. [ ] 17.1 Check the TheSportsDB free-tier API documentation for a staff/manager endpoint (look for `/lookupteam.php`, `/lookupeventmanager.php`, or similar)
2. [ ] 17.2 **If staff data is available (Option A)**:
   1. [ ] 17.2a Implement `async fn fetch_thesportsdb_staff(tsdb_team_id: &str) -> Vec<TheSportsDbStaff>` following the same pattern as `fetch_thesportsdb_players`
   2. [ ] 17.2b Call `fetch_thesportsdb_staff` inside `get_team_detail` and populate `TeamDetail::staff`
   3. [ ] 17.2c Update the frontend TypeScript `TeamDetail` type if the `staff` field shape changes
   4. [ ] 17.2d Implement the staff section UI in `TeamDetailView` (name + role list; hide if empty)
3. [ ] 17.3 **If staff data is unavailable (Option B)**:
   1. [ ] 17.3a Remove the `TheSportsDbStaff` struct from `lib.rs`
   2. [ ] 17.3b Remove the `staff: Vec<TheSportsDbStaff>` field from `TeamDetail`
   3. [ ] 17.3c Remove all `staff: vec![]` initialisers from `TeamDetail` construction sites
   4. [ ] 17.3d Remove the `staff` field from the `TeamCacheFile` serialised JSON (add a migration note if cached files exist)
   5. [ ] 17.3e Remove the `staff` field from the frontend TypeScript `TeamDetail` type
   6. [ ] 17.3f Remove any staff UI section from `TeamDetailView`
4. [ ] 17.4 Run `cargo build` to confirm no compilation errors after either option
5. [ ] 17.5 Delete any existing `team_cache/*.json` files in the app data dir to avoid stale cache entries after the schema change
