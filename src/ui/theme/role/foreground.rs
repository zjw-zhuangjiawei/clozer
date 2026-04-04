//! Foreground role functions
//!
//! Provides color selection for text/foreground elements.

use crate::ui::theme::color::{Color, ColorMode, ColorScale};

pub struct ForegroundRole;

impl ForegroundRole {
    /// Primary text color
    pub fn primary(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w700()
    }

    /// Secondary text color
    pub fn secondary(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w500()
    }

    /// Tertiary/muted text color
    pub fn tertiary(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w400()
    }

    /// Disabled text color
    pub fn disabled(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w300()
    }
}
