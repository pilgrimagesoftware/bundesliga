# Design: staff-implementation

## Context

`bundesliga-sports-ui`'s team-data-cache capability plans TheSportsDB enrichment (squad + staff) for the team detail view. The staff half of that plan carries an unresolved question inherited from the original Tauri-era attempt, where a `TheSportsDbStaff` struct and a `TeamDetail::staff` field were scaffolded but never populated — the field always serialized as `[]`. This change resolves that question up front, before the new data layer is written, so the mistake isn't repeated.

## Goals / Non-Goals

**Goals:**

- Resolve whether TheSportsDB's free tier provides usable coaching staff data.
- Ensure `bundesliga-sports-ui`'s team detail model either fully implements staff lookup or never declares an unpopulated `staff` field.

**Non-Goals:**

- Building a full coaching staff management feature.
- Displaying staff data beyond name and role.

## Decisions

**Investigate first, then implement or omit**: The decision between Option A and Option B depends on what TheSportsDB's free tier API actually provides. TheSportsDB does have a `/lookup_all_players.php` endpoint; it may also have a `/lookupeventmanager.php` or similar. Confirm availability before writing `bundesliga-sports-ui`'s TheSportsDB integration tasks.

**Prefer omission if data quality is poor**: If staff data requires paid access or returns incomplete/unreliable results, Option B (no `staff` field at all) is preferable to shipping a field that always serializes as empty.

## Risks / Trade-offs

- Option A adds another external API call per team detail request (within the existing cooldown/cache TTL window from `bundesliga-sports-ui`, so rate impact is minimal).
- Choosing Option A late (after `bundesliga-sports-ui`'s team-data-cache is already implemented) would require a schema change to `TeamDetail` and its disk cache format; resolving this first avoids that.

## Open Questions

- Does TheSportsDB free tier (`/api/v1/json/3/`) provide coaching staff data? If yes, which endpoint?
