//! Root view: composes the title bar, sidebar, toolbar, and status bar
//! around an empty main-content placeholder.
//!
//! This is scaffolding only — no league/match/team data or screens exist
//! yet. It exists so the app shell is visibly running with the right theme
//! and chrome.

use gpui::prelude::*;
use gpui::{Context, Render, Window, div, px};

use crate::data::theme::FullTimeTheme;
use crate::ui::views::sidebar::render_sidebar;
use crate::ui::views::status_bar::render_status_bar;
use crate::ui::views::title_bar::render_title_bar;
use crate::ui::views::toolbar::render_toolbar;

/// Top-level GPUI view for the FullTime main window.
pub struct RootView;

impl RootView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.global::<FullTimeTheme>().colors.clone();

        div().flex()
             .flex_col()
             .size_full()
             .bg(colors.desktop_bg)
             .child(render_title_bar(&colors, cx))
             .child(div().flex()
                         .flex_1()
                         .min_h(px(0.0))
                         .child(render_sidebar(&colors, cx))
                         .child(div().flex()
                                     .flex_col()
                                     .flex_1()
                                     .min_w(px(0.0))
                                     .bg(colors.surface)
                                     .child(render_toolbar(&colors, cx))))
             .child(render_status_bar(&colors, cx))
    }
}
