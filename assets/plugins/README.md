# Bundled first-party plugins

Each first-party data-provider plugin ships here as its own subdirectory,
embedded into the `FullTime` binary at compile time (see
`crates/fulltime-core/src/plugin_host/bundled.rs`):

```
assets/plugins/<plugin-id>/manifest.toml
assets/plugins/<plugin-id>/plugin.wasm
```

Empty for now — the Bundesliga reference plugin (`Plugins/Bundesliga`) lands
here once `openspec/changes/plugin-host-runtime` task 5.2 wires it in.
