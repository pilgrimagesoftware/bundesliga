//! WASM plugin host runtime.
//!
//! Loads `fulltime-plugin-api`-conformant data-provider plugins into a
//! `wasmtime` Component Model sandbox, with no ambient filesystem or network
//! access — the only capability a plugin has is the `host.fetch` import,
//! scoped to the hostnames its manifest declares. See
//! `openspec/changes/plugin-host-runtime/design.md` for the design decisions
//! behind the instance-per-call model this module uses.
//!
//! Gated behind the `plugin-host` feature; see `Cargo.toml`.
//!
//! Not yet called from `app::run()`: task group 5 ("App Cutover") in
//! `openspec/changes/plugin-host-runtime/tasks.md` wires this in once the
//! app's UI/business logic is ready to consume `fulltime-plugin-api`'s
//! canonical schema. Until then, this module's public API is exercised only
//! by its own tests below.
#![allow(dead_code, reason = "consumed by app cutover in task group 5")]

mod bindings;
mod bundled;
mod error;
mod host_impl;
pub mod registry;
#[cfg(test)]
mod test_support;

use std::collections::HashMap;
use std::path::Path;

use bindings::Plugin;
use bindings::fulltime::plugin_api::errors::ProviderError;
pub use bindings::fulltime::plugin_api::types::{Competition, Fixture, Standings};
pub use error::PluginHostError;
use fulltime_plugin_api::{INTERFACE_VERSION, Manifest, SCHEMA_VERSION};
use host_impl::HostState;
use wasmtime::component::{Component, HasSelf, Linker};
use wasmtime::{Config, Engine, Store};

/// A plugin whose manifest has been validated and whose component bytes have
/// been compiled, ready to instantiate on demand.
struct LoadedPlugin {
    component: Component,
    manifest:  Manifest,
}

/// Loads and invokes data-provider plugins.
///
/// Each data-provider call creates a fresh [`Store`] and instantiates a new
/// component instance (see the design doc's instance-per-call rationale):
/// a trap in one call cannot corrupt state a later call on the same plugin
/// depends on, and cannot affect any other loaded plugin, since every
/// instance is independent.
pub struct PluginHost {
    engine:  Engine,
    linker:  Linker<HostState>,
    runtime: tokio::runtime::Runtime,
    plugins: HashMap<String, LoadedPlugin>,
}

impl PluginHost {
    /// Creates a new, empty plugin host.
    ///
    /// # Errors
    /// Returns an error if the underlying `wasmtime` engine or the async
    /// runtime used to drive plugin calls fails to initialize.
    pub fn new() -> anyhow::Result<Self> {
        let engine = Engine::new(&Config::new())?;

        let mut linker = Linker::<HostState>::new(&engine);
        wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
        bindings::fulltime::plugin_api::host::add_to_linker::<HostState, HasSelf<HostState>>(
            &mut linker,
            |state| state,
        )?;

        let runtime = tokio::runtime::Builder::new_multi_thread().enable_all()
                                                                 .build()?;

        Ok(Self { engine,
                  linker,
                  runtime,
                  plugins: HashMap::new() })
    }

    /// Loads (or reloads) a plugin from its manifest and component files on
    /// disk — the path a user-installed plugin discovered by
    /// [`registry`](super::registry) is loaded through.
    ///
    /// Reloading an already-loaded plugin (same `id`) replaces the compiled
    /// component in place; the next call to that plugin uses the new bytes.
    /// No app restart is required in either case.
    ///
    /// # Errors
    /// Returns [`PluginHostError::ManifestIo`] if the manifest cannot be
    /// read, [`PluginHostError::ComponentIo`] if the component bytes cannot
    /// be read, or any error [`Self::load_from_source`] returns.
    pub fn load(&mut self, manifest_path: &Path, component_path: &Path)
                -> Result<(), PluginHostError> {
        let manifest_source = std::fs::read_to_string(manifest_path).map_err(|source| {
                                  PluginHostError::ManifestIo { location:
                                                                    manifest_path.display()
                                                                                 .to_string(),
                                                                source }
                              })?;
        let component_bytes = std::fs::read(component_path).map_err(|source| {
                                                               PluginHostError::ComponentIo {
                                              location: component_path.display().to_string(),
                                              source,
                                          }
                                                           })?;

        self.load_from_source(&manifest_path.display().to_string(),
                              &manifest_source,
                              component_bytes)
    }

