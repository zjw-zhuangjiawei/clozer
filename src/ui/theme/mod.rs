//! Centralized theme definitions and colors.
//!
//! Provides consistent styling across all UI components with support
//! for light and dark themes. The theme system is built on design tokens
//! that ensure visual consistency and accessibility compliance.
pub mod color;
pub mod role;
pub mod semantic;

use clap::ValueEnum;
use iced::theme::{Base as ThemeBase, Mode, Palette, Style as ThemeStyle};
use iced::widget::button::Style;
use serde::{Deserialize, Serialize};
use strum::{Display, VariantArray};

pub use color::ThemeColors;
pub use semantic::{
    BorderSemantic, InteractiveSemantic, SemanticPalette, SurfaceSemantic, TextSemantic,
};

pub use role::{BackgroundRole, BorderRole, ForegroundRole, InteractiveRole};

pub type StyleFn<'a> = Box<dyn Fn(&AppTheme, iced::widget::button::Status) -> Style + 'a>;

pub use super::design_tokens::{
    BorderRadiusValues, DimensionTokens, FontFamily, FontSize, IconSizeValues, IconSizeVariant,
    InputHeightTokens, RadiusVariant, ScaleFactor, ScaleSystem, Spacing, TouchTargetSize,
};

use std::sync::OnceLock;

static LIGHT_COLORS: OnceLock<color::ThemeColors> = OnceLock::new();
static DARK_COLORS: OnceLock<color::ThemeColors> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, VariantArray)]
pub enum ButtonSize {
    #[default]
    Small,
    Medium,
    Standard,
    Large,
}

impl ButtonSize {
    pub fn padding(self) -> (f32, f32) {
        match self {
            ButtonSize::Small => (8.0, 3.0),
            ButtonSize::Medium => (12.0, 6.0),
            ButtonSize::Standard => (16.0, 8.0),
            ButtonSize::Large => (24.0, 12.0),
        }
    }

    pub fn font_size(&self) -> FontSize {
        match self {
            ButtonSize::Small => FontSize::Footnote,
            ButtonSize::Medium => FontSize::Body,
            ButtonSize::Standard => FontSize::Body,
            ButtonSize::Large => FontSize::Subtitle,
        }
    }

    pub fn height(&self) -> f32 {
        let line_height = self.font_size().line_height();
        let (_, v_padding) = self.padding();
        line_height + v_padding * 2.0
    }

    pub fn to_iced_padding(self) -> iced::Padding {
        let (h, v) = self.padding();
        iced::Padding::from([v, h])
    }

    pub fn touch_target_min(&self) -> f32 {
        match self {
            ButtonSize::Small => 32.0,
            ButtonSize::Medium => 40.0,
            ButtonSize::Standard => 44.0,
            ButtonSize::Large => 48.0,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    ValueEnum,
    Display,
    strum::VariantArray,
)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum AppTheme {
    #[default]
    Light,
    Dark,
}

impl AppTheme {
    pub fn colors(&self) -> &'static color::ThemeColors {
        match self {
            AppTheme::Light => LIGHT_COLORS.get_or_init(color::ThemeColors::light),
            AppTheme::Dark => DARK_COLORS.get_or_init(color::ThemeColors::dark),
        }
    }
}

impl ThemeBase for AppTheme {
    fn default(preference: Mode) -> Self {
        match preference {
            Mode::Light => AppTheme::Light,
            Mode::Dark => AppTheme::Dark,
            Mode::None => AppTheme::Light,
        }
    }

    fn mode(&self) -> Mode {
        match self {
            AppTheme::Light => Mode::Light,
            AppTheme::Dark => Mode::Dark,
        }
    }

    fn base(&self) -> ThemeStyle {
        let colors = self.colors();
        match self {
            AppTheme::Light => ThemeStyle {
                background_color: colors.neutral.w50(),
                text_color: colors.neutral.w900(),
            },
            AppTheme::Dark => ThemeStyle {
                background_color: colors.neutral.w900(),
                text_color: colors.neutral.w50(),
            },
        }
    }

    fn palette(&self) -> Option<Palette> {
        let colors = self.colors();
        Some(Palette {
            background: colors.neutral.w900(),
            text: colors.neutral.w50(),
            primary: colors.primary.w500(),
            success: colors.functional.success.w500(),
            warning: colors.functional.warning.w500(),
            danger: colors.functional.danger.w500(),
        })
    }

    fn name(&self) -> &str {
        match self {
            AppTheme::Light => "clozer_light",
            AppTheme::Dark => "clozer_dark",
        }
    }
}
