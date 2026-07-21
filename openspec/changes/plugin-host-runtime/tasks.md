## 1. Runtime Setup

- [x] 1.1 Add `wasmtime` (Component Model) to `fulltime-core` behind a feature flag
- [x] 1.2 Add a dependency on `fulltime-plugin-api` for the manifest format, canonical
  schema types, and WIT interface

## 2. Plugin Host Runtime

- [x] 2.1 Implement plugin loading/instantiation from the manifest, rejecting incompatible
  schema/interface versions
- [x] 2.2 Implement the host-provided HTTP fetch capability, scoped to hosts declared in
  the plugin's manifest
- [x] 2.3 Implement fault isolation so a plugin panic/trap does not crash the host or
  affect other plugins
- [x] 2.4 Implement plugin unloading and re-loading without an app restart
- [x] 2.5 Validate the runtime against a minimal fixture/test plugin (not the real
  Bundesliga plugin) before wiring in a real one

## 3. Plugin Manifest Registry

- [ ] 3.1 Implement discovery of bundled first-party plugins at startup
- [ ] 3.2 Implement discovery of user-installed plugins from a user plugin directory
- [ ] 3.3 Validate discovered manifests via `fulltime-plugin-api`, skipping and logging
  invalid ones rather than failing startup
- [ ] 3.4 Implement enable/disable state per plugin, persisted across restarts, separate
  from the plugin's own manifest file
- [ ] 3.5 Implement installed-version tracking and update-availability detection for
  plugins with a declared update source

## 4. Plugin Management UI

- [ ] 4.1 Add a `fulltime-ui` screen listing installed plugins (name, version,
  enabled/disabled)
- [ ] 4.2 Add enable/disable controls that take effect immediately
- [ ] 4.3 Add an update-availability indicator per plugin

## 5. App Cutover

- [ ] 5.1 Switch the app's UI/business logic to consume `fulltime-plugin-api`'s canonical
  schema instead of any provider-specific type
- [ ] 5.2 Coordinate with the `Plugins/Bundesliga` reference-plugin change to cut the app
  over to loading Bundesliga data through the plugin path once that plugin is ready
- [ ] 5.3 Remove the feature flag once the Bundesliga plugin path is validated in place of
  any direct SDK dependency

## 6. Verification

- [ ] 6.1 Benchmark plugin-path data fetch latency against an in-process baseline call
- [ ] 6.2 Test sandbox enforcement: confirm a plugin cannot access the filesystem or call
  an undeclared network host
- [ ] 6.3 Test fault isolation: confirm a deliberately panicking plugin does not crash the
  host or other loaded plugins