    /// Loads (or reloads) a plugin from an already-read manifest source and
    /// component bytes — the path a bundled plugin discovered by
    /// [`registry`](super::registry), embedded into the binary rather than
    /// present as a real file, is loaded through.
    ///
    /// `location` is a human-readable identifier used only in error
    /// messages (a filesystem path for [`Self::load`], or a descriptive
    /// label like `"bundled:<id>"` for an embedded plugin).
    ///
    /// # Errors
    /// Returns [`PluginHostError::InvalidManifest`] if the manifest doesn't
    /// parse, [`PluginHostError::IncompatibleVersion`] if the plugin targets
    /// a schema/interface version this host does not support, or
    /// [`PluginHostError::Compile`] if the component bytes don't compile as
    /// a valid component.
    pub fn load_from_source(&mut self, location: &str, manifest_source: &str,
                            component_bytes: Vec<u8>)
                            -> Result<(), PluginHostError> {
        let manifest = Manifest::parse(manifest_source).map_err(|source| {
                                                           PluginHostError::InvalidManifest {
                                                 location: location.to_owned(),
                                                 source,
                                             }
                                                       })?;

        if !SCHEMA_VERSION.accepts(manifest.schema_version)
           || !INTERFACE_VERSION.accepts(manifest.interface_version)
        {
            return Err(PluginHostError::IncompatibleVersion { plugin_id:        manifest.id,
                                                              plugin_schema:
                                                                  manifest.schema_version,
                                                              plugin_interface:
                                                                  manifest.interface_version,
                                                              host_schema:      SCHEMA_VERSION,
                                                              host_interface:   INTERFACE_VERSION, });
        }

        let component = Component::new(&self.engine, component_bytes).map_err(|source| {
                                                                         PluginHostError::Compile {
                plugin_id: manifest.id.clone(),
                source: source.into(),
            }
                                                                     })?;

        self.plugins.insert(manifest.id.clone(),
                            LoadedPlugin { component,
                                           manifest });
        Ok(())
    }

    /// Unloads a plugin, dropping its compiled component. A later call for
    /// this `id` fails with [`PluginHostError::NotLoaded`] until it is
    /// [`load`](Self::load)ed again.
    ///
    /// # Errors
    /// Returns [`PluginHostError::NotLoaded`] if no plugin with this `id` is
    /// loaded.
    pub fn unload(&mut self, plugin_id: &str) -> Result<(), PluginHostError> {
        self.plugins.remove(plugin_id).map(|_| ()).ok_or_else(|| {
                                                      PluginHostError::NotLoaded {
                plugin_id: plugin_id.to_owned(),
            }
                                                  })
    }

    /// Returns whether a plugin with this `id` is currently loaded.
    #[must_use]
    pub fn is_loaded(&self, plugin_id: &str) -> bool {
        self.plugins.contains_key(plugin_id)
    }

    /// Lists the competitions a plugin can supply data for.
    ///
    /// # Errors
    /// Returns [`PluginHostError::NotLoaded`] if the plugin isn't loaded,
    /// [`PluginHostError::Call`] if instantiation or the call itself fails
    /// (including a caught plugin trap), or [`PluginHostError::ProviderError`]
    /// if the plugin's call completed but reported a structured failure.
    pub fn list_competitions(&mut self, plugin_id: &str)
                             -> Result<Vec<Competition>, PluginHostError> {
        let (mut store, component, linker) = self.prepare(plugin_id)?;
        let plugin_id = plugin_id.to_owned();
        self.runtime.block_on(async move {
                        let instance =
                            Self::instantiate(&plugin_id, &mut store, &component, &linker).await?;
                        let result = instance.fulltime_plugin_api_data_provider()
                                             .call_list_competitions(&mut store)
                                             .await;
                        Self::fold(&plugin_id, result)
                    })
    }

    /// Fetches upcoming and in-progress fixtures for a competition.
    ///
    /// # Errors
    /// See [`Self::list_competitions`].
    pub fn fetch_fixtures(&mut self, plugin_id: &str, competition_id: &str)
                          -> Result<Vec<Fixture>, PluginHostError> {
        let (mut store, component, linker) = self.prepare(plugin_id)?;
        let plugin_id = plugin_id.to_owned();
        let competition_id = competition_id.to_owned();
        self.runtime.block_on(async move {
                        let instance =
                            Self::instantiate(&plugin_id, &mut store, &component, &linker).await?;
                        let result = instance.fulltime_plugin_api_data_provider()
                                             .call_fetch_fixtures(&mut store, &competition_id)
                                             .await;
                        Self::fold(&plugin_id, result)
                    })
    }

