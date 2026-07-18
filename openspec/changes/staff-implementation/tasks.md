# app-staff-implementation — Tasks

1. [ ] 1.1 Check the TheSportsDB free-tier API documentation for a staff/manager endpoint (look for `/lookupteam.php`, `/lookupeventmanager.php`, or similar).
2. [ ] 1.2 **If staff data is available (Option A)**, fold into `bundesliga-sports-ui` task 10 (TheSportsDB integration) rather than as a follow-up patch:
   1. [ ] 1.2a Implement `async fn fetch_thesportsdb_staff(tsdb_team_id: &str) -> Vec<TheSportsDbStaff>` following the same pattern as the players fetch, in `crates/fulltime-ui/src/data/`.
   2. [ ] 1.2b Call `fetch_thesportsdb_staff` inside the team detail fetch and populate the `staff` field on the team detail model from the start.
   3. [ ] 1.2c Implement the staff section UI in `crates/fulltime-ui/src/ui/views/team_detail_view.rs` (name + role list; hide if empty).
3. [ ] 1.3 **If staff data is unavailable (Option B)**, ensure `bundesliga-sports-ui` task 10 never introduces the field:
   1. [ ] 1.3a Do not add a `TheSportsDbStaff` type.
   2. [ ] 1.3b Do not add a `staff` field to the team detail model.
   3. [ ] 1.3c Do not add a staff section to `team_detail_view.rs`.
4. [ ] 1.4 Run `cargo build --workspace` to confirm no compilation errors after the decision is applied.
