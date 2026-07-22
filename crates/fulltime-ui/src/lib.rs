//! FullTime desktop application UI: the reusable GPUI application shell.
//!
//! This crate holds everything platform-agnostic — window/menu bootstrap,
//! theming, i18n, and the shell views (persistent header, status bar,
//! and the app-screen views). League/match/team data lands in later
//! crates/modules once the OpenLigaDB/TheSportsDB port happens, so those
//! screens render the Claude Design mockup's empty-state layouts with no
//! live data wired in yet. The Plugins screen is the exception: it's backed
//! by real data via the [`ui::plugin_manager::PluginManager`] trait, which
//! `fulltime-core` implements against its `PluginHost`/`PluginRegistry`.

// Embed all YAML locale files from `crates/fulltime-ui/i18n/` at compile
// time. The `t!("module.key")` macro resolves to `crate::_rust_i18n_t(...)`,
// so this invocation must be at the crate root.
rust_i18n::i18n!("i18n", fallback = "en");

pub mod build_info;
pub mod data;
pub mod i18n;
pub mod ui {
    pub mod actions;
    pub mod activity;
    pub mod app;
    pub mod app_state;
    pub mod plugin_manager;
    pub mod widgets;
    pub mod views {
        pub mod components {
            pub mod activity_panel;
            pub mod alert_history_panel;
            pub mod back_button;
            pub mod form_dots;
            pub mod hero;
            pub mod league_selector;
            pub mod legend;
            pub mod stat_grid;
            pub mod status_pill;
            pub mod tab_bar;
        }
        pub mod header;
        pub mod history;
        pub mod match_view;
        pub mod player;
        pub mod plugins;
        pub mod root_view;
        pub mod standings;
        pub mod status_bar;
        pub mod team;
    }
}
pub mod util;
