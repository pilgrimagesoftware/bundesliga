## Context

The desktop app header currently exposes season selection as a native `select` bound to a short numeric season list. That works for the current backend, but it will not scale if seasons include historical data, named competitions, or ranges from multiple sources. The picker must remain compact because it lives in the app header beside league selection, live status, and refresh controls.

The change should be frontend-first. Existing screens already react to `setSeason`, so the picker should preserve that contract and avoid backend changes unless a future data source returns richer season metadata.

## Goals / Non-Goals

**Goals:**

- Make season selection searchable, filterable, and sortable.
- Keep the default header UI compact.
- Make advanced controls disclosable but obvious enough to discover.
- Support both simple numeric seasons and richer display names that contain one or more years.
- Keep keyboard and pointer interaction practical for desktop use.

**Non-Goals:**

- Redesign the entire header or app shell.
- Add a new backend endpoint for season metadata.
- Persist season picker preferences across app launches unless implementation can do so without changing backend persistence.
- Support mobile-specific picker behavior; the app has desktop window constraints.

## Decisions

### 1. Extract a dedicated `SeasonPicker` component

**Decision**: Replace the inline season `select` in `Header.svelte` with a dedicated `SeasonPicker.svelte` component.

**Rationale**: Search, filter, sort mode, popover state, and keyboard handling are enough complexity to justify a focused component. `Header.svelte` should keep ownership of layout and pass `seasons`, the current selected season, and an `onSelect` callback.

**Alternative considered**: Keep all logic in `Header.svelte`. Rejected because it would mix header state synchronization with picker-specific UI behavior.

### 2. Use a compact trigger plus popover

**Decision**: The header shows a compact season trigger that displays the selected season. Activating it opens a popover anchored to the trigger. The popover contains search, list, and a visible "Options" disclosure row for sorting/filtering controls.

**Rationale**: This keeps the header dense and stable while making the enhanced controls discoverable. Users see that additional controls exist without paying the full visual cost by default.

**Alternative considered**: Always show search and sort controls directly in the header. Rejected because the header would become crowded and less scannable.

### 3. Normalize season records in the frontend

**Decision**: Convert the current `number[]` seasons into frontend view models:

```ts
type SeasonOption = {
  value: number;
  label: string;
  sortYears: number[];
}
```

The initial label remains `YYYY/YYYY + 1`. The year parser should be implemented against labels rather than only numeric values so it continues to work if labels become richer later.

**Rationale**: Sorting and filtering need a consistent shape. Keeping this in the frontend avoids backend churn while giving room for future metadata.

**Alternative considered**: Change `get_seasons` to return objects. Rejected for this change because the current requirement is UX behavior and does not require API changes.

### 4. Sorting modes

**Decision**: Support at least two sort modes:

- `name-asc`: locale-aware A-Z collation by label using `Intl.Collator`.
- `year-desc` and `year-asc`: intelligent chronological sorting by parsed year or year range. The primary year is the first year-like token found in the label; ties fall back to the full parsed range, then label collation.

**Rationale**: A-Z is predictable for named seasons; year-aware sorting handles labels such as `2025/2026`, `2025-26`, `1999`, and `DFB Pokal 2024`.

**Alternative considered**: Only sort numeric values. Rejected because the request explicitly calls out intelligent sorting by year or years found in the name.

### 5. Search and filters are applied before sorting

**Decision**: Search and filters reduce the candidate list first; sorting then orders the remaining list.

**Rationale**: This is the most predictable data-table style behavior and keeps counts easy to explain: "showing N of M seasons".

**Filter baseline**:

- All seasons.
- Current decade.
- Last 5 seasons.
- Custom year range with `from` and `to` numeric inputs.

The implementation can adjust labels if the real season list makes different presets more useful, but it must keep at least one quick preset and one custom range option.

## Risks / Trade-offs

- **Popover complexity in a dense header** -> Keep dimensions fixed, use constrained max height, and test at the configured minimum window width.
- **Ambiguous year parsing** -> Document parser behavior and fall back to name collation when no year is found.
- **Too many controls hidden behind disclosure** -> The trigger should include a visible chevron/settings affordance, and the popover should show an "Options" row even when collapsed.
- **Current backend returns only four seasons** -> Build against the same component behavior with synthetic long lists in component tests or local fixtures so the massive-list case is covered.

## Migration Plan

1. Add the `SeasonPicker` component and helper functions for label generation, year parsing, sorting, searching, and filtering.
2. Replace the season `select` in `Header.svelte` with the component while preserving the `setSeason` behavior.
3. Add focused tests for sorting, search, and filters using a long synthetic season list.
4. Verify the header at normal and minimum desktop widths.
