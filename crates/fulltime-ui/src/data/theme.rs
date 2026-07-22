//! FullTime theme system: a light and a dark variant, matching the Style A
//! palette, typography, and radius tokens of the Claude Design mockup
//! ("Football Scores and tracking app") — canvas colors `#f0eee6` (light) /
//! `#2e2c26` (dark), with zone-highlight colors and form-indicator colors
//! extracted from the mockup's OKLCH formulas (`oklch(93%|28% <chroma>
//! <hue>)` for zone highlights). GPUI has no OKLCH constructor, so these are
//! pre-computed HSLA approximations rather than a runtime OKLCH conversion —
//! see `openspec/changes/ui-skeleton/design.md`.

use gpui::{Hsla, Pixels, SharedString, hsla, px};

const DEFAULT_BODY_FONT: &str = "Manrope";
const DEFAULT_HEADING_FONT: &str = "Sora";
const DEFAULT_UI_TEXT_SIZE: f32 = 14.0;

/// Semantic color tokens for one FullTime theme.
#[derive(Debug, Clone)]
pub struct ColorTokens {
    /// App desktop background color.
    pub desktop_bg:     Hsla,
    /// Main window / panel background.
    pub surface:        Hsla,
    /// Sidebar / secondary surface background.
    pub surface_alt:    Hsla,
    /// Hover state background.
    pub hover:          Hsla,
    /// Primary text.
    pub text_primary:   Hsla,
    /// Secondary / dimmed text.
    pub text_secondary: Hsla,
    /// Tertiary / placeholder text.
    pub text_tertiary:  Hsla,
    /// Default border / divider.
    pub border:         Hsla,
    /// Stronger border for inputs.
    pub border_strong:  Hsla,
    /// Accent (active nav, focus rings) — Bundesliga red.
    pub accent:         Hsla,
    /// Accent at low opacity for backgrounds.
    pub accent_soft:    Hsla,
    /// Text color drawn on top of an accent background.
    pub accent_on:      Hsla,
    /// Error / destructive state (red).
    pub error:          Hsla,
    /// Success state (green).
    pub success:        Hsla,
    /// Warning state (amber).
    pub warning:        Hsla,
    /// Informational state (blue).
    pub info:           Hsla,
}

/// Identifies one of the two named themes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeKey {
    #[default]
    Pitch,
    PitchNight,
}

impl ThemeKey {
    /// Stable, lowercase identifier used for persistence — independent of the
    /// enum variant name so a future rename doesn't silently invalidate saved
    /// preferences.
    pub fn as_str(self) -> &'static str {
        match self {
            ThemeKey::Pitch => "pitch",
            ThemeKey::PitchNight => "pitch_night",
        }
    }

    /// Resolves a persisted key back to a `ThemeKey`, or `None` if it doesn't
    /// match any known theme.
    pub fn from_persisted_key(key: &str) -> Option<Self> {
        match key {
            "pitch" => Some(ThemeKey::Pitch),
            "pitch_night" => Some(ThemeKey::PitchNight),
            _ => None,
        }
    }
}

/// Font-family and size selections for a [`FullTimeTheme`].
///
/// Style A only (per `openspec/changes/ui-skeleton/proposal.md`): heading
/// font `Sora`, body font `Manrope`. There is no Style B variant or runtime
/// style toggle.
#[derive(Debug, Clone)]
pub struct FontSelections {
    pub heading_font: SharedString,
    pub body_font:    SharedString,
    pub ui_text_size: Pixels,
}

impl Default for FontSelections {
    fn default() -> Self {
        Self { heading_font: SharedString::from(DEFAULT_HEADING_FONT),
               body_font:    SharedString::from(DEFAULT_BODY_FONT),
               ui_text_size: px(DEFAULT_UI_TEXT_SIZE), }
    }
}

/// Style A type scale, matching the mockup's named text sizes.
#[derive(Debug, Clone)]
pub struct TypeScale {
    /// Header brand wordmark.
    pub brand:         Pixels,
    /// Hero band title (Standings/History hero).
    pub hero_title:    Pixels,
    /// Section/card title.
    pub section_title: Pixels,
    /// Match score display.
    pub score:         Pixels,
    /// Default body text.
    pub body:          Pixels,
}

