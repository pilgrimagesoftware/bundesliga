//! Bridges `fulltime-core`'s [`PluginHost`]/[`PluginRegistry`] into
//! `fulltime-ui`'s [`PluginManager`] trait, so the Plugins screen can list
//! and toggle real plugins without `fulltime-ui` depending on `wasmtime` or
//! `fulltime-plugin-api` — see that trait's own doc comment for why.

use std::path::PathBuf;

use fulltime_ui::ui::plugin_manager::{
    PluginManager, PluginManagerHandle, PluginSummary, StandingsRowSnapshot, StandingsSnapshot,
};

use crate::plugin_host::PluginHost;
use crate::plugin_host::registry::PluginRegistry;

/// The plugin id this pass fetches standings for. Picking a season/
/// competition automatically (rather than exposing a picker) is
/// deliberately minimal for the first real data flow — see
/// `openspec/changes/plugin-host-runtime` task 5.1.
const STANDINGS_PLUGIN_ID: &str = "bundesliga";

struct FulltimePluginManager {
    host:      PluginHost,
    registry:  PluginRegistry,
    standings: Option<StandingsSnapshot>,
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

    fn standings(&self) -> Option<StandingsSnapshot> {
        self.standings.clone()
    }
}

/// Fetches the Bundesliga plugin's current-season standings, or `None` if
/// it isn't loaded or the fetch fails for any reason (logged, not
/// propagated — this is best-effort data for a screen that already has a
/// mockup fallback).
///
/// Picks the highest-numbered `bl1-<season>` competition id (`Plugins/
/// Bundesliga`'s `mapping::map_competition` issues one per season), since
/// `list_competitions` returns every season OpenLigaDB has, not just the
/// current one.
fn fetch_bundesliga_standings(host: &mut PluginHost) -> Option<StandingsSnapshot> {
    if !host.is_loaded(STANDINGS_PLUGIN_ID) {
        return None;
    }

    let competitions = match host.list_competitions(STANDINGS_PLUGIN_ID) {
        Ok(competitions) => competitions,
        Err(error) => {
            tracing::warn!(%error, "failed to list Bundesliga competitions");
            return None;
        }
    };

    let latest = competitions.iter().max_by_key(|competition| {
                                         competition.id
                                                    .rsplit('-')
                                                    .next()
                                                    .and_then(|season| season.parse::<u32>().ok())
                                                    .unwrap_or(0)
                                     })?;

    let standings = match host.fetch_standings(STANDINGS_PLUGIN_ID, &latest.id) {
        Ok(standings) => standings,
        Err(error) => {
            tracing::warn!(competition_id = %latest.id, %error, "failed to fetch Bundesliga standings");
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

    Some(StandingsSnapshot { competition_name: latest.name.clone(),
                             rows })
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

    let standings = fetch_bundesliga_standings(&mut host);

    Ok(PluginManagerHandle(Box::new(FulltimePluginManager { host,
                                                            registry,
                                                            standings })))
}
