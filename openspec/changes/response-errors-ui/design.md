## Context

The app currently performs data loading through Tauri `invoke` calls inside individual views. Each view owns its own `error` string and renders the message inline, which means users can see the immediate failure but cannot inspect a history of response errors, compare repeated failures, or revisit an error after navigating away.

This change introduces a shared frontend error capture and viewing surface. The first implementation should avoid backend persistence and external dependencies; response errors are session-scoped diagnostics for the current app run.

## Goals / Non-Goals

**Goals:**

- Capture response errors from shared frontend data-loading paths.
- Expose recent errors from a compact, discoverable app-shell entry point.
- Let users inspect concise and detailed error information.
- Allow users to clear errors.
- Allow retry where a caller provides a retry handler.
- Avoid displaying sensitive request or response values.

**Non-Goals:**

- Persist error history across launches.
- Replace all inline error states; local context-specific errors can remain.
- Add remote telemetry, crash reporting, or analytics.
- Add a backend database or file-backed error log.
- Capture non-response UI validation errors.

## Decisions

### 1. Use a session-scoped frontend error store

**Decision**: Add a Svelte store module, for example `src/lib/stores/responseErrors.svelte.ts`, that owns an array of structured response error records.

```ts
type ResponseErrorRecord = {
  id: string;
  occurredAt: number;
  source: "tauri" | "http" | "unknown";
  operation: string;
  view?: string;
  message: string;
  detail?: string;
  retryKey?: string;
}
```

**Rationale**: The feature is UI-facing and session-scoped. Keeping it in frontend state avoids backend complexity and lets all views record failures consistently.

**Alternative considered**: Store errors in Rust `AppState`. Rejected for the initial feature because the primary consumers are frontend components and no persistence is required.

### 2. Wrap response-producing operations at call sites first

**Decision**: Update existing view fetch functions to record caught errors through the shared store. A future helper can centralize `invoke` wrapping, but the initial work should stay explicit at each user-visible load path.

**Rationale**: The current code already has clear `try/catch` boundaries in table, matches, teams, team detail, and match detail views. Explicit recording reduces risk and avoids a broad abstraction before patterns settle.

**Alternative considered**: Monkey-patch or globally wrap `invoke`. Rejected because it would obscure operation names, retry ownership, and view context.

### 3. Add a compact header indicator and drawer/panel

**Decision**: Add a small response-error indicator in the app header. When there are no errors, it is either hidden or subdued. When errors exist, it shows a count and opens a right-side drawer or anchored panel with the error list and detail view.

**Rationale**: Response errors are important but not the primary workflow. A compact header entry keeps them discoverable without pushing table/matches/team content around.

**Alternative considered**: Dedicated sidebar navigation item. Rejected because errors are cross-cutting diagnostics, not a primary app destination.

### 4. Detail view uses sanitized payload text

**Decision**: Store and display stringified details only after sanitization. The sanitizer should remove or mask obvious sensitive keys such as `token`, `authorization`, `password`, `secret`, `api_key`, and `cookie`.

**Rationale**: Error payloads can be useful for debugging, but the UI should not casually expose credentials or secret-bearing headers.

**Alternative considered**: Show only the concise message. Rejected because the user explicitly wants response errors to be viewed, and detail payloads are often needed to understand failures.

### 5. Retry is optional and operation-owned

**Decision**: The error record may include a `retryKey`, but retry handlers are registered by the current mounted view or shell integration. If no retry handler is available, the UI shows the error as inspectable but not retryable.

**Rationale**: Retrying a response requires operation-specific parameters and current state. Keeping retry ownership with the caller avoids stale or unsafe retries.

**Alternative considered**: Store closures directly in error records. Rejected because closures are not serializable, can capture stale state, and complicate cleanup.

## Risks / Trade-offs

- **Error list becomes noisy** -> Cap the retained list, for example 50 records, and deduplicate identical operation/message pairs within a short window.
- **Sensitive data leaks into detail text** -> Sanitize common sensitive keys and keep raw details best-effort, not exhaustive.
- **Retry action uses stale context** -> Only show retry when a current handler is registered; otherwise hide or disable retry.
- **Users miss hidden diagnostics** -> Show an obvious count badge when errors exist and keep the panel one click away from the header.

## Migration Plan

1. Add the response error type, store, record/clear APIs, and sanitizer.
2. Add a `ResponseErrorsPanel` component and a compact header indicator.
3. Wire the panel into the app shell/header.
4. Record errors from existing fetch paths.
5. Add retry handlers where existing views already expose refresh/load functions.
6. Verify error capture with simulated failing operations and normal successful flows.
