## Why

`FontSelections` already names `Sora` and `Manrope` as the heading/body font families (per `design-tokens`), but no font files ship with the app and nothing registers them with GPUI's text system. On any machine that doesn't happen to have Sora/Manrope installed as system fonts, GPUI silently falls back to a generic sans-serif, so the app doesn't actually render with the Claude Design mockup's typography.

## What Changes

- Vendor the Sora and Manrope static font files (OFL-licensed, from Google Fonts) into the app as embedded assets.
- Register the embedded font files with GPUI's text system during app startup, before the first window is created.
- Keep the existing `FontSelections` values (`Sora` heading, `Manrope` body) as-is — only the font-loading mechanism changes, not the font choice.

## Capabilities

### New Capabilities

(none)

### Modified Capabilities

- `design-tokens`: adds a requirement that the Sora and Manrope font family tokens resolve to embedded font files loaded into GPUI's text system, rather than relying on the font being present on the host OS.

## Impact

- `crates/fulltime-ui/assets/fonts/`: new vendored `.ttf`/`.otf` files for Sora and Manrope, plus their OFL license text.
- `crates/fulltime-ui/src/util/init.rs` (or app bootstrap): loads and registers the embedded font bytes with GPUI's text system on startup.
- `Cargo.toml` / build: no new dependencies expected — GPUI's `App::text_system().add_fonts(...)` takes raw font bytes directly.
