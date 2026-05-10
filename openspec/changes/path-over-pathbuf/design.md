# Design: path-over-pathbuf 

## Context

`std::path::PathBuf` is the owned, heap-allocated path type. `std::path::Path` is the borrowed, unsized path type — analogous to `String` vs `str`. The Rust API guidelines (C-CALLER-CONTROL) recommend accepting `&Path` when a function only needs to read or traverse the path, and accepting `PathBuf` (by value or reference) only when the function needs to own or extend the path. Both `read_team_cache` and `write_team_cache` only use the path for construction (via `.join()`), which is available on `Path` as well as `PathBuf`.

## Goals / Non-Goals

**Goals:**

- Follow Rust API guidelines for path parameter types
- Slightly broaden the accepted input types for these functions

**Non-Goals:**

- Changing the filesystem operations performed by these functions

## Decisions

**Mechanical change only**: Replace `&PathBuf` with `&Path` in both signatures. All callers already pass a `&PathBuf` (which coerces to `&Path`), so no call-site changes are needed.

## Risks / Trade-offs

- None. `&PathBuf` derefs to `&Path`; all existing call sites compile unchanged.

## Open Questions

- None.
