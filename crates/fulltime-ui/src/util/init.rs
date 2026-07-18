use gpui::App;

use crate::data::theme::FullTimeTheme;
use crate::i18n;

/// Detects the system locale and activates it, then registers GPUI app-level
/// globals.
///
/// Always starts from [`FullTimeTheme::default_theme`] for now — persisted
/// theme/font preferences aren't wired up yet (no `UiPreferences` store
/// exists in this scaffold).
///
/// Must be called before any view renders.
pub fn init_globals(cx: &mut App) {
    i18n::init();
    cx.set_global(FullTimeTheme::default_theme());
}
