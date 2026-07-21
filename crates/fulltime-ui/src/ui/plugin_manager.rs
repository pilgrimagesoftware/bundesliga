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

/// One row of a fetched standings table, mirroring `fulltime-plugin-api`'s
/// canonical `standings-row` shape minus any field the Standings screen
/// doesn't render. There is no "recent form" field in the canonical schema
/// (`Fixture`/`Standings`/`Competition`/`Team` only), so the screen's Form
/// column has nothing to show for real rows — see
/// `ui::views::standings`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandingsRowSnapshot {
    pub team_name:     String,
    pub rank:          u16,
    pub played:        u16,
    pub won:           u16,
    pub drawn:         u16,
    pub lost:          u16,
    pub goals_for:     u16,
    pub goals_against: u16,
    pub points:        u16,
}

/// A fetched standings table for one competition, single-group only (the
/// Standings screen doesn't yet render group-stage tournaments).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StandingsSnapshot {
    pub competition_name: String,
    pub rows:             Vec<StandingsRowSnapshot>,
}

/// Lists and toggles installed plugins, and exposes the one real league-data
/// fetch wired up so far. Implemented by `fulltime-core`.
pub trait PluginManager: 'static {
    /// Every discovered plugin (bundled and user-installed), in whatever
    /// order the implementation finds natural — the screen sorts for
    /// display.
    fn list(&self) -> Vec<PluginSummary>;

    /// Enables or disables `id`, taking effect immediately. Implementations
    /// log and otherwise silently ignore a failure (e.g. an unknown `id`);
    /// the next [`Self::list`] call reflects whatever actually happened.
    fn set_enabled(&mut self, id: &str, enabled: bool);

    /// The Bundesliga plugin's standings table for its current season,
    /// fetched once at startup. `None` if the plugin isn't loaded/enabled or
    /// the fetch failed — the Standings screen falls back to its mockup
    /// layout in that case.
    fn standings(&self) -> Option<StandingsSnapshot>;
}

/// [`gpui::Global`] wrapper so a boxed [`PluginManager`] can be registered
/// with `cx.set_global`. Absent entirely (no global set) means the plugin
/// host runtime isn't compiled in for this build.
pub struct PluginManagerHandle(pub Box<dyn PluginManager>);

impl Global for PluginManagerHandle {}
