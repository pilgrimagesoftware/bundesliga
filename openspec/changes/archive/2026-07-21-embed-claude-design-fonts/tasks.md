## 1. Vendor font files

- [x] 1.1 Download the Sora and Manrope variable font files (`Sora[wght].ttf`, `Manrope[wght].ttf`) from the `google/fonts` upstream repo (no static per-weight files exist for either family).
- [x] 1.2 Add them under `crates/fulltime-ui/assets/fonts/sora/` and `crates/fulltime-ui/assets/fonts/manrope/`, each alongside that font's `OFL.txt`.

## 2. Register fonts at startup

- [x] 2.1 In `crates/fulltime-ui/src/util/init.rs` (or the app's startup entry point), `include_bytes!` each vendored `.ttf` file.
- [x] 2.2 Call `App::text_system().add_fonts(...)` with the embedded font bytes before the first window is created.
- [x] 2.3 Propagate/log-and-abort on a font registration error rather than swallowing it.

## 3. Verify

- [x] 3.1 Run the app and confirm headings render in Sora and body text renders in Manrope.
- [x] 3.2 Confirm the vendored `OFL.txt` files are included in the build output / repo, satisfying Google Fonts' license distribution requirement.
- [x] 3.3 `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo +nightly fmt --all -- --check` pass.
