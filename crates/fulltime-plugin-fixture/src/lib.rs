//! Minimal `fulltime-plugin-api` data-provider component, used only to
//! validate `fulltime-core`'s plugin host runtime end-to-end (see
//! `openspec/changes/plugin-host-runtime/tasks.md`, task 2.5) before any real
//! plugin (`Plugins/Bundesliga`) exists to test against.
//!
//! Every operation returns the same canned data, keyed off `competition_id`
//! for the two operations that take one:
//!
//! - `"panic"` deliberately panics, to exercise the host's fault isolation.
//! - `"undeclared-host"` calls the host `fetch` capability against a hostname
//!   this plugin's manifest does *not* declare, to exercise the host's
//!   network-capability scoping.
//! - Anything else returns the fixed [`Competition`] below.

use fulltime_plugin_api::{
    Competition, Fixture, FixtureStatus, ProviderError, Standings, StandingsGroup, StandingsRow,
    Team,
};

fn fixture_team() -> Team {
    Team { id:         "fixture-team".to_owned(),
           name:       "Fixture United".to_owned(),
           short_name: "FIX".to_owned(), }
}

fn fixture_competition() -> Competition {
    Competition { id:   "fixture-competition".to_owned(),
                  name: "Fixture League".to_owned(), }
}

fn fixture_fixture() -> Fixture {
    Fixture { id:             "fixture-match".to_owned(),
              competition_id: fixture_competition().id,
              group:          None,
              kickoff:        "2026-01-01T15:00:00Z".to_owned(),
              home_team:      fixture_team(),
              away_team:      fixture_team(),
              venue:          None,
              status:         FixtureStatus::Scheduled,
              score:          None, }
}

fn fixture_standings() -> Standings {
    Standings { competition_id: fixture_competition().id,
                groups:         vec![StandingsGroup { name: None,
                                                      rows: vec![StandingsRow { team:
                                                                                    fixture_team(),
                                                                                rank:          1,
                                                                                played:        0,
                                                                                won:           0,
                                                                                drawn:         0,
                                                                                lost:          0,
                                                                                goals_for:     0,
                                                                                goals_against: 0,
                                                                                points:        0, }], }], }
}

/// Triggers the deliberate-panic path used to exercise fault isolation.
const PANIC_TRIGGER: &str = "panic";
/// Triggers a `host.fetch` call against a hostname this plugin's manifest
/// does not declare, used to exercise network-capability scoping.
const UNDECLARED_HOST_TRIGGER: &str = "undeclared-host";

struct FixturePlugin;

impl fulltime_plugin_api::Guest for FixturePlugin {
    fn list_competitions() -> Result<Vec<Competition>, ProviderError> {
        Ok(vec![fixture_competition()])
    }

    fn fetch_fixtures(competition_id: String) -> Result<Vec<Fixture>, ProviderError> {
        if competition_id == PANIC_TRIGGER {
            panic!("fixture plugin: deliberate panic for fault-isolation testing");
        }
        Ok(vec![fixture_fixture()])
    }

    fn fetch_results(competition_id: String) -> Result<Vec<Fixture>, ProviderError> {
        if competition_id == PANIC_TRIGGER {
            panic!("fixture plugin: deliberate panic for fault-isolation testing");
        }
        Ok(vec![fixture_fixture()])
    }

    fn fetch_standings(competition_id: String) -> Result<Standings, ProviderError> {
        if competition_id == PANIC_TRIGGER {
            panic!("fixture plugin: deliberate panic for fault-isolation testing");
        }
        Ok(fixture_standings())
    }

    fn fetch_metadata(competition_id: String) -> Result<Competition, ProviderError> {
        if competition_id == PANIC_TRIGGER {
            panic!("fixture plugin: deliberate panic for fault-isolation testing");
        }
        if competition_id == UNDECLARED_HOST_TRIGGER {
            return fulltime_plugin_api::host_fetch("https://not-declared.invalid/ping")
                .map(|_| fixture_competition())
                .map_err(ProviderError::NetworkFailure);
        }
        Ok(fixture_competition())
    }
}

fulltime_plugin_api::export!(FixturePlugin with_types_in fulltime_plugin_api);
