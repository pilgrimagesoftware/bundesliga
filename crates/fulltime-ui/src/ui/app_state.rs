//! App-level UI state shared across the header and content-area views:
//! which of the five screens is active, which league is selected, and
//! (for the Match screen) which detail tab is active.

use rust_i18n::t;

/// The five app views switched via the header's screen navigation control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Standings,
    Match,
    History,
    Player,
    Team,
}

impl AppScreen {
    pub const ALL: [AppScreen; 5] = [AppScreen::Standings,
                                     AppScreen::Match,
                                     AppScreen::History,
                                     AppScreen::Player,
                                     AppScreen::Team];

    /// Display label for the header's screen navigation control.
    pub fn label(self) -> String {
        let key = match self {
            AppScreen::Standings => "screen.standings",
            AppScreen::Match => "screen.match",
            AppScreen::History => "screen.history",
            AppScreen::Player => "screen.player",
            AppScreen::Team => "screen.team",
        };
        t!(key).to_string()
    }
}

/// The three tabs on the Match screen's score header.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchTab {
    Summary,
    Lineups,
    Stats,
}

impl MatchTab {
    pub const ALL: [MatchTab; 3] = [MatchTab::Summary, MatchTab::Lineups, MatchTab::Stats];

    pub fn label(self) -> String {
        let key = match self {
            MatchTab::Summary => "match_tab.summary",
            MatchTab::Lineups => "match_tab.lineups",
            MatchTab::Stats => "match_tab.stats",
        };
        t!(key).to_string()
    }
}
