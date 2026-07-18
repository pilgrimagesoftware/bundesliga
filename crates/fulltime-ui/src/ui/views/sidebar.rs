//! Sidebar view: primary navigation.
//!
//! Placeholder shell with no nav items yet — the League/Table/Matches/
//! Teams/Stats navigation (see the openspec proposals `bundesliga-sports-ui`
//! and `stats-view`) gets built here once there's data to navigate to.

use gpui::prelude::*;
use gpui::{App, div, px};

use crate::data::theme::ColorTokens;

/// Renders the sidebar panel.
pub fn render_sidebar(colors: &ColorTokens, _cx: &App) -> impl IntoElement + 'static + use<> {
    div().w(px(200.0))
         .flex_none()
         .h_full()
         .border_r_1()
         .border_color(colors.border)
         .bg(colors.surface_alt)
}
