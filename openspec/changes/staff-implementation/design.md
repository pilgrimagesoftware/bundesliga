# Design: staff-implementation

## Context

The `bundesliga-sports-ui` change spec (the original app rebuild) included `TheSportsDbStaff` as part of the team enrichment plan. The implementation was deferred or partially forgotten: the struct exists, the field exists, but no fetching logic was written. The result is a struct that appears to be a data model but never carries data.

## Goals / Non-Goals

**Goals:**

- Resolve the gap between the declared data model and what is actually fetched
- Either fully implement staff lookup or cleanly remove the dead abstraction

**Non-Goals:**

- Building a full coaching staff management feature
- Displaying staff data beyond name and role

## Decisions

**Investigate first, then implement or remove**: The decision between Option A and Option B depends on what TheSportsDB's free tier API actually provides. The TheSportsDB does have a `/lookup_all_players.php` endpoint; it may also have a `/lookupeventmanager.php` or similar. Confirm availability before writing code.

**Prefer removal if data quality is poor**: If staff data requires paid access or returns incomplete/unreliable results, Option B (removal) is preferable to having a stub field that always serialises as `[]`.

## Risks / Trade-offs

- Option A adds another external API call per team detail request (within the existing cooldown/cache TTL window, so rate impact is minimal)
- Option B is a breaking change to the `TeamDetail` struct and the corresponding TypeScript type

## Open Questions

- Does TheSportsDB free tier (`/api/v1/json/3/`) provide coaching staff data? If yes, which endpoint?
- Is the frontend currently rendering a staff section (even as empty)? If so, Option B requires a frontend UI change.
