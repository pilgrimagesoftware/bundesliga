## Context

The toolbar (`crates/fulltime-ui/src/ui/views/toolbar.rs`) will expose season selection alongside league selection, a refresh control, and a live-match badge (see `bundesliga-sports-ui`). A native `select`-equivalent works for a handful of recent years, but it will not scale if seasons include historical data, named competitions, or ranges from multiple sources. The picker must remain compact because it shares toolbar space with those other controls.

The change is UI-layer-only. `available_seasons()` already returns a plain `Vec<i32>` (see `bundesliga-sports-ui`'s data layer); this change wraps that in a picker view and preserves the existing "selecting a season updates `NavState`" contract without touching the data layer, unless a future data source returns richer season metadata.

## Goals / Non-Goals

**Goals:**

- Make season selection searchable, filterable, and sortable.
- Keep the default toolbar UI compact.
- Make advanced controls disclosable but obvious enough to discover.
- Support both simple numeric seasons and richer display names that contain one or more years.
- Keep keyboard and pointer interaction practical for desktop use.

**Non-Goals:**

- Redesign the entire toolbar or app shell.
- Add a new data-layer entry point for season metadata.
- Persist season picker preferences across app launches unless implementation can do so without changing the existing `AppViewState` persistence (see `bundesliga-sports-ui`).
- Support touch/mobile-specific picker behavior; the app has desktop window constraints.

## Decisions

### 1. Extract a dedicated season-picker view

**Decision**: Add `crates/fulltime-ui/src/ui/views/season_picker.rs` exposing a `render_season_picker(seasons: &[SeasonOption], selected: i32, colors: &ColorTokens, cx) -> impl IntoElement` function (or a small `Entity`-backed component if internal open/search/sort state needs to persist across renders — see Decision 2). `toolbar.rs` keeps ownership of layout and passes the season list, current selection, and a selection callback.

**Rationale**: Search, filter, sort mode, popover open/close state, and keyboard handling are enough complexity to justify a focused module, matching the file-per-view convention already established by `title_bar.rs`/`sidebar.rs`/`status_bar.rs`.

**Alternative considered**: Keep all logic inline in `toolbar.rs`. Rejected because it would mix toolbar layout with picker-specific interaction state.

### 2. Compact trigger plus popover, backed by a small `Entity`

**Decision**: The toolbar shows a compact season trigger button that displays the selected season. Activating it opens a `gpui-component` popover anchored to the trigger. The popover contains a search input, the season list, and a visible "Options" disclosure row for sorting/filtering controls. Because open/closed state, search text, and sort/filter mode must survive across renders, this is a `SeasonPicker` `Entity<SeasonPicker>` (a small `Render`-implementing struct), not a stateless render function.

**Rationale**: This keeps the toolbar dense and stable while making the enhanced controls discoverable. Users see that additional controls exist without paying the full visual cost by default.

**Alternative considered**: Always show search and sort controls directly in the toolbar. Rejected because the toolbar would become crowded and less scannable.

### 3. `SeasonOption` view model

**Decision**: Convert the current `Vec<i32>` seasons into a view model at the picker boundary:

```rust
struct SeasonOption {
    value: i32,
    label: SharedString,
    sort_years: Vec<i32>,
}
```

The initial label remains `YYYY/YYYY + 1`. The year parser should work against labels rather than only numeric values so it continues to work if labels become richer later.

**Rationale**: Sorting and filtering need a consistent shape. Keeping this conversion at the picker boundary avoids touching `available_seasons()` while giving room for future metadata.

**Alternative considered**: Change `available_seasons()` to return `SeasonOption`s directly. Rejected for this change because the requirement is UX behavior, not a data-layer change.

### 4. Sorting modes

**Decision**: Support at least two sort modes:

- `NameAsc`: case-insensitive A-Z ordering by label using `str::to_lowercase` comparison (no ICU/`Intl.Collator` equivalent is in the dependency tree, and season labels are ASCII digits/slashes, so a full locale-aware collator isn't warranted).
- `YearDesc`/`YearAsc`: intelligent chronological sorting by parsed year or year range. The primary year is the first year-like token found in the label; ties fall back to the full parsed range, then label ordering.

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

- **Popover complexity in a dense toolbar** -> Keep dimensions fixed, use a constrained max height, and test at the configured minimum window width (960px, see `bundesliga-sports-ui`).
- **Ambiguous year parsing** -> Document parser behavior and fall back to name ordering when no year is found.
- **Too many controls hidden behind disclosure** -> The trigger should include a visible chevron affordance, and the popover should show an "Options" row even when collapsed.
- **`available_seasons()` currently returns only four seasons** -> Build against the same component behavior with synthetic long lists in unit tests so the massive-list case is covered.

## Migration Plan

1. Add the `SeasonPicker` entity and helper functions for label generation, year parsing, sorting, searching, and filtering.
2. Replace the season placeholder in `toolbar.rs` with the picker while preserving the existing season-selection callback into `NavState`.
3. Add focused unit tests for sorting, search, and filters using a long synthetic season list.
4. Verify the toolbar at normal and minimum desktop window widths.
