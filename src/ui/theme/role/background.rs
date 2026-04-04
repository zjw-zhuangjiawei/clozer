//! Background role functions
//!
//! Provides color selection for background surfaces.

use crate::ui::theme::color::{Color, ColorMode, ColorScale};

pub struct BackgroundRole;

impl BackgroundRole {
    /// Page background
    pub fn base(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w50(),
            ColorMode::Dark => scale.w900(),
        }
    }

    /// Card/panel background
    pub fn raised(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w100(),
            ColorMode::Dark => scale.w800(),
        }
    }

    /// Elevated/floating layer background
    pub fn elevated(scale: &ColorScale, mode: ColorMode) -> Color {
        match mode {
            ColorMode::Light => scale.w200(),
            ColorMode::Dark => scale.w700(),
        }
    }

    /// Overlay/modal backdrop
    pub fn overlay(scale: &ColorScale, _mode: ColorMode) -> Color {
        scale.w900()
    }
}
