## 1. Error Model and Store

- [ ] 1.1 Create `ResponseErrorRecord` in `crates/fulltime-ui/src/data/response_errors.rs`: operation, source, timestamp, message, detail, view, optional retry key.
- [ ] 1.2 Create `ResponseErrorStore` as a GPUI global with record, list, select, clear-one, and clear-all methods.
- [ ] 1.3 Add a bounded retention policy so the store keeps only the most recent response errors (e.g. 50).
- [ ] 1.4 Add best-effort deduplication for repeated operation/message pairs within a short window.
- [ ] 1.5 Add a sanitizer that masks obvious sensitive keys in detail payloads.
- [ ] 1.6 Add a `HashMap<SharedString, Box<dyn Fn(&mut App)>>` on the store for registered retry handlers, keyed by `retry_key`.

## 2. Error Viewer UI

- [ ] 2.1 Add a compact response-error indicator to `crates/fulltime-ui/src/ui/views/toolbar.rs` showing the active count.
- [ ] 2.2 Create `crates/fulltime-ui/src/ui/views/response_errors_panel.rs` as a drawer or anchored panel that opens without changing the current `NavScreen`.
- [ ] 2.3 Render a recent error list with timestamp, source, operation, and concise message.
- [ ] 2.4 Render selected error details with sanitized detail text or an unavailable-detail fallback.
- [ ] 2.5 Add clear-one and clear-all actions; update the indicator count reactively.
- [ ] 2.6 Add a retry action rendered only when a current retry handler is registered for the error's `retry_key`.

## 3. Capture Integration

- [ ] 3.1 Record errors from the table view's fetch failures.
- [ ] 3.2 Record errors from the matches view's matchday and match loading failures.
- [ ] 3.3 Record errors from the match detail view's loading failures.
- [ ] 3.4 Record errors from the teams view's team list loading failures.
- [ ] 3.5 Record errors from the team detail view's table, detail, and team-match loading failures.
- [ ] 3.6 Preserve existing inline error messages while also recording shared response errors.

## 4. Retry Integration

- [ ] 4.1 Register retry handlers for views that already expose refresh or load functions.
- [ ] 4.2 Deregister retry handlers when their owning view/entity is dropped.
- [ ] 4.3 Keep errors inspectable when no retry handler is currently registered.

## 5. Verification

- [ ] 5.1 Add unit tests for sanitizer behavior and store record/clear/deduplication behavior.
- [ ] 5.2 Run `cargo clippy --workspace`.
- [ ] 5.3 Run `cargo +nightly fmt --check`.
- [ ] 5.4 Run `cargo run -p fulltime-core` and visually verify the empty indicator state, active count, panel list, detail view, clear actions, and retry availability.
