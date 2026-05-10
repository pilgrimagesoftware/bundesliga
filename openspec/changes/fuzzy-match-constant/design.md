# Design: fuzzy-match-constant 

## Context

The Jaro-Winkler similarity score between `"fc bayern munchen"` and `"bayern munich"` is approximately 0.88. Between `"borussia dortmund"` and `"dortmund"` it is approximately 0.79. A threshold of `0.75` accepts both while rejecting clearly wrong matches (score < 0.60). The value was presumably determined empirically but is not documented anywhere.

## Goals / Non-Goals

**Goals:**

- Make the threshold visible and self-documenting
- Make future tuning a one-line change

**Non-Goals:**

- Re-tuning the threshold value (can be done as a follow-up if needed)
- Building a test suite for name matching (useful but separate scope)

## Decisions

**Named constant at module scope**: Placing the constant near the other TheSportsDB helper code makes the relationship clear. A leading doc comment `///` or inline `//` comment explains the rationale.

## Risks / Trade-offs

- None. The behaviour is identical; only code readability improves.

## Open Questions

- Should the threshold be made configurable via Tauri app settings in the future? (Probably not necessary; the value is stable once tuned.)
