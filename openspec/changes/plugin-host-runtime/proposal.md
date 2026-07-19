## Why

`FullTime.rs` currently has no plugin loading mechanism; league data will come exclusively
from a `fulltime-plugin-api`-conformant WASM plugin going forward (per the umbrella change,
`FullTime#1`, `openspec/changes/league-data-plugin-system`). The app needs a host runtime
that loads, sandboxes, and manages plugins, plus a registry that tracks which plugins are
installed and enabled, before any plugin (including the Bundesliga reference plugin) has
somewhere to run.

## What Changes

- Add `wasmtime` (Component Model) to `fulltime-core` behind a feature flag.
- Implement plugin loading/instantiation from a manifest conforming to
  `fulltime-plugin-api`'s manifest format, rejecting plugins with an incompatible schema or
  interface version.
- Implement the host-provided HTTP fetch capability, scoped per-plugin to the hostnames
  declared in its manifest — plugins get no direct network access.
- Implement fault isolation so a plugin panic/trap cannot crash the host or affect other
  loaded plugins.
- Implement plugin unloading and re-loading without an app restart.
- Implement discovery of bundled first-party plugins and user-installed plugins from
  designated plugin directories at startup.
- Implement per-plugin enable/disable state, persisted across restarts, and installed-version
  tracking with update-availability detection for plugins declaring an update source.
- Add a basic plugin management UI (list installed plugins, enable/disable, show update
  availability) to `fulltime-ui`.
- Switch the app's UI/business logic to consume `fulltime-plugin-api`'s canonical schema
  types instead of any provider-specific type.

## Capabilities

### New Capabilities

- `plugin-host-runtime`: WASM plugin loading, sandboxing, resource limits, and lifecycle
  management inside `fulltime-core`.
- `plugin-manifest-registry`: discovery of bundled/installed plugins, enable/disable state,
  and update-availability tracking.
- `plugin-management-ui`: minimal `fulltime-ui` surface for listing, enabling/disabling, and
  checking updates for installed plugins.

### Modified Capabilities

- (none — no existing specs predate this change in this repo)

## Impact

- **`fulltime-core`**: new plugin host module; new dependency on `wasmtime` (feature-flagged)
  and `fulltime-plugin-api`.
- **`fulltime-ui`**: new plugin management screen; any existing/planned data-fetch code
  (see `bundesliga-sports-ui`) is repointed to consume plugin output via the canonical
  schema instead of a direct SDK dependency.
- Depends on `fulltime-plugin-api`'s `define-league-data-contract` change for the manifest
  format, schema types, and WIT interface this runtime loads against.
- Out of scope: the Bundesliga plugin implementation itself (`Plugins/Bundesliga`, separate
  child change) and the plugin SDK/template for building further plugins (umbrella task
  group 7, proposed separately once this runtime and the reference plugin both exist).
