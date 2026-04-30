//! Foreground role functions
//!
//! Provides color selection for text/foreground elements.

use crate::ui::theme::color::{Color, ColorMode, ColorScale};

pub struct ForegroundRole;

impl ForegroundRole {
    /// Primary text color
    pub fn primary(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w700(),
            ColorMode::Dark => scale.w100(),
        }
    }

    /// Secondary text color
    pub fn secondary(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w500(),
            ColorMode::Dark => scale.w300(),
        }
    }

    /// Tertiary/muted text color
    pub fn tertiary(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w400(),
            ColorMode::Dark => scale.w500(),
        }
    }

    /// Disabled text color
    pub fn disabled(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w300(),
            ColorMode::Dark => scale.w600(),
        }
    }
}
