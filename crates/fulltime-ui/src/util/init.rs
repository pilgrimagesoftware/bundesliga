use gpui::App;

use crate::data::theme::FullTimeTheme;
use crate::i18n;

/// Sora and Manrope, vendored as their upstream variable-font files — see
/// `openspec/changes/embed-claude-design-fonts/design.md` for why no static
/// per-weight files are used.
const SORA_VARIABLE: &[u8] = include_bytes!("../../assets/fonts/sora/Sora-Variable.ttf");
const MANROPE_VARIABLE: &[u8] = include_bytes!("../../assets/fonts/manrope/Manrope-Variable.ttf");

/// Detects the system locale and activates it, then registers GPUI app-level
/// globals.
///
/// Always starts from [`FullTimeTheme::default_theme`] for now — persisted
/// theme/font preferences aren't wired up yet (no `UiPreferences` store
/// exists in this scaffold).
///
/// Must be called before any view renders.
///
/// # Panics
///
/// Panics if the embedded Sora/Manrope font bytes fail to register with
/// GPUI's text system. This is a fatal startup error rather than a silent
/// fallback to a mismatched system font, since that fallback is the exact
/// bug this embedding fixes.
#[allow(clippy::expect_used)]
pub fn init_globals(cx: &mut App) {
    i18n::init();
    cx.text_system()
      .add_fonts(vec![SORA_VARIABLE.into(), MANROPE_VARIABLE.into()])
      .expect("failed to register embedded Sora/Manrope fonts");
    cx.set_global(FullTimeTheme::default_theme());
}
