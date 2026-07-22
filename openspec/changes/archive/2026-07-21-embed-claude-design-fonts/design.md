## Context

`FontSelections` (`crates/fulltime-ui/src/data/theme.rs`) already sets `heading_font = "Sora"` and `body_font = "Manrope"`, matching the Claude Design mockup's Style A typography. GPUI resolves a font family name against whatever fonts the OS reports (`gpui_platform` is built with the `font-kit` feature, which does OS font matching), and the app never calls `App::text_system().add_fonts(...)` or ships any font bytes. Sora and Manrope are Google Fonts, not preinstalled on macOS/Windows/Linux, so on a machine without them installed GPUI silently substitutes a system sans-serif and the app doesn't visually match the mockup.

## Goals / Non-Goals

**Goals:**
- App renders with Sora (headings) and Manrope (body) regardless of what's installed on the host OS.
- Font loading happens once, at startup, before any window/view is created.

**Non-Goals:**
- No change to which fonts are used (still Sora/Manrope, still Style A only, per `design-tokens`'s "No Style B tokens" requirement).
- No dynamic/user-configurable font selection.
- No font subsetting or custom build pipeline — vendor the font files needed and no more.

## Decisions

- **Vendor the variable font files as-is, not statically-instanced per-weight files.** Google Fonts' upstream repo (`google/fonts`) only distributes Sora and Manrope as single variable-font files (`Sora[wght].ttf`, `Manrope[wght].ttf`) — no static per-weight `.ttf` files exist to vendor. Confirmed against the pinned GPUI revision (`1d217ee`, `crates/gpui_macos/src/text_system.rs`): `add_fonts` hands embedded bytes to CoreText via `CGFont::from_data_provider`, and GPUI's `Font` type already carries a `weight: FontWeight` field used at font-matching time — CoreText resolves the `wght` variation axis to the requested weight natively, so one variable-font file per family covers every weight the mockup uses (Sora 600/700/800, Manrope 400/500/600/700) without static instancing or extra tooling.
- **Embed via `include_bytes!` at compile time**, not a runtime `AssetSource` lookup. There's no existing `AssetSource` implementation in `fulltime-ui` (`gpui-component-assets` is a separate crate for component icons, not app fonts), and adding one is unnecessary indirection for a fixed, small set of font files. `include_bytes!` keeps the binary self-contained with no runtime file-not-found failure mode.
- **Register fonts in the same startup path as theme initialization** (`crates/fulltime-ui/src/util/init.rs`, called before the first window opens), so `FontSelections`'s family names are guaranteed resolvable the first time any view renders text.
- **License compliance**: vendor each font's `OFL.txt` alongside the `.ttf` files under `assets/fonts/<family>/`, matching Google Fonts' distribution requirement to keep the license with the font files.

## Risks / Trade-offs

- [Binary size grows by the embedded font bytes (roughly a few hundred KB across both families' weights)] → Acceptable for a desktop app; no mitigation needed.
- [`add_fonts` returns a `Result` if a font fails to parse] → Startup init treats this as a fatal, loud failure (return the error, don't swallow it) rather than silently falling back to a mismatched system font, since a silent fallback is the exact bug this change fixes.
- [Font family name collision with a system-installed Sora/Manrope of a different version] → GPUI's embedded fonts take precedence once registered via `add_fonts` in the same process, so this doesn't need special handling.

## Open Questions

None.