impl Default for TypeScale {
    fn default() -> Self {
        Self { brand:         px(19.0),
               hero_title:    px(34.0),
               section_title: px(14.5),
               score:         px(44.0),
               body:          px(13.0), }
    }
}

/// Style A radius tokens (base `16px`, matching the mockup's rounded/pill
/// variant), with derived smaller/larger variants for compact controls and
/// large cards.
#[derive(Debug, Clone)]
pub struct RadiusTokens {
    pub sm:   Pixels,
    pub base: Pixels,
    pub lg:   Pixels,
}

impl Default for RadiusTokens {
    fn default() -> Self {
        Self { sm:   px(8.0),
               base: px(16.0),
               lg:   px(24.0), }
    }
}

/// GPUI app-level global containing the active FullTime theme, font
/// selections, type scale, and radius tokens.
#[derive(Debug, Clone)]
pub struct FullTimeTheme {
    pub key:        ThemeKey,
    pub colors:     ColorTokens,
    pub fonts:      FontSelections,
    pub type_scale: TypeScale,
    pub radius:     RadiusTokens,
}

impl gpui::Global for FullTimeTheme {}

impl FullTimeTheme {
    /// Constructs the theme for `key` and `fonts`.
    pub fn new(key: ThemeKey, fonts: FontSelections) -> Self {
        let colors = match key {
            ThemeKey::Pitch => pitch_colors(),
            ThemeKey::PitchNight => pitch_night_colors(),
        };
        Self { key,
               colors,
               fonts,
               type_scale: TypeScale::default(),
               radius: RadiusTokens::default() }
    }

    /// Returns the default theme (Pitch, default fonts).
    pub fn default_theme() -> Self {
        Self::new(ThemeKey::Pitch, FontSelections::default())
    }
}

/// Standings qualification-zone highlight colors, matching the mockup's
/// zone hues (UCL 150, UEL 60, relegation 25) at light-mode (93% lightness)
/// or dark-mode (28% lightness) backgrounds.
#[derive(Debug, Clone)]
pub struct ZoneColors {
    pub ucl:        Hsla,
    pub uel:        Hsla,
    pub relegation: Hsla,
}

/// Resolves zone-highlight colors for `key` (light or dark).
pub fn zone_colors(key: ThemeKey) -> ZoneColors {
    let l = match key {
        ThemeKey::Pitch => 0.93,
        ThemeKey::PitchNight => 0.28,
    };
    ZoneColors { ucl:        hsla(150.0 / 360.0, 0.45, l, 1.0),
                 uel:        hsla(60.0 / 360.0, 0.45, l, 1.0),
                 relegation: hsla(25.0 / 360.0, 0.45, l, 1.0), }
}

/// Form-indicator (win/draw/loss) colors, matching the mockup's form-dot
/// palette. Stable across light/dark themes.
#[derive(Debug, Clone)]
pub struct FormColors {
    pub win:  Hsla,
    pub draw: Hsla,
    pub loss: Hsla,
}

/// Returns the win/draw/loss form-indicator colors.
pub fn form_colors() -> FormColors {
    FormColors { win:  hsla(150.0 / 360.0, 0.45, 0.42, 1.0),
                 draw: hsla(0.0, 0.0, 0.60, 1.0),
                 loss: hsla(25.0 / 360.0, 0.65, 0.50, 1.0), }
}

