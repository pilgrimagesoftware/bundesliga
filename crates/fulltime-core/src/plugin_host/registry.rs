//! Plugin manifest registry: discovers bundled and user-installed plugins,
//! and tracks per-plugin enable/disable state persisted separately from
//! each plugin's own manifest file (see
//! `openspec/changes/plugin-host-runtime/design.md`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use fulltime_plugin_api::Manifest;
use serde::{Deserialize, Serialize};

use super::bundled;
use super::{PluginHost, PluginHostError};

/// Where a discovered plugin's manifest and component came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginSource {
    /// Embedded into the binary from `assets/plugins/`.
    Bundled,
    /// Found on disk under the user plugin directory.
    UserInstalled,
}

/// Enough to reload a discovered plugin into a [`PluginHost`] without
/// re-scanning: either the file paths of a user-installed plugin, or the
/// already-embedded bytes of a bundled one.
enum PluginArtifact {
    Bundled {
        label:           String,
        manifest_source: String,
        component_bytes: Vec<u8>,
    },
    UserInstalled {
        manifest_path:  PathBuf,
        component_path: PathBuf,
    },
}

/// A plugin the registry has discovered and validated, ready to load.
pub struct DiscoveredPlugin {
    /// The plugin's parsed, valid manifest.
    pub manifest: Manifest,
    /// Where this plugin was discovered.
    pub source:   PluginSource,
    artifact:     PluginArtifact,
}

impl DiscoveredPlugin {
    fn load_into(&self, host: &mut PluginHost) -> Result<(), PluginHostError> {
        match &self.artifact {
            PluginArtifact::Bundled { label,
                                      manifest_source,
                                      component_bytes, } => {
                host.load_from_source(&format!("bundled:{label}"),
                                      manifest_source,
                                      component_bytes.clone())
            }
            PluginArtifact::UserInstalled { manifest_path,
                                            component_path, } => {
                host.load(manifest_path, component_path)
            }
        }
    }
}

