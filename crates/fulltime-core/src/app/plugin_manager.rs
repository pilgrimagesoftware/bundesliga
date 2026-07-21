//! Bridges `fulltime-core`'s [`PluginHost`]/[`PluginRegistry`] into
//! `fulltime-ui`'s [`PluginManager`] trait, so the Plugins screen can list
//! and toggle real plugins without `fulltime-ui` depending on `wasmtime` or
//! `fulltime-plugin-api` — see that trait's own doc comment for why.

use std::path::PathBuf;

use fulltime_ui::ui::plugin_manager::{PluginManager, PluginManagerHandle, PluginSummary};

use crate::plugin_host::PluginHost;
use crate::plugin_host::registry::PluginRegistry;

struct FulltimePluginManager {
    host:     PluginHost,
    registry: PluginRegistry,
}

impl PluginManager for FulltimePluginManager {
    fn list(&self) -> Vec<PluginSummary> {
        self.registry
            .discovered()
            .map(|(id, plugin)| PluginSummary { id:      id.to_owned(),
                                                version: plugin.manifest.version.clone(),
                                                enabled: self.registry.is_enabled(id), })
            .collect()
    }

    fn set_enabled(&mut self, id: &str, enabled: bool) {
        let result = if enabled {
            self.registry
                .enable(&mut self.host, id)
                .map_err(|error| error.to_string())
        }
        else {
            self.registry
                .disable(&mut self.host, id)
                .map_err(|error| error.to_string())
        };

        if let Err(error) = result {
            tracing::warn!(plugin_id = id, %error, "failed to toggle plugin enabled state");
        }
    }
}

/// `~/Library/Application Support/com.pilgrimagesoftware.fulltime` on
/// macOS (matching `logging::platform_log_dir`'s bundle-identifier
/// convention, but under Application Support rather than Logs), or the
/// platform-equivalent local-data directory elsewhere.
fn app_data_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        dirs::home_dir().map(|home| {
                            home.join("Library/Application Support/com.pilgrimagesoftware.fulltime")
                        })
                        .unwrap_or_else(|| PathBuf::from("/tmp/com.pilgrimagesoftware.fulltime"))
    }
    #[cfg(not(target_os = "macos"))]
    {
        dirs::data_local_dir()
            .map(|dir| dir.join("com.pilgrimagesoftware.fulltime"))
            .unwrap_or_else(|| PathBuf::from("/tmp/com.pilgrimagesoftware.fulltime"))
    }
}

/// Builds the real plugin manager: discovers bundled and user-installed
/// plugins, loads the enabled ones into a fresh [`PluginHost`], and returns
/// a handle ready for `cx.set_global`.
///
/// A plugin that fails to load at startup is logged and skipped rather than
/// failing the whole app, matching the registry's own discovery behavior.
///
/// # Errors
/// Returns an error only if the [`PluginHost`] or its state file can't be
/// initialized at all — never for an individual plugin failing to load.
pub fn build() -> anyhow::Result<PluginManagerHandle> {
    let data_dir = app_data_dir();

    let mut registry = PluginRegistry::new(data_dir.join("plugin_state.json"))?;
    registry.discover_bundled();
    registry.discover_user_installed(&data_dir.join("plugins"));

    let mut host = PluginHost::new()?;
    for (plugin_id, error) in registry.load_enabled(&mut host) {
        tracing::warn!(%plugin_id, %error, "failed to load plugin at startup");
    }

    Ok(PluginManagerHandle(Box::new(FulltimePluginManager { host, registry })))
}
