## 1. Season Option Model

- [ ] 1.1 Create a `SeasonOption` type or local equivalent with `value`, `label`, and parsed year metadata.
- [ ] 1.2 Add helper logic to format current numeric seasons as `YYYY/YYYY + 1` labels.
- [ ] 1.3 Add a year parser that extracts single years and year ranges from season labels.
- [ ] 1.4 Add unit-testable helpers for search matching, filter matching, and sorting.

## 2. Sorting and Filtering Behavior

- [ ] 2.1 Implement locale-aware A-Z sorting with `Intl.Collator`.
- [ ] 2.2 Implement year-aware ascending and descending sorting using parsed year metadata.
- [ ] 2.3 Implement baseline filter presets, including all seasons, current decade, and last 5 seasons.
- [ ] 2.4 Implement custom year range filtering with inclusive range intersection.
- [ ] 2.5 Ensure search and filters are applied before sorting.

## 3. Picker Component

- [ ] 3.1 Create `src/lib/components/SeasonPicker.svelte`.
- [ ] 3.2 Implement a compact header trigger that displays the selected season and a discoverable affordance.
- [ ] 3.3 Implement the picker popover with search input and selectable season list.
- [ ] 3.4 Add a visible disclosable options row for sorting and filtering controls.
- [ ] 3.5 Add an empty state when search/filter combinations match no seasons.
- [ ] 3.6 Keep the current app season selected even when filters hide it from the visible list.

## 4. Header Integration

- [ ] 4.1 Replace the native season `select` in `Header.svelte` with `SeasonPicker`.
- [ ] 4.2 Preserve existing `setSeason` behavior so table, matches, teams, and detail views refresh normally.
- [ ] 4.3 Keep header layout stable at the configured minimum desktop window width.
- [ ] 4.4 Ensure the picker closes after selection and restores focus to the trigger.

## 5. Verification

- [ ] 5.1 Add focused tests for year parsing, A-Z sorting, year-aware sorting, search, and filters.
- [ ] 5.2 Run `pnpm check`.
- [ ] 5.3 Run `pnpm build`.
- [ ] 5.4 Start the dev server and visually verify the closed header, opened picker, expanded options, filtered empty state, and season selection flow.
