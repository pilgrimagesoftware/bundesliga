## 1. Error Model and Store

- [ ] 1.1 Create a `ResponseErrorRecord` type for operation, source, timestamp, message, detail, view, and optional retry key.
- [ ] 1.2 Create `src/lib/stores/responseErrors.svelte.ts` with record, list, select, clear one, and clear all helpers.
- [ ] 1.3 Add a bounded retention policy so the store keeps only the most recent response errors.
- [ ] 1.4 Add best-effort deduplication for repeated operation/message pairs within a short window.
- [ ] 1.5 Add a sanitizer that masks obvious sensitive keys in detail payloads.

## 2. Error Viewer UI

- [ ] 2.1 Create a compact app-shell/header response error indicator with an active count.
- [ ] 2.2 Create `ResponseErrorsPanel.svelte` as a drawer or anchored panel that opens without changing the current app view.
- [ ] 2.3 Render a recent error list with timestamp, source, operation, and concise message.
- [ ] 2.4 Render selected error details with sanitized detail text or an unavailable-detail fallback.
- [ ] 2.5 Add clear-one and clear-all actions and update the indicator count reactively.
- [ ] 2.6 Add retry action rendering only when a current retry handler is available.

## 3. Capture Integration

- [ ] 3.1 Record errors from `TableView.svelte` table loading failures.
- [ ] 3.2 Record errors from `MatchesView.svelte` matchday and match loading failures.
- [ ] 3.3 Record errors from `MatchDetailView.svelte` detail loading failures.
- [ ] 3.4 Record errors from `TeamsView.svelte` team list loading failures.
- [ ] 3.5 Record errors from `TeamDetailView.svelte` table, detail, and team-match loading failures.
- [ ] 3.6 Preserve existing inline error messages while also recording shared response errors.

## 4. Retry Integration

- [ ] 4.1 Register retry handlers for views that already expose refresh or load functions.
- [ ] 4.2 Ensure retry handlers are removed or ignored when their owning view unmounts.
- [ ] 4.3 Keep errors inspectable when no retry handler is currently available.

## 5. Verification

- [ ] 5.1 Add focused tests for sanitizer behavior and store record/clear/deduplication behavior.
- [ ] 5.2 Run `pnpm check`.
- [ ] 5.3 Run `pnpm build`.
- [ ] 5.4 Start the dev server and visually verify the empty indicator state, active count, panel list, detail view, clear actions, and retry availability.