/// Errors persisting or reading the enable/disable state file.
#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    /// The state file exists but could not be read.
    #[error("failed to read plugin state file at {path}: {source}")]
    Io {
        /// Path to the state file.
        path:   PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The state file's contents are not valid JSON.
    #[error("plugin state file at {path} is not valid JSON: {source}")]
    Parse {
        /// Path to the state file.
        path:   PathBuf,
        /// Underlying parse error.
        #[source]
        source: serde_json::Error,
    },

    /// The state file could not be written.
    #[error("failed to write plugin state file at {path}: {source}")]
    Write {
        /// Path to the state file.
        path:   PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
}

/// Per-plugin enable/disable state, persisted as JSON separately from any
/// plugin's own manifest file. A plugin absent from `enabled` (never
/// explicitly toggled) is enabled by default.
#[derive(Default, Serialize, Deserialize)]
struct PersistedState {
    #[serde(default)]
    enabled: HashMap<String, bool>,
}

/// Discovers bundled and user-installed plugins and tracks their
/// enable/disable state.
pub struct PluginRegistry {
    discovered: HashMap<String, DiscoveredPlugin>,
    state:      PersistedState,
    state_path: PathBuf,
}

impl PluginRegistry {
    /// Creates a registry backed by the given state file path, loading any
    /// previously persisted enable/disable state. A missing file is treated
    /// as "no plugin has been toggled yet", not an error.
    ///
    /// # Errors
    /// Returns [`RegistryError::Io`] or [`RegistryError::Parse`] if
    /// the state file exists but can't be read or parsed.
    pub fn new(state_path: PathBuf) -> Result<Self, RegistryError> {
        let state = match std::fs::read_to_string(&state_path) {
            Ok(source) => {
                serde_json::from_str(&source).map_err(|source| {
                                                 RegistryError::Parse { path: state_path.clone(),
                                                                        source }
                                             })?
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => PersistedState::default(),
            Err(source) => {
                return Err(RegistryError::Io { path: state_path.clone(),
                                               source });
            }
        };

        Ok(Self { discovered: HashMap::new(),
                  state,
                  state_path })
    }

    /// Discovers plugins embedded into the binary under `assets/plugins/`.
    /// A subdirectory with an invalid manifest is logged and skipped, not
    /// treated as a discovery failure.
    pub fn discover_bundled(&mut self) {
        for plugin in bundled::bundled_plugins() {
            match Manifest::parse(&plugin.manifest_source) {
                Ok(manifest) => {
                    self.discovered.insert(manifest.id.clone(), DiscoveredPlugin {
                        manifest,
                        source: PluginSource::Bundled,
                        artifact: PluginArtifact::Bundled {
                            label:            plugin.label,
                            manifest_source:  plugin.manifest_source,
                            component_bytes:  plugin.component_bytes,
                        },
                    });
                }
                Err(error) => {
                    tracing::warn!(
                        plugin = %plugin.label,
                        %error,
                        "skipping bundled plugin: invalid manifest"
                    );
                }
            }
        }
    }

    /// Discovers plugins on disk under `dir`, one subdirectory per plugin
    /// (`<dir>/<id>/manifest.toml` + `<dir>/<id>/plugin.wasm`). A missing
    /// directory is treated as "no user plugins installed", not an error;
    /// a subdirectory with an invalid manifest or unreadable component is
    /// logged and skipped.
    pub fn discover_user_installed(&mut self, dir: &Path) {
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => {
                tracing::warn!(dir = %dir.display(), %error, "failed to read user plugin directory");
                return;
            }
        };

        for entry in entries.flatten() {
            let plugin_dir = entry.path();
            if !plugin_dir.is_dir() {
                continue;
            }

            let manifest_path = plugin_dir.join("manifest.toml");
            let manifest_source = match std::fs::read_to_string(&manifest_path) {
                Ok(source) => source,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => {
                    tracing::warn!(
                        path = %manifest_path.display(),
                        %error,
                        "skipping user plugin: failed to read manifest"
                    );
                    continue;
                }
            };

            match Manifest::parse(&manifest_source) {
                Ok(manifest) => {
                    let component_path = plugin_dir.join("plugin.wasm");
                    self.discovered.insert(manifest.id.clone(), DiscoveredPlugin {
                        manifest,
                        source: PluginSource::UserInstalled,
                        artifact: PluginArtifact::UserInstalled {
                            manifest_path,
                            component_path,
                        },
                    });
                }
                Err(error) => {
                    tracing::warn!(
                        path = %manifest_path.display(),
                        %error,
                        "skipping user plugin: invalid manifest"
                    );
                }
            }
        }
    }

    /// Iterates every discovered plugin (bundled and user-installed).
    pub fn discovered(&self) -> impl Iterator<Item = (&str, &DiscoveredPlugin)> {
        self.discovered
            .iter()
            .map(|(id, plugin)| (id.as_str(), plugin))
    }

    /// Whether `plugin_id` is enabled. A plugin never explicitly toggled
    /// defaults to enabled.
    #[must_use]
    pub fn is_enabled(&self, plugin_id: &str) -> bool {
        self.state.enabled.get(plugin_id).copied().unwrap_or(true)
    }

    /// Sets `plugin_id`'s enabled state and persists it immediately, so the
    /// choice survives a restart.
    ///
    /// This only updates the registry's own record — it does not load or
    /// unload the plugin in a running [`PluginHost`]; see
    /// [`Self::enable`]/[`Self::disable`] for the combined operation the
    /// plugin management UI (task group 4) uses.
    ///
    /// # Errors
    /// Returns [`RegistryError::Write`] if the state file can't be written.
    pub fn set_enabled(&mut self, plugin_id: &str, enabled: bool) -> Result<(), RegistryError> {
        self.state.enabled.insert(plugin_id.to_owned(), enabled);
        self.persist()
    }

    /// Enables `plugin_id`, persists that choice, and loads it into `host`
    /// immediately — no app restart required.
    ///
    /// # Errors
    /// Returns [`RegistryError::Write`] if the state file can't be
    /// written, or the [`PluginHostError`] loading the plugin returned.
    pub fn enable(&mut self, host: &mut PluginHost, plugin_id: &str) -> Result<(), EnableError> {
        self.set_enabled(plugin_id, true)
            .map_err(EnableError::Registry)?;
        let plugin = self.discovered
                         .get(plugin_id)
                         .ok_or_else(|| EnableError::NotDiscovered { plugin_id:
                                                                         plugin_id.to_owned(), })?;
        plugin.load_into(host).map_err(EnableError::Host)
    }

    /// Disables `plugin_id`, persists that choice, and unloads it from
    /// `host` immediately, without requiring the plugin to have been
    /// currently loaded.
    ///
    /// # Errors
    /// Returns [`RegistryError::Write`] if the state file can't be written.
    pub fn disable(&mut self, host: &mut PluginHost, plugin_id: &str) -> Result<(), RegistryError> {
        self.set_enabled(plugin_id, false)?;
        let _ = host.unload(plugin_id);
        Ok(())
    }

    /// Loads every discovered, enabled plugin into `host` — called once at
    /// startup, after
    /// [`Self::discover_bundled`]/[`Self::discover_user_installed`].
    /// A plugin that fails to load is reported in the returned list rather
    /// than aborting the rest.
    pub fn load_enabled(&self, host: &mut PluginHost) -> Vec<(String, PluginHostError)> {
        let mut failures = Vec::new();
        for (id, plugin) in &self.discovered {
            if !self.is_enabled(id) {
                continue;
            }
            if let Err(error) = plugin.load_into(host) {
                failures.push((id.clone(), error));
            }
        }
        failures
    }

    fn persist(&self) -> Result<(), RegistryError> {
        if let Some(parent) = self.state_path.parent() {
            drop(std::fs::create_dir_all(parent));
        }
        let source = serde_json::to_string_pretty(&self.state).unwrap_or_default();
        std::fs::write(&self.state_path, source).map_err(|source| {
                                                    RegistryError::Write { path: self.state_path
                                                                                     .clone(),
                                                                           source }
                                                })
    }
}

/// Errors from [`PluginRegistry::enable`], which combines persisting state
/// with an immediate load.
#[derive(Debug, thiserror::Error)]
pub enum EnableError {
    /// The enabled state couldn't be persisted.
    #[error(transparent)]
    Registry(#[from] RegistryError),

    /// `plugin_id` was enabled but has never been discovered, so there is
    /// nothing to load.
    #[error("plugin {plugin_id:?} was enabled but has not been discovered")]
    NotDiscovered {
        /// The plugin ID that was requested.
        plugin_id: String,
    },

    /// The plugin was discovered but failed to load.
    #[error(transparent)]
    Host(#[from] PluginHostError),
}

#[cfg(test)]
mod tests {
    use super::super::PluginHost;
    use super::super::test_support::build_fixture;
    use super::*;

    /// Copies the built fixture plugin into `<dir>/fixture/` in the layout
    /// [`PluginRegistry::discover_user_installed`] expects.
    fn install_fixture_into(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let (manifest_path, wasm_path) = build_fixture()?;
        let plugin_dir = dir.join("fixture");
        std::fs::create_dir_all(&plugin_dir)?;
        std::fs::copy(manifest_path, plugin_dir.join("manifest.toml"))?;
        std::fs::copy(wasm_path, plugin_dir.join("plugin.wasm"))?;
        Ok(())
    }

    /// Discovers and loads the real Bundesliga plugin, vendored into
    /// `assets/plugins/bundesliga/` by `scripts/vendor-bundesliga-plugin.sh`
    /// (see that script and `plugins/bundesliga`, a git submodule). Only
    /// compiles and instantiates the component — it does not call any
    /// data-provider operation, since every one of them makes a live
    /// request to `api.openligadb.de` (see
    /// `discover_and_call_bundled_bundesliga_plugin_live` for that, ignored
    /// by default the same way `plugins/bundesliga`'s own live tests are).
    ///
    /// Run `./scripts/vendor-bundesliga-plugin.sh` first if this fails with
    /// zero discovered plugins.
    #[test]
    fn discover_bundled_finds_and_loads_the_real_bundesliga_plugin(
        )
        -> Result<(), Box<dyn std::error::Error>>
    {
        let state_dir = tempfile::tempdir()?;
        let mut registry = PluginRegistry::new(state_dir.path().join("state.json"))?;

        registry.discover_bundled();
        assert_eq!(registry.discovered().count(), 1);
        let (id, plugin) = registry.discovered()
                                   .next()
                                   .ok_or("expected one discovered plugin")?;
        assert_eq!(id, "bundesliga");
        assert_eq!(plugin.source, PluginSource::Bundled);

        let mut host = PluginHost::new()?;
        let failures = registry.load_enabled(&mut host);
        assert!(failures.is_empty(),
                "unexpected load failures: {failures:?}");
        assert!(host.is_loaded("bundesliga"));

        Ok(())
    }

    /// Calls the real Bundesliga plugin's `list_competitions` through the
    /// full discovery -> load -> call path, against the live
    /// `api.openligadb.de` API. Ignored by default (live network call, not
    /// suitable for routine `cargo test` runs); run explicitly with
    /// `cargo test --features plugin-host -- --ignored`.
    #[test]
    #[ignore = "makes a live request to api.openligadb.de"]
    fn discover_and_call_bundled_bundesliga_plugin_live(
        )
        -> Result<(), Box<dyn std::error::Error>>
    {
        let state_dir = tempfile::tempdir()?;
        let mut registry = PluginRegistry::new(state_dir.path().join("state.json"))?;
        registry.discover_bundled();

        let mut host = PluginHost::new()?;
        registry.load_enabled(&mut host);

        let competitions = host.list_competitions("bundesliga")?;
        assert!(!competitions.is_empty());

        Ok(())
    }

    #[test]
    fn discover_user_installed_finds_and_loads_a_copied_fixture(
        )
        -> Result<(), Box<dyn std::error::Error>>
    {
        let plugin_dir = tempfile::tempdir()?;
        let state_dir = tempfile::tempdir()?;
        install_fixture_into(plugin_dir.path())?;

        let mut registry = PluginRegistry::new(state_dir.path().join("state.json"))?;
        registry.discover_user_installed(plugin_dir.path());
        assert_eq!(registry.discovered().count(), 1);
        assert!(registry.is_enabled("fixture"));

        let mut host = PluginHost::new()?;
        let failures = registry.load_enabled(&mut host);
        assert!(failures.is_empty(),
                "unexpected load failures: {failures:?}");
        assert!(host.is_loaded("fixture"));
        assert_eq!(host.list_competitions("fixture")?.len(), 1);

        Ok(())
    }

    #[test]
    fn discover_user_installed_treats_a_missing_directory_as_no_plugins(
        )
        -> Result<(), Box<dyn std::error::Error>>
    {
        let state_dir = tempfile::tempdir()?;
        let mut registry = PluginRegistry::new(state_dir.path().join("state.json"))?;

        registry.discover_user_installed(&state_dir.path().join("does-not-exist"));

        assert_eq!(registry.discovered().count(), 0);
        Ok(())
    }

    #[test]
    fn enabled_state_persists_across_registry_instances(
        )
        -> Result<(), Box<dyn std::error::Error>>
    {
        let state_dir = tempfile::tempdir()?;
        let state_path = state_dir.path().join("state.json");

        let mut registry = PluginRegistry::new(state_path.clone())?;
        assert!(registry.is_enabled("never-toggled"),
                "absent id should default to enabled");
        registry.set_enabled("fixture", false)?;

        let reloaded = PluginRegistry::new(state_path)?;
        assert!(!reloaded.is_enabled("fixture"));
        assert!(reloaded.is_enabled("never-toggled"));

        Ok(())
    }

    #[test]
    fn disable_then_enable_take_effect_without_a_restart(
        )
        -> Result<(), Box<dyn std::error::Error>>
    {
        let plugin_dir = tempfile::tempdir()?;
        let state_dir = tempfile::tempdir()?;
        install_fixture_into(plugin_dir.path())?;

        let mut registry = PluginRegistry::new(state_dir.path().join("state.json"))?;
        registry.discover_user_installed(plugin_dir.path());

        let mut host = PluginHost::new()?;
        registry.load_enabled(&mut host);
        assert!(host.is_loaded("fixture"));

        registry.disable(&mut host, "fixture")?;
        assert!(!host.is_loaded("fixture"));
        assert!(!registry.is_enabled("fixture"));

        registry.enable(&mut host, "fixture")?;
        assert!(host.is_loaded("fixture"));
        assert!(registry.is_enabled("fixture"));

        Ok(())
    }
}