    /// Fetches completed fixtures (results) for a competition.
    ///
    /// # Errors
    /// See [`Self::list_competitions`].
    pub fn fetch_results(&mut self, plugin_id: &str, competition_id: &str)
                         -> Result<Vec<Fixture>, PluginHostError> {
        let (mut store, component, linker) = self.prepare(plugin_id)?;
        let plugin_id = plugin_id.to_owned();
        let competition_id = competition_id.to_owned();
        self.runtime.block_on(async move {
                        let instance =
                            Self::instantiate(&plugin_id, &mut store, &component, &linker).await?;
                        let result = instance.fulltime_plugin_api_data_provider()
                                             .call_fetch_results(&mut store, &competition_id)
                                             .await;
                        Self::fold(&plugin_id, result)
                    })
    }

    /// Fetches standings for a competition.
    ///
    /// # Errors
    /// See [`Self::list_competitions`].
    pub fn fetch_standings(&mut self, plugin_id: &str, competition_id: &str)
                           -> Result<Standings, PluginHostError> {
        let (mut store, component, linker) = self.prepare(plugin_id)?;
        let plugin_id = plugin_id.to_owned();
        let competition_id = competition_id.to_owned();
        self.runtime.block_on(async move {
                        let instance =
                            Self::instantiate(&plugin_id, &mut store, &component, &linker).await?;
                        let result = instance.fulltime_plugin_api_data_provider()
                                             .call_fetch_standings(&mut store, &competition_id)
                                             .await;
                        Self::fold(&plugin_id, result)
                    })
    }

    /// Fetches metadata for a single competition.
    ///
    /// # Errors
    /// See [`Self::list_competitions`].
    pub fn fetch_metadata(&mut self, plugin_id: &str, competition_id: &str)
                          -> Result<Competition, PluginHostError> {
        let (mut store, component, linker) = self.prepare(plugin_id)?;
        let plugin_id = plugin_id.to_owned();
        let competition_id = competition_id.to_owned();
        self.runtime.block_on(async move {
                        let instance =
                            Self::instantiate(&plugin_id, &mut store, &component, &linker).await?;
                        let result = instance.fulltime_plugin_api_data_provider()
                                             .call_fetch_metadata(&mut store, &competition_id)
                                             .await;
                        Self::fold(&plugin_id, result)
                    })
    }

    /// Looks up `plugin_id` and builds the fresh, independent [`Store`] and
    /// cloned [`Component`]/[`Linker`] handles the instance-per-call model
    /// needs for one call (see the module doc's fault-isolation rationale).
    ///
    /// [`Component`] and [`Linker`] are cheap to clone (both are `Arc`-backed
    /// handles into data owned by [`Engine`]); the [`Store`] is the only part
    /// actually created fresh per call.
    fn prepare(&self, plugin_id: &str)
               -> Result<(Store<HostState>, Component, Linker<HostState>), PluginHostError> {
        let plugin = self.plugins
                         .get(plugin_id)
                         .ok_or_else(|| PluginHostError::NotLoaded { plugin_id:
                                                                         plugin_id.to_owned(), })?;

        let host_state = HostState::new(plugin.manifest.network_hosts.clone());
        let store = Store::new(&self.engine, host_state);
        Ok((store, plugin.component.clone(), self.linker.clone()))
    }

    /// Instantiates a freshly-prepared component. The caller reaches its
    /// `data-provider` export via `.fulltime_plugin_api_data_provider()` on
    /// the returned instance, which borrows from it — so the instance must
    /// stay alive for as long as that accessor's result is in use.
    async fn instantiate(plugin_id: &str, store: &mut Store<HostState>, component: &Component,
                         linker: &Linker<HostState>)
                         -> Result<Plugin, PluginHostError> {
        Plugin::instantiate_async(store, component, linker).await
                                                           .map_err(|source| {
                                                               PluginHostError::Call {
                plugin_id: plugin_id.to_owned(),
                source: source.into(),
            }
                                                           })
    }

