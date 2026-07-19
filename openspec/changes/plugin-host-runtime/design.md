## Context

`fulltime-core` is the native GPUI application entry point (see `bundesliga-sports-ui`); it
has no IPC boundary and no existing plugin/extension mechanism. Per the umbrella design
(`FullTime#1`), the plugin host uses `wasmtime` with the Component Model, and plugins get a
host-provided HTTP `fetch` capability rather than raw network access. `fulltime-plugin-api`
(its own child change, `define-league-data-contract`) is the source of truth for the
manifest format, canonical schema, and WIT interface this runtime loads plugins against.

## Goals / Non-Goals

**Goals:**
- Load, sandbox, and run WASM plugins conforming to `fulltime-plugin-api` inside
  `fulltime-core` without linking plugin code into the host binary.
- Isolate plugin faults so one misbehaving plugin cannot crash the host or other plugins.
- Give `fulltime-ui` a stable in-process API to request league data without knowing which
  plugin is serving it.
- Track installed/enabled plugin state persistently, and support unload/reload without an
  app restart.

**Non-Goals:**
- Implementing any specific plugin (Bundesliga, EPL, national teams) — those are separate
  child changes.
- A plugin marketplace or remote plugin discovery/download — this change only detects
  plugins already present in bundled or user plugin directories.
- Defining the manifest format, schema, or WIT interface — consumed from
  `fulltime-plugin-api`, not redefined here.

## Decisions

**Plugin host lives in a new module inside `fulltime-core`, not a separate crate, for v1.**
Rationale: the host runtime is tightly coupled to `fulltime-core`'s process lifecycle
(startup discovery, shutdown, app-data directory conventions already established by
`fulltime-core::logging::platform_log_dir`). Splitting it into its own crate before a second
consumer exists would be premature; revisit if the host runtime needs to be reused outside
`fulltime-core`.

**HTTP fetch capability is implemented as a minimal custom host function initially, not
WASI HTTP**, with a note to switch to WASI HTTP once tooling stabilizes.
Rationale: per the umbrella design's stated risk, WASI HTTP tooling in the Rust Component
Model ecosystem is still stabilizing. A minimal custom `fetch(request) -> response` host
function scoped to the manifest's declared hosts satisfies the sandboxing goal now without
blocking on WASI HTTP maturity; this is an explicit, documented deviation from the
umbrella's preferred approach, not a silent one.

**Plugin state (enabled/disabled, installed version) persists as a JSON file under the
platform app-data directory, not embedded in the manifest.**
Rationale: the manifest is a static, plugin-authored file; runtime state is host-owned and
mutated by user action. Keeping them separate avoids the host ever writing to a directory a
plugin might overwrite on update.

**Plugin unload/reload without restart uses `wasmtime`'s instance-per-call model** (a fresh
component instance per invocation, not a long-lived instance) rather than holding one
long-lived instance per plugin.
Rationale: simplifies reload (drop and reinstantiate) and fault isolation (a trapped
instance doesn't require host-side cleanup of half-mutated state), at the cost of
per-call instantiation overhead — acceptable since calls are already network-bound (see the
umbrella design's overhead risk note).

## Risks / Trade-offs

- [Per-call instantiation overhead vs. long-lived instances] → Network-bound calls dominate,
  per the umbrella design; benchmark in verification tasks before treating this as settled.
- [Custom `fetch` host function diverges from WASI HTTP, so plugins can't reuse
  WASI-HTTP-based tooling] → Documented deviation; migrate when WASI HTTP tooling is judged
  stable, tracked as an open question below.
- [Fault isolation via instance-per-call still needs an explicit trap-catching boundary
  around `wasmtime` calls] → Implement and test this explicitly (task group 3 and the
  verification tasks) rather than assuming `wasmtime` traps propagate safely by default.

## Migration Plan

1. Add `wasmtime` behind a feature flag; no plugin is loaded yet, existing app behavior
   unchanged.
2. Implement loading/instantiation, HTTP capability, and fault isolation against a minimal
   test/fixture plugin (not the real Bundesliga plugin) to validate the runtime in
   isolation.
3. Implement the manifest registry (discovery, enable/disable, version tracking) and plugin
   management UI.
4. Once `Plugins/Bundesliga` has a working plugin, wire it in as the first real consumer
   (coordinated with that change, not part of this one's tasks).

Rollback: keep the feature flag until a real plugin is validated end-to-end; the runtime is
inert (no plugins loaded) with the flag off.

## Open Questions

- When should the custom `fetch` host function be replaced with WASI HTTP? Revisit once
  `wasmtime`'s WASI HTTP support is judged stable enough for this project's Rust/toolchain
  version.
- Does the plugin management UI belong in a dedicated `fulltime-ui` screen, or as a
  settings-panel section? Deferred to implementation; either satisfies the proposal's "basic
  plugin management UI" requirement.
