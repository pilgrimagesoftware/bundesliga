//! Tab bar component: a pill-style tab row wrapping `gpui_component::Tab`/
//! `TabBar`, reused by the header's league and screen navigation and by the
//! Match screen's Summary/Lineups/Stats tabs.

use gpui::prelude::*;
use gpui::{App, ElementId, Window};
use gpui_component::tab::{Tab, TabBar};

/// Renders a pill-style tab bar. `on_click` receives the clicked tab's
/// index; callers translate that back into their own enum/state.
pub fn render_tab_bar(id: impl Into<ElementId>, labels: Vec<String>, selected_index: usize,
                      on_click: impl Fn(&usize, &mut Window, &mut App) + 'static)
                      -> impl IntoElement {
    let mut bar = TabBar::new(id).pill()
                                 .selected_index(selected_index)
                                 .on_click(on_click);
    for label in labels {
        bar = bar.child(Tab::new().child(label));
    }
    bar
}
