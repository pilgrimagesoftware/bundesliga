## ADDED Requirements

### Requirement: gpui-ce is adopted via a workspace-level patch, not a gpui-component change
`fulltime-ui` SHALL move its `gpui`/`gpui_platform` dependency from `zed-industries/zed` to
`gpui-ce` using a workspace-level Cargo `[patch]` override that redirects the
`zed-industries/zed` source for both packages, rather than waiting on or forking
`gpui-component` to add `gpui-ce` support itself. `gpui-component`'s own `Cargo.toml` remains
unchanged; the patch redirects its transitive `gpui`/`gpui_platform` dependency along with
fulltime-ui's direct one, so both resolve to the same concrete `gpui` implementation.

#### Scenario: Patch redirects both direct and transitive gpui usage
- **WHEN** the top-level `Cargo.toml` declares
  `[patch."https://github.com/zed-industries/zed"]` redirecting `gpui` and `gpui_platform` to
  `gpui-ce/gpui-ce`
- **THEN** both fulltime-ui's own code and `gpui-component`'s widgets (`Avatar`, `GroupBox`,
  `Tag`, `Button`) compile against the same `gpui-ce`-sourced `Window`/`App`/`Div` types,
  without any change to `gpui-component`'s own manifest

### Requirement: A gpui-ce patch is verified before being considered complete
`fulltime-ui` SHALL verify a `gpui`/`gpui_platform` source swap to `gpui-ce` with a clean
`cargo build`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test
--workspace`, and a full manual visual and interaction pass across every screen covered by the
widget migration, because `gpui-ce` tracking upstream Zed is not a guarantee of exact parity
with the previously pinned Zed `gpui` revision.

#### Scenario: Compile failure after patching is treated as expected-case drift
- **WHEN** `cargo build` or `cargo clippy` fails immediately after adding the `[patch]` override
- **THEN** the failure is triaged as `gpui-ce`/Zed `gpui` API drift — by pinning the patch to a
  `gpui-ce` revision closer to the previously pinned Zed `gpui` revision, or by tracking the
  specific gap — rather than treated as a reason to abandon the patch-based approach

#### Scenario: Dependency source swap triggers full re-verification
- **WHEN** the `gpui`/`gpui_platform` dependency source changes from `zed-industries/zed` to
  `gpui-ce`
- **THEN** every screen covered by the widget migration's visual verification pass (Standings,
  Match, Team, Player, History, Plugins) is manually re-checked — including layout, both
  `FullTimeTheme` variants, window resize, input focus, and keyboard navigation — before the
  change is considered complete
