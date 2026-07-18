//! Application shell modules for the Rust desktop frontend.

mod assets;

use fulltime_ui::ui::app::setup;
use gpui::*;

/// Boots the GPUI application shell. No service factories are wired up
/// yet — this is the scaffolding pass only; league/match/team data fetching
/// gets wired in once the OpenLigaDB/TheSportsDB port lands.
pub fn run() {
    gpui_platform::application().with_assets(assets::Assets::new("assets/icons"))
                                .with_quit_mode(QuitMode::LastWindowClosed)
                                .run(|cx| {
                                    setup(cx);
                                });
}