    /// Folds a raw call result — an outer [`wasmtime::Result`] (instantiation
    /// or a caught trap) around an inner `result<T, provider-error>` (the
    /// plugin's own reported outcome) — into [`PluginHostError`] uniformly.
    fn fold<T>(plugin_id: &str, result: wasmtime::Result<Result<T, ProviderError>>)
               -> Result<T, PluginHostError> {
        match result {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(provider_error)) => {
                Err(PluginHostError::ProviderError { plugin_id: plugin_id.to_owned(),
                                                     source:    provider_error, })
            }
            Err(source) => Err(PluginHostError::Call { plugin_id: plugin_id.to_owned(),
                                                       source:    source.into(), }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    /// Asserts the vendored `wit/data-provider.wit` in this crate stays
    /// byte-identical to the copy shipped inside the `fulltime-plugin-api`
    /// crate this workspace resolved, so the host-side bindings generated
    /// here can't silently drift from the contract `fulltime-plugin-api`
    /// itself was built against.
    #[test]
    fn vendored_wit_matches_fulltime_plugin_api() -> Result<(), Box<dyn std::error::Error>> {
        // `fulltime-plugin-api` is an optional dependency gated by the
        // `plugin-host` feature, so it's absent from `cargo metadata`'s
        // resolved package graph unless all features are requested.
        let metadata =
            cargo_metadata::MetadataCommand::new().features(cargo_metadata::CargoOpt::AllFeatures)
                                                  .exec()?;

        let package = metadata.packages
                              .iter()
                              .find(|package| package.name.as_str() == "fulltime-plugin-api")
                              .ok_or("fulltime-plugin-api should be a resolved dependency")?;

        let upstream_wit = package.manifest_path
                                  .parent()
                                  .ok_or("manifest path should have a parent directory")?
                                  .join("wit")
                                  .join("data-provider.wit");

        let upstream = std::fs::read_to_string(&upstream_wit)?;
        let vendored = std::fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("wit/data-provider.wit"),
        )?;

        assert_eq!(
                   upstream, vendored,
                   "wit/data-provider.wit has drifted from fulltime-plugin-api {}; \
             re-copy it from that crate's own wit/data-provider.wit",
                   package.version
        );
        Ok(())
    }

    use super::test_support::build_fixture;

    /// Loads the fixture plugin and drives every operation through it,
    /// covering the whole of task group 2: loading with version validation,
    /// the host `fetch` capability's scoping, fault isolation from a
    /// deliberately panicking call, and unload/reload without a restart.
    #[test]
    fn plugin_host_runtime_end_to_end() -> Result<(), Box<dyn std::error::Error>> {
        let (manifest_path, wasm_path) = build_fixture()?;

        let mut host = super::PluginHost::new()?;
        host.load(&manifest_path, &wasm_path)?;
        assert!(host.is_loaded("fixture"));

        let competitions = host.list_competitions("fixture")?;
        assert_eq!(competitions.len(), 1);
        assert_eq!(competitions[0].id, "fixture-competition");

        assert_eq!(host.fetch_fixtures("fixture", "any")?.len(), 1);
        assert_eq!(host.fetch_results("fixture", "any")?.len(), 1);
        assert_eq!(host.fetch_standings("fixture", "any")?.groups.len(), 1);
        assert_eq!(host.fetch_metadata("fixture", "any")?.id,
                   "fixture-competition");

        // Fault isolation: a panicking call comes back as an error, not a
        // host crash, and a later call on the same plugin still succeeds.
        let panicked = host.fetch_fixtures("fixture", "panic");
        assert!(matches!(panicked, Err(super::PluginHostError::Call { .. })));
        assert_eq!(host.list_competitions("fixture")?.len(), 1);

        // Network capability scoping: a fetch to a host this plugin's
        // manifest doesn't declare is rejected without making a network call.
        let undeclared_host = host.fetch_metadata("fixture", "undeclared-host");
        assert!(matches!(undeclared_host,
                         Err(super::PluginHostError::ProviderError { .. })));

        // Unload/reload without an app restart.
        host.unload("fixture")?;
        assert!(!host.is_loaded("fixture"));
        assert!(matches!(host.list_competitions("fixture"),
                         Err(super::PluginHostError::NotLoaded { .. })));

        host.load(&manifest_path, &wasm_path)?;
        assert!(host.is_loaded("fixture"));
        assert_eq!(host.list_competitions("fixture")?.len(), 1);

        Ok(())
    }
}