/// Overrides `gpui_component::Theme`'s semantic colors to match `colors`.
///
/// `gpui-component` widgets (`Button`, `Input`, `Popover`/`PopupMenu`,
/// tooltips, scrollbars, `Sidebar`, `TabBar`/`Tab`, `StatusBar`, `TitleBar`,
/// etc.) read their colors from `cx.theme()` (`gpui_component::Theme`),
/// which is a separate global from [`FullTimeTheme`] and is never otherwise
/// synced with the active FullTime palette. Call this whenever
/// [`FullTimeTheme`] changes.
pub fn apply_theme_colors(theme: &mut gpui_component::Theme, colors: &ColorTokens) {
    theme.colors.background = colors.surface;
    theme.colors.foreground = colors.text_primary;
    theme.colors.border = colors.border;
    theme.colors.muted = colors.surface_alt;
    theme.colors.muted_foreground = colors.text_tertiary;
    theme.colors.ring = colors.accent;
    theme.colors.selection = colors.accent_soft;
    theme.colors.caret = colors.text_primary;

    theme.tokens.background = colors.surface.into();
    theme.tokens.foreground = colors.text_primary.into();
    theme.tokens.border = colors.border.into();
    theme.tokens.muted = colors.surface_alt.into();
    theme.tokens.muted_foreground = colors.text_tertiary.into();
    theme.tokens.ring = colors.accent.into();
    theme.tokens.selection = colors.accent_soft.into();
    theme.tokens.caret = colors.text_primary.into();

    theme.colors.primary = colors.accent;
    theme.colors.primary_active = colors.accent_soft;
    theme.colors.primary_foreground = colors.accent_on;
    theme.colors.primary_hover = colors.accent;
    theme.colors.secondary = colors.surface_alt;
    theme.colors.secondary_active = colors.accent_soft;
    theme.colors.secondary_foreground = colors.text_secondary;
    theme.colors.secondary_hover = colors.hover;
    theme.colors.danger = colors.error;
    theme.colors.danger_active = colors.error;
    theme.colors.danger_foreground = colors.accent_on;
    theme.colors.danger_hover = colors.error;
    theme.colors.success = colors.success;
    theme.colors.success_active = colors.success;
    theme.colors.success_foreground = colors.accent_on;
    theme.colors.success_hover = colors.success;
    theme.colors.warning = colors.warning;
    theme.colors.warning_active = colors.warning;
    theme.colors.warning_foreground = colors.accent_on;
    theme.colors.warning_hover = colors.warning;
    theme.colors.info = colors.info;
    theme.colors.info_active = colors.info;
    theme.colors.info_foreground = colors.accent_on;
    theme.colors.info_hover = colors.info;

    theme.tokens.primary = colors.accent.into();
    theme.tokens.primary_active = colors.accent_soft.into();
    theme.tokens.primary_foreground = colors.accent_on.into();
    theme.tokens.primary_hover = colors.accent.into();
    theme.tokens.secondary = colors.surface_alt.into();
    theme.tokens.secondary_active = colors.accent_soft.into();
    theme.tokens.secondary_foreground = colors.text_secondary.into();
    theme.tokens.secondary_hover = colors.hover.into();
    theme.tokens.danger = colors.error.into();
    theme.tokens.danger_active = colors.error.into();
    theme.tokens.danger_foreground = colors.accent_on.into();
    theme.tokens.danger_hover = colors.error.into();
    theme.tokens.success = colors.success.into();
    theme.tokens.success_active = colors.success.into();
    theme.tokens.success_foreground = colors.accent_on.into();
    theme.tokens.success_hover = colors.success.into();
    theme.tokens.warning = colors.warning.into();
    theme.tokens.warning_active = colors.warning.into();
    theme.tokens.warning_foreground = colors.accent_on.into();
    theme.tokens.warning_hover = colors.warning.into();
    theme.tokens.info = colors.info.into();
    theme.tokens.info_active = colors.info.into();
    theme.tokens.info_foreground = colors.accent_on.into();
    theme.tokens.info_hover = colors.info.into();

    theme.colors.button = colors.surface_alt;
    theme.colors.button_hover = colors.hover;
    theme.colors.button_active = colors.accent_soft;
    theme.colors.button_foreground = colors.text_primary;
    theme.colors.button_primary = colors.accent;
    theme.colors.button_primary_active = colors.accent_soft;
    theme.colors.button_primary_foreground = colors.accent_on;
    theme.colors.button_primary_hover = colors.accent;

    theme.tokens.button = colors.surface_alt.into();
    theme.tokens.button_hover = colors.hover.into();
    theme.tokens.button_active = colors.accent_soft.into();
    theme.tokens.button_foreground = colors.text_primary.into();
    theme.tokens.button_primary = colors.accent.into();
    theme.tokens.button_primary_active = colors.accent_soft.into();
    theme.tokens.button_primary_foreground = colors.accent_on.into();
    theme.tokens.button_primary_hover = colors.accent.into();

    theme.colors.input = colors.border;
    theme.tokens.input = colors.border.into();

    theme.colors.popover = colors.surface;
    theme.colors.popover_foreground = colors.text_primary;
    theme.tokens.popover = colors.surface.into();
    theme.tokens.popover_foreground = colors.text_primary.into();

    theme.colors.scrollbar = colors.surface_alt;
    theme.colors.scrollbar_thumb = colors.border_strong;
    theme.tokens.scrollbar = colors.surface_alt.into();
    theme.tokens.scrollbar_thumb = colors.border_strong.into();

    theme.colors.sidebar = colors.surface_alt;
    theme.colors.sidebar_accent = colors.accent_soft;
    theme.colors.sidebar_accent_foreground = colors.text_secondary;
    theme.colors.sidebar_border = colors.border;
    theme.colors.sidebar_foreground = colors.text_primary;
    theme.colors.sidebar_primary = colors.accent;
    theme.colors.sidebar_primary_foreground = colors.accent_on;
    theme.tokens.sidebar = colors.surface_alt.into();
    theme.tokens.sidebar_accent = colors.accent_soft.into();
    theme.tokens.sidebar_accent_foreground = colors.text_secondary.into();
    theme.tokens.sidebar_border = colors.border.into();
    theme.tokens.sidebar_foreground = colors.text_primary.into();
    theme.tokens.sidebar_primary = colors.accent.into();
    theme.tokens.sidebar_primary_foreground = colors.accent_on.into();

    theme.colors.title_bar = colors.surface;
    theme.colors.title_bar_border = colors.border;
    theme.colors.status_bar = colors.surface_alt;
    theme.colors.status_bar_border = colors.border;

    theme.tokens.title_bar = colors.surface.into();
    theme.tokens.title_bar_border = colors.border.into();
    theme.tokens.status_bar = colors.surface_alt.into();
    theme.tokens.status_bar_border = colors.border.into();
}

