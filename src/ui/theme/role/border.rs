//! Border role functions
//!
//! Provides color selection for borders.

use crate::ui::theme::color::{Color, ColorMode, ColorScale};

pub struct BorderRole;

impl BorderRole {
    /// Default border color
    pub fn default(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w200(),
            ColorMode::Dark => scale.w600(),
        }
    }

    /// Strong/emphasized border color
    pub fn strong(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w300(),
            ColorMode::Dark => scale.w500(),
        }
    }

    /// Focus ring color
    pub fn focus(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w500()
    }
}
