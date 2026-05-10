# Design: windows-subsystem-placement

## Context

The `windows_subsystem` inner attribute is a crate-level attribute that affects the Windows PE header of the compiled binary. It only has meaning in a binary (`bin`) crate target. The Tauri template generates it in `main.rs`, which is correct. It should never appear in `lib.rs`.

## Goals / Non-Goals

**Goals:**
- Remove the no-op attribute from `lib.rs` to eliminate confusion
- Ensure `main.rs` remains the sole canonical home for this attribute

**Non-Goals:**
- Changing the Windows subsystem behaviour

## Decisions

**Remove from lib.rs only**: The attribute in `main.rs` is correct and should be left untouched.

## Risks / Trade-offs

- None. The attribute in `lib.rs` was already ignored by the compiler for library targets.

## Open Questions

- None.
