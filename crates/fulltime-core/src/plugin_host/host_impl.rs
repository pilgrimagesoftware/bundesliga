//! Implementation of the `host` interface's `fetch` import: the only network
//! access a plugin has, scoped to the hostnames declared in its manifest.
//!
//! A `wasm32-wasip2` component always imports the standard WASI Preview 2
//! interfaces (`wasi:io/poll`, `wasi:clocks/*`, `wasi:filesystem/*`, ...) as
//! part of the Rust runtime's own startup, whether or not the plugin's code
//! actually calls into them. [`HostState`] wires up `wasmtime-wasi`'s
//! standard implementations of those (see
//! [`PluginHost::new`](super::PluginHost::new)) so instantiation succeeds, but
//! with a [`wasmtime_wasi::WasiCtx`] built from
//! [`wasmtime_wasi::WasiCtxBuilder::new`]'s defaults — no preopened
//! directories, no inherited stdio/env/args/network — so none of it grants a
//! plugin any real capability. `host.fetch` is the only capability a plugin
//! actually has.

use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use super::bindings::fulltime::plugin_api::host::{Host, NetworkFailure};

/// Per-instantiation state for a single plugin call: the hosts that plugin's
/// manifest declared, the client used to reach them, and the (deliberately
/// capability-less) WASI context every `wasm32-wasip2` component needs.
///
/// A fresh [`HostState`] is created for every call (see the instance-per-call
/// design decision in `openspec/changes/plugin-host-runtime/design.md`), so
/// there is no state here that needs to survive across calls.
pub(super) struct HostState {
    allowed_hosts: Vec<String>,
    client:        reqwest::Client,
    wasi_ctx:      WasiCtx,
    wasi_table:    ResourceTable,
}

impl HostState {
    pub(super) fn new(allowed_hosts: Vec<String>) -> Self {
        Self { allowed_hosts,
               client: reqwest::Client::new(),
               wasi_ctx: WasiCtxBuilder::new().build(),
               wasi_table: ResourceTable::new() }
    }
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx:   &mut self.wasi_ctx,
                      table: &mut self.wasi_table, }
    }
}

impl Host for HostState {
    async fn fetch(&mut self, url: String) -> Result<Vec<u8>, NetworkFailure> {
        let parsed = reqwest::Url::parse(&url).map_err(|err| NetworkFailure {
            message: format!("invalid URL {url:?}: {err}"),
        })?;

        let host =
            parsed.host_str()
                  .ok_or_else(|| NetworkFailure { message: format!("URL {url:?} has no host"), })?;

        if !self.allowed_hosts.iter().any(|allowed| allowed == host) {
            return Err(NetworkFailure { message: format!("host {host:?} is not declared in this plugin's manifest network_hosts"), });
        }

        let response = self
            .client
            .get(parsed)
            .send()
            .await
            .map_err(|err| NetworkFailure {
                message: format!("request to {url:?} failed: {err}"),
            })?;

        if !response.status().is_success() {
            return Err(NetworkFailure { message: format!("request to {url:?} returned non-success status {}",
                                                         response.status()), });
        }

        response.bytes().await.map(|b| b.to_vec()).map_err(|err| NetworkFailure {
            message: format!("reading response body from {url:?} failed: {err}"),
        })
    }
}
