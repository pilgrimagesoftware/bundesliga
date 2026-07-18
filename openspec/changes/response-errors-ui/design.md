## Context

The app fetches data through async functions in `crates/fulltime-ui/src/data/` (see `bundesliga-sports-ui`). Each view currently owns its own error state and renders the message inline, which means users can see the immediate failure but cannot inspect a history of response errors, compare repeated failures, or revisit an error after navigating away.

This change introduces a shared, session-scoped error capture and viewing surface. The first implementation should avoid disk persistence and external dependencies; response errors are session-scoped diagnostics for the current app run.

## Goals / Non-Goals

**Goals:**

- Capture response errors from shared data-loading paths.
- Expose recent errors from a compact, discoverable app-shell entry point.
- Let users inspect concise and detailed error information.
- Allow users to clear errors.
- Allow retry where a caller provides a retry handler.
- Avoid displaying sensitive request or response values.

**Non-Goals:**

- Persist error history across launches.
- Replace all inline error states; local context-specific errors can remain.
- Add remote telemetry, crash reporting, or analytics (a future Sentry integration, if enabled via `fulltime-core`'s `sentry` feature, is separate from this UI).
- Add a disk-backed error log.
- Capture non-response UI validation errors.

## Decisions

### 1. Use a session-scoped GPUI global for the error store

**Decision**: Add `crates/fulltime-ui/src/data/response_errors.rs` with a `ResponseErrorStore` GPUI global (`impl gpui::Global`) owning a bounded list of structured response error records.

```rust
struct ResponseErrorRecord {
    id: u64,
    occurred_at: Instant,
    source: ErrorSource, // Data, Http, Unknown
    operation: SharedString,
    view: Option<SharedString>,
    message: SharedString,
    detail: Option<SharedString>,
    retry_key: Option<SharedString>,
}
```

**Rationale**: The feature is UI-facing and session-scoped. A GPUI global is the same mechanism `FullTimeTheme` already uses (`crates/fulltime-ui/src/data/theme.rs`), and it is directly readable from any view via `cx.global::<ResponseErrorStore>()`.

**Alternative considered**: Store errors in a data-layer `DataCache` struct. Rejected — the primary consumers are UI components (toolbar indicator, panel), and no persistence is required, so the error store belongs with other UI-facing globals, not the fetch/cache layer.

### 2. Wrap response-producing operations at call sites first

**Decision**: Update existing view fetch code (the `cx.spawn` closures that call into `crates/fulltime-ui/src/data/`) to record caught errors through the shared store. A future helper can centralize this at the data-layer boundary, but the initial work should stay explicit at each user-visible load path.

**Rationale**: Each fetch is already wrapped in a `Result`-returning async call with a clear error boundary in the view's spawn closure. Explicit recording reduces risk and avoids a broad abstraction before patterns settle.

**Alternative considered**: Wrap every `crates/fulltime-ui/src/data/` function centrally (e.g., a `with_error_capture` combinator). Rejected for the initial version because it would obscure operation names, retry ownership, and view context that are easiest to supply at the call site.

### 3. Add a compact toolbar indicator and drawer/panel

**Decision**: Add a small response-error indicator in the toolbar (`crates/fulltime-ui/src/ui/views/toolbar.rs`). When there are no errors, it is either hidden or subdued. When errors exist, it shows a count and opens a right-side drawer or anchored panel (`response_errors_panel.rs`) with the error list and detail view.

**Rationale**: Response errors are important but not the primary workflow. A compact toolbar entry keeps them discoverable without pushing table/matches/team content around.

**Alternative considered**: Dedicated sidebar navigation item. Rejected because errors are cross-cutting diagnostics, not a primary app destination.

### 4. Detail view uses sanitized payload text

**Decision**: Store and display stringified details only after sanitization. The sanitizer removes or masks obvious sensitive keys such as `token`, `authorization`, `password`, `secret`, `api_key`, and `cookie`.

**Rationale**: Error payloads can be useful for debugging, but the UI should not casually expose credentials or secret-bearing headers.

**Alternative considered**: Show only the concise message. Rejected because the user explicitly wants response errors to be viewed, and detail payloads are often needed to understand failures.

### 5. Retry is optional and operation-owned

**Decision**: The error record may include a `retry_key`, but retry closures are registered by the currently mounted view (held in a `HashMap<SharedString, Box<dyn Fn(&mut App)>>` on the store, keyed by `retry_key`) rather than stored inside the record itself. If no retry handler is registered, the UI shows the error as inspectable but not retryable.

**Rationale**: Retrying a response requires operation-specific parameters and current state. Keeping retry ownership with the caller avoids stale or unsafe retries, and keeps `ResponseErrorRecord` plain data (cheap to clone for rendering).

**Alternative considered**: Store closures directly in error records. Rejected — it would make records non-`Clone`, complicate the bounded-retention eviction logic, and risk holding onto stale captured state.

## Risks / Trade-offs

- **Error list becomes noisy** -> Cap the retained list, for example 50 records, and deduplicate identical operation/message pairs within a short window.
- **Sensitive data leaks into detail text** -> Sanitize common sensitive keys and keep raw details best-effort, not exhaustive.
- **Retry action uses stale context** -> Only show retry when a current handler is registered; otherwise hide or disable retry.
- **Users miss hidden diagnostics** -> Show an obvious count badge when errors exist and keep the panel one click away from the toolbar.

## Migration Plan

1. Add the response error type, store, record/clear APIs, and sanitizer.
2. Add a `response_errors_panel.rs` view and a compact toolbar indicator.
3. Wire the panel into the toolbar.
4. Record errors from existing fetch paths.
5. Add retry handlers where existing views already expose refresh/load functions.
6. Verify error capture with simulated failing operations and normal successful flows.