fn hex(r: u8, g: u8, b: u8) -> Hsla {
    let n = (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b);
    gpui::rgb(n).into()
}

fn hex_a(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    let n = (u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b);
    let base: Hsla = gpui::rgb(n).into();
    Hsla { a, ..base }
}

// ── Pitch (light) ──────────────────────────────────────────────────────────
// desktop_bg matches the Claude Design mockup's light canvas (#f0eee6).
fn pitch_colors() -> ColorTokens {
    ColorTokens { desktop_bg:     hex(0xF0, 0xEE, 0xE6),
                  surface:        hex(0xFC, 0xFB, 0xF8),
                  surface_alt:    hex(0xEE, 0xEC, 0xE3),
                  hover:          hex(0xE6, 0xE3, 0xD8),
                  text_primary:   hex(0x20, 0x1F, 0x1A),
                  text_secondary: hex(0x55, 0x53, 0x49),
                  text_tertiary:  hex(0x88, 0x85, 0x78),
                  border:         hex(0xE1, 0xDE, 0xD2),
                  border_strong:  hex(0xD3, 0xCF, 0xBF),
                  accent:         hex(0xD2, 0x05, 0x15),
                  accent_soft:    hex_a(0xD2, 0x05, 0x15, 0.13),
                  accent_on:      hex(0xFC, 0xFB, 0xF8),
                  error:          hex(0xB0, 0x30, 0x28),
                  success:        hex(0x04, 0x78, 0x57),
                  warning:        hex(0xB4, 0x53, 0x09),
                  info:           hex(0x03, 0x69, 0xA1), }
}

// ── Pitch Night (dark) ─────────────────────────────────────────────────────
// desktop_bg matches the Claude Design mockup's dark canvas (#2e2c26).
fn pitch_night_colors() -> ColorTokens {
    ColorTokens { desktop_bg:     hex(0x2E, 0x2C, 0x26),
                  surface:        hex(0x22, 0x21, 0x1C),
                  surface_alt:    hex(0x2A, 0x28, 0x22),
                  hover:          hex(0x34, 0x32, 0x2A),
                  text_primary:   hex(0xEC, 0xEA, 0xE1),
                  text_secondary: hex(0xB2, 0xAF, 0xA1),
                  text_tertiary:  hex(0x85, 0x82, 0x74),
                  border:         hex(0x3A, 0x38, 0x2F),
                  border_strong:  hex(0x46, 0x43, 0x39),
                  accent:         hex(0xE8, 0x3A, 0x47),
                  accent_soft:    hex_a(0xE8, 0x3A, 0x47, 0.16),
                  accent_on:      hex(0x22, 0x21, 0x1C),
                  error:          hex(0xE0, 0x58, 0x58),
                  success:        hex(0x4A, 0xDE, 0x80),
                  warning:        hex(0xFB, 0xBF, 0x24),
                  info:           hex(0x38, 0xBD, 0xF8), }
}
