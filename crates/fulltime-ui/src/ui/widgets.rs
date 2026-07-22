//! Small reusable rendering helpers shared across views.
//!
//! Evaluated against `gpui-component` during the `use-gpui-component`
//! migration and kept custom: `small_caps_text` is a string transform, not
//! a render function, so it isn't a "UI component" this migration covers.

use gpui::SharedString;

/// Uppercases a field/section label.
///
/// `gpui` has no `font-variant: small-caps`, and synthesizing it by rendering
/// two run sizes in one row put mismatched-height text on the same line
/// (baselines didn't line up). Plain uppercase gets the "this is a label, not
/// a value" visual distinction without that defect.
pub fn small_caps_text(text: impl Into<SharedString>) -> String {
    text.into().to_uppercase()
}
