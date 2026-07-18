## 1. Season Option Model

- [ ] 1.1 Create `SeasonOption` struct (`value`, `label`, parsed `sort_years`) in `crates/fulltime-ui/src/ui/views/season_picker.rs`.
- [ ] 1.2 Add helper logic to format numeric seasons as `YYYY/YYYY + 1` labels.
- [ ] 1.3 Add a year parser that extracts single years and year ranges from season labels.
- [ ] 1.4 Add unit-testable helpers for search matching, filter matching, and sorting.

## 2. Sorting and Filtering Behavior

- [ ] 2.1 Implement case-insensitive A-Z ordering by label.
- [ ] 2.2 Implement year-aware ascending and descending sorting using parsed year metadata.
- [ ] 2.3 Implement baseline filter presets: all seasons, current decade, last 5 seasons.
- [ ] 2.4 Implement custom year range filtering with inclusive range intersection.
- [ ] 2.5 Ensure search and filters are applied before sorting.

## 3. Picker Component

- [ ] 3.1 Create `crates/fulltime-ui/src/ui/views/season_picker.rs` with a `SeasonPicker` entity holding open/search/sort/filter state.
- [ ] 3.2 Implement a compact toolbar trigger that displays the selected season and a discoverable affordance.
- [ ] 3.3 Implement the picker popover (via `gpui-component`'s popover primitive) with a search input and selectable season list.
- [ ] 3.4 Add a visible disclosable options row for sorting and filtering controls.
- [ ] 3.5 Add an empty state when search/filter combinations match no seasons.
- [ ] 3.6 Keep the current app season selected even when filters hide it from the visible list.

## 4. Toolbar Integration

- [ ] 4.1 Replace the season placeholder in `crates/fulltime-ui/src/ui/views/toolbar.rs` with `SeasonPicker`.
- [ ] 4.2 Preserve the existing season-selection callback so table, matches, teams, and detail views refresh normally through `NavState`.
- [ ] 4.3 Keep toolbar layout stable at the configured minimum desktop window width (960px).
- [ ] 4.4 Ensure the picker closes after selection and restores focus to the trigger.

## 5. Verification

- [ ] 5.1 Add focused unit tests for year parsing, A-Z sorting, year-aware sorting, search, and filters.
- [ ] 5.2 Run `cargo clippy --workspace`.
- [ ] 5.3 Run `cargo +nightly fmt --check`.
- [ ] 5.4 Run `cargo run -p fulltime-core` and visually verify the closed toolbar, opened picker, expanded options, filtered empty state, and season selection flow.
