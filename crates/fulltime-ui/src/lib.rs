//! FullTime desktop application UI: the reusable GPUI application shell.
//!
//! This crate holds everything platform-agnostic — window/menu bootstrap,
//! theming, i18n, and the shell views (title bar, sidebar, status bar,
//! toolbar, root view). League/match/team data and the screens that render
//! it land in later crates/modules once the OpenLigaDB/TheSportsDB port
//! happens; this scaffold only opens a themed, empty window.

// Embed all YAML locale files from `crates/fulltime-ui/i18n/` at compile
// time. The `t!("module.key")` macro resolves to `crate::_rust_i18n_t(...)`,
// so this invocation must be at the crate root.
rust_i18n::i18n!("i18n", fallback = "en");

pub mod build_info;
pub mod data;
pub mod i18n;
pub mod ui {
    pub mod actions;
    pub mod app;
    pub mod widgets;
    pub mod views {
        pub mod root_view;
        pub mod sidebar;
        pub mod status_bar;
        pub mod title_bar;
        pub mod toolbar;
    }
}
pub mod util;
