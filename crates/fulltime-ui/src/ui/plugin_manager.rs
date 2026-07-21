//! The interface the Plugins screen (`ui::views::plugins`) uses to list and
//! toggle installed data-provider plugins.
//!
//! `fulltime-ui` has no dependency on `wasmtime`/`fulltime-plugin-api` — the
//! real implementation (backed by `fulltime-core`'s `PluginHost`/
//! `PluginRegistry`) lives in `fulltime-core` and is injected as a
//! [`gpui::Global`] before the main window opens (see
//! `fulltime-core`'s `app::plugin_manager`). This keeps the plugin host
//! runtime out of the UI crate's dependency graph, the same way it has no
//! direct dependency on league-data types.

use gpui::Global;

/// A plugin as the Plugins screen displays it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginSummary {
    /// The plugin's manifest-declared `id`. Manifests have no separate
    /// display-name field, so this doubles as the label shown in the UI.
    pub id:      String,
    /// The plugin's own release version (manifest `version` field).
    pub version: String,
    /// Whether the plugin is currently enabled.
    pub enabled: bool,
}

/// Lists and toggles installed plugins. Implemented by `fulltime-core`.
pub trait PluginManager: 'static {
    /// Every discovered plugin (bundled and user-installed), in whatever
    /// order the implementation finds natural — the screen sorts for
    /// display.
    fn list(&self) -> Vec<PluginSummary>;

    /// Enables or disables `id`, taking effect immediately. Implementations
    /// log and otherwise silently ignore a failure (e.g. an unknown `id`);
    /// the next [`Self::list`] call reflects whatever actually happened.
    fn set_enabled(&mut self, id: &str, enabled: bool);
}

/// [`gpui::Global`] wrapper so a boxed [`PluginManager`] can be registered
/// with `cx.set_global`. Absent entirely (no global set) means the plugin
/// host runtime isn't compiled in for this build.
pub struct PluginManagerHandle(pub Box<dyn PluginManager>);

impl Global for PluginManagerHandle {}
