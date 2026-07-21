//! Bridges `fulltime-core`'s [`PluginHost`]/[`PluginRegistry`] into
//! `fulltime-ui`'s [`PluginManager`] trait, so the Plugins screen can list
//! and toggle real plugins without `fulltime-ui` depending on `wasmtime` or
//! `fulltime-plugin-api` — see that trait's own doc comment for why.

use std::path::PathBuf;

use fulltime_ui::ui::plugin_manager::{
    CompetitionSummary, LeagueSummary, PluginManager, PluginManagerHandle, PluginSummary,
    StandingsRowSnapshot, StandingsSnapshot,
};

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

    fn available_leagues(&self) -> Vec<LeagueSummary> {
        let locale = rust_i18n::locale();
        self.registry
            .discovered()
            .filter(|(id, _)| self.host.is_loaded(id))
            .map(|(id, plugin)| LeagueSummary { plugin_id:    id.to_owned(),
                                                display_name: plugin.manifest
                                                                    .localized_name(&locale)
                                                                    .to_owned(), })
            .collect()
    }

    fn competitions(&mut self, plugin_id: &str) -> Vec<CompetitionSummary> {
        let mut competitions = match self.host.list_competitions(plugin_id) {
            Ok(competitions) => competitions,
            Err(error) => {
                tracing::warn!(%plugin_id, %error, "failed to list competitions");
                return Vec::new();
            }
        };

        // Sort most-recent-first, using the trailing numeric component of
        // the competition id as a recency heuristic (e.g. Plugins/
        // Bundesliga issues `bl1-<season>` ids) — the canonical schema has
        // no structured season field to sort on instead.
        competitions.sort_by_key(|competition| {
                        std::cmp::Reverse(competition.id
                                                     .rsplit('-')
                                                     .next()
                                                     .and_then(|suffix| suffix.parse::<u32>().ok())
                                                     .unwrap_or(0))
                    });

        competitions.into_iter()
                    .map(|competition| CompetitionSummary { id:   competition.id,
                                                            name: competition.name, })
                    .collect()
    }

    fn fetch_standings(&mut self, plugin_id: &str, competition_id: &str)
                       -> Option<StandingsSnapshot> {
        let standings = match self.host.fetch_standings(plugin_id, competition_id) {
            Ok(standings) => standings,
            Err(error) => {
                tracing::warn!(%plugin_id, %competition_id, %error, "failed to fetch standings");
                return None;
            }
        };

        let rows = standings.groups
                            .into_iter()
                            .flat_map(|group| group.rows)
                            .map(|row| StandingsRowSnapshot { team_name:     row.team.name,
                                                              rank:          row.rank,
                                                              played:        row.played,
                                                              won:           row.won,
                                                              drawn:         row.drawn,
                                                              lost:          row.lost,
                                                              goals_for:     row.goals_for,
                                                              goals_against: row.goals_against,
                                                              points:        row.points, })
                            .collect();

        let competition_name = self.competitions(plugin_id)
                                   .into_iter()
                                   .find(|competition| competition.id == competition_id)
                                   .map(|competition| competition.name)
                                   .unwrap_or_default();

        Some(StandingsSnapshot { competition_name,
                                 rows })
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
/// plugins and loads the enabled ones into a fresh [`PluginHost`]. Returns a
/// handle ready for `cx.set_global`; no standings fetch happens here
/// anymore — the league/competition selector fetches on demand once the
/// window is up (see `ui::views::root_view::RootView::new`).
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
