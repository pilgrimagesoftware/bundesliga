//! Bridges `fulltime-core`'s [`PluginHost`]/[`PluginRegistry`] into
//! `fulltime-ui`'s [`PluginManager`] trait, so the Plugins screen can list
//! and toggle real plugins without `fulltime-ui` depending on `wasmtime` or
//! `fulltime-plugin-api` — see that trait's own doc comment for why.

use std::collections::HashSet;
use std::path::PathBuf;

use fulltime_ui::ui::activity::{ActivityControllerHandle, Status};
use fulltime_ui::ui::plugin_manager::{
    CompetitionSummary, LeagueSummary, PluginManager, PluginManagerHandle, PluginSummary,
    StandingsRowSnapshot, StandingsSnapshot,
};
use gpui::App;

use crate::plugin_host::PluginHost;
use crate::plugin_host::registry::PluginRegistry;

/// Records `label`/`status` into the activity log, if the controller is
/// installed (see `fulltime_ui::ui::activity::install`, called before
/// [`build`] in `app::run`). A no-op otherwise, so this never panics if the
/// activity controller isn't wired up (e.g. in a future headless build).
fn record_activity(cx: &mut App, label: impl Into<String>, status: Status) {
    let Some(entity) = cx.try_global::<ActivityControllerHandle>()
                         .map(|handle| handle.0.clone())
    else {
        return;
    };
    entity.update(cx, |controller, cx| controller.record(label, status, cx));
}

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

    fn set_enabled(&mut self, id: &str, enabled: bool, cx: &mut App) {
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

        let action = if enabled { "enable" } else { "disable" };
        match result {
            Ok(()) => record_activity(cx, format!("Plugin {id} {action}d"), Status::Complete),
            Err(error) => {
                tracing::warn!(plugin_id = id, %error, "failed to toggle plugin enabled state");
                record_activity(cx,
                                format!("Plugin {id} failed to {action}"),
                                Status::Failed(error));
            }
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
/// Every *discovered* plugin's load outcome (success or failure) is also
/// recorded into the activity log — see `fulltime_ui::ui::activity::install`,
/// which callers must have already run on `cx` before calling this, so the
/// entries have somewhere to land. A plugin that fails discovery itself
/// (e.g. an invalid manifest) never reaches this function's loop and so has
/// no activity entry, only the registry's own `tracing::warn!`.
///
/// # Errors
/// Returns an error only if the [`PluginHost`] or its state file can't be
/// initialized at all — never for an individual plugin failing to load.
pub fn build(cx: &mut App) -> anyhow::Result<PluginManagerHandle> {
    let data_dir = app_data_dir();

    let mut registry = PluginRegistry::new(data_dir.join("plugin_state.json"))?;
    registry.discover_bundled();
    registry.discover_user_installed(&data_dir.join("plugins"));

    let mut host = PluginHost::new()?;
    let failures = registry.load_enabled(&mut host);
    let failed_ids: HashSet<&str> = failures.iter().map(|(id, _)| id.as_str()).collect();

    for (plugin_id, error) in &failures {
        tracing::warn!(%plugin_id, %error, "failed to load plugin at startup");
        record_activity(cx,
                        format!("Plugin {plugin_id} failed to load"),
                        Status::Failed(error.to_string()));
    }
    for (plugin_id, _) in
        registry.discovered()
                .filter(|(id, _)| registry.is_enabled(id) && !failed_ids.contains(id))
    {
        record_activity(cx, format!("Plugin {plugin_id} loaded"), Status::Complete);
    }

    Ok(PluginManagerHandle(Box::new(FulltimePluginManager { host, registry })))
}
