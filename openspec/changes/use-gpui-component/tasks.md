## 1. Theme bridge fix (prerequisite for the Tag migration)

- [ ] 1.1 Grep every current `gpui-component` widget usage (`Combobox`, `TabBar`,
      `gpui_component::badge::Badge`, `Popover`, `Button`, `StatusBar`) for reliance on
      `theme.colors.success`/`.warning`/`.info` to confirm none currently depend on
      `gpui-component`'s built-in defaults for those fields
- [ ] 1.2 Extend `data::theme::apply_theme_colors` to map `success`, `warning`, and `info`
      (plus their `_foreground`/`_active`/`_hover` counterparts) from `ColorTokens`, following
      the existing `primary`/`secondary`/`danger` pattern
- [ ] 1.3 Manually verify (run the app, switch themes) that a `Tag` rendered with each semantic
      variant reflects the active `FullTimeTheme` palette

## 2. Migrate badge.rs to Avatar

- [ ] 2.1 Replace `render_badge` call sites in `match_view.rs`, `team.rs`, `standings.rs`, and
      `player.rs` with `gpui_component::avatar::Avatar`
- [ ] 2.2 Visually compare each of the four screens against pre-migration screenshots
- [ ] 2.3 Delete `views/components/badge.rs` and its module declaration

## 3. Migrate card.rs to GroupBox

- [ ] 3.1 Replace `render_card` call sites in `standings.rs` with `gpui_component::group_box::GroupBox`,
      overriding padding/gap via `StyleRefinement` if the defaults don't match `px(16.0)`/`px(12.0)`
- [ ] 3.2 Visually compare `standings.rs` against its pre-migration screenshot
- [ ] 3.3 Replace `render_card` call sites in `plugins.rs` with `GroupBox`
- [ ] 3.4 Visually compare `plugins.rs` against its pre-migration screenshot
- [ ] 3.5 Delete `views/components/card.rs` and its module declaration

## 4. Migrate back_button.rs to Button

- [ ] 4.1 Replace `render_back_button` call sites in `team.rs`, `match_view.rs`, `history.rs`,
      and `player.rs` with a ghost/text-variant `gpui_component::button::Button` plus a leading
      arrow `Icon`
- [ ] 4.2 Visually compare each of the four screens against pre-migration screenshots
- [ ] 4.3 Delete `views/components/back_button.rs` and its module declaration

## 5. Migrate status_pill.rs to Tag

- [ ] 5.1 Replace `render_status_pill`/`MatchStatus` call sites in `match_view.rs` and
      `standings.rs` with `gpui_component::tag::Tag`, mapping `Live`/`FullTime`/`Scheduled` to
      the appropriate `TagVariant`
- [ ] 5.2 Wrap the `Live`-state `Tag` in the existing `AnimationExt` opacity-pulse loop so the
      animation is preserved
- [ ] 5.3 Visually compare `match_view.rs` and `standings.rs` against pre-migration screenshots,
      including the `Live` pulse animation
- [ ] 5.4 Delete `views/components/status_pill.rs` and its module declaration

## 6. Record what stays custom

- [ ] 6.1 Add a short doc comment to `form_dots.rs`, `legend.rs`, `stat_grid.rs`, `hero.rs`, and
      `widgets.rs` noting that they were evaluated against `gpui-component` during this change
      and kept because no equivalent widget exists, so a future audit doesn't re-litigate them

## 7. Verification

- [ ] 7.1 `cargo build` and `cargo clippy --all-targets --all-features -- -D warnings` pass
- [ ] 7.2 `cargo test --workspace` passes
- [ ] 7.3 Full manual pass through every migrated screen (Standings, Match, Team, Player,
      History, Plugins) in both light and dark `FullTimeTheme` variants
