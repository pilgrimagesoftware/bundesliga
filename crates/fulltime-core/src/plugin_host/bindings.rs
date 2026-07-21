//! Host-side Component Model bindings for the `plugin` world.
//!
//! Generated from `wit/data-provider.wit`, a vendored copy of
//! `fulltime-plugin-api`'s contract (currently pinned to `0.1.0`, matching the
//! `fulltime-plugin-api = "0.1.1"` dependency in `Cargo.toml` — the WIT package
//! version and the crate version differ because the WIT package version only
//! bumps on a schema/interface change, not every crate release). The
//! `wit_sync` test in `super::tests` asserts this copy stays byte-identical to
//! the one shipped inside the `fulltime-plugin-api` crate itself.
//!
//! Imports and exports are both async because the `host.fetch` import is
//! backed by `reqwest`'s async client, driven by the dedicated Tokio runtime
//! in [`super::PluginHost`].

wasmtime::component::bindgen!({
    world: "plugin",
    path: "wit",
    imports: { default: async },
    exports: { default: async },
});
