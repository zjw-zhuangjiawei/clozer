//! Interactive role functions
//!
//! Provides color selection for interactive elements (buttons, links, etc.).

use crate::ui::theme::color::{Color, ColorMode, ColorScale, FunctionalPalette};

pub struct InteractiveRole;

impl InteractiveRole {
    /// Primary action color
    pub fn primary(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w500()
    }

    /// Primary action hover color
    pub fn primary_hover(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w600()
    }

    /// Secondary action background
    pub fn secondary(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w200(),
            ColorMode::Dark => scale.w700(),
        }
    }

    /// Secondary action hover background
    pub fn secondary_hover(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w300(),
            ColorMode::Dark => scale.w600(),
        }
    }

    /// Danger/destructive action color
    pub fn danger(functional: &FunctionalPalette, _mode: ColorMode) -> Color {
        functional.danger.w500()
    }

    /// Danger/destructive action hover color
    pub fn danger_hover(functional: &FunctionalPalette, _mode: ColorMode) -> Color {
        functional.danger.w600()
    }

    /// Link color
    pub fn link(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w600()
    }
}
