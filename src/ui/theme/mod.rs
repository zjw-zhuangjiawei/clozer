//! Centralized theme definitions and colors.
//!
//! Provides consistent styling across all UI components with support
//! for light and dark themes. The theme system is built on design tokens
//! that ensure visual consistency and accessibility compliance.
pub mod color;
pub mod role;
pub mod semantic;

use clap::ValueEnum;
use iced::Length;
use iced::theme::{Base as ThemeBase, Mode, Palette, Style as ThemeStyle};
use iced::widget::button::Style;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{Display, VariantArray};

pub use color::ThemeColors;
pub use semantic::{
    BorderSemantic, InteractiveSemantic, SemanticPalette, SurfaceSemantic, TextSemantic,
};

pub use role::{BackgroundRole, BorderRole, ForegroundRole, InteractiveRole};

pub type StyleFn<'a> = Box<dyn Fn(&AppTheme, iced::widget::button::Status) -> Style + 'a>;

pub use super::design_tokens::{
    BorderRadiusValues, DimensionTokens, FontFamily, FontWeights, IconSizeValues, IconSizeVariant,
    InputHeightTokens, LineHeights, RadiusVariant, ScaleFactor, ScaleSystem, SpacingScale,
    SpacingValue, TouchTargetSize, TypographyScale, TypographySizes, TypographyVariant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, VariantArray)]
pub enum Breakpoint {
    #[default]
    Desktop,
    Tablet,
    Mobile,
}

impl Breakpoint {
    pub fn from_width(width: f32) -> Self {
        if width < 600.0 {
            Breakpoint::Mobile
        } else if width < 1024.0 {
            Breakpoint::Tablet
        } else {
            Breakpoint::Desktop
        }
    }

    pub fn column_ratio(&self) -> (f32, f32) {
        match self {
            Breakpoint::Mobile => (0.0, 1.0),
            Breakpoint::Tablet => (0.3, 0.7),
            Breakpoint::Desktop => (0.4, 0.6),
        }
    }

    pub fn sidebar_width(&self) -> Length {
        match self {
            Breakpoint::Mobile => Length::Fill,
            Breakpoint::Tablet => Length::Fixed(200.0),
            Breakpoint::Desktop => Length::Fixed(250.0),
        }
    }

    pub fn is_single_column(&self) -> bool {
        matches!(self, Breakpoint::Mobile)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, VariantArray)]
pub enum FontSize {
    #[default]
    Caption,
    Footnote,
    Body,
    Subtitle,
    Title,
    Heading,
    Display,
}

impl FontSize {
    pub const fn px(self) -> f32 {
        match self {
            FontSize::Caption => 11.0,
            FontSize::Footnote => 12.0,
            FontSize::Body => 14.0,
            FontSize::Subtitle => 16.0,
            FontSize::Title => 18.0,
            FontSize::Heading => 20.0,
            FontSize::Display => 24.0,
        }
    }

    pub const fn line_height(self) -> f32 {
        match self {
            FontSize::Caption => 16.0,
            FontSize::Footnote => 18.0,
            FontSize::Body => 20.0,
            FontSize::Subtitle => 24.0,
            FontSize::Title => 26.0,
            FontSize::Heading => 28.0,
            FontSize::Display => 32.0,
        }
    }

    pub fn to_typography_variant(&self) -> TypographyVariant {
        match self {
            FontSize::Caption => TypographyVariant::Caption,
            FontSize::Footnote => TypographyVariant::Footnote,
            FontSize::Body => TypographyVariant::Body,
            FontSize::Subtitle => TypographyVariant::Subtitle,
            FontSize::Title => TypographyVariant::Title,
            FontSize::Heading => TypographyVariant::Heading,
            FontSize::Display => TypographyVariant::Display,
        }
    }
}

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

#[derive(Debug, Clone, Copy, Default)]
pub struct Spacing {
    pub xxs: f32,
    pub xs: f32,
    pub xs2: f32,
    pub s: f32,
    pub s2: f32,
    pub m: f32,
    pub l: f32,
    pub l2: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl Spacing {
    pub const DEFAULT: Self = Self {
        xxs: 2.0,
        xs: 4.0,
        xs2: 5.0,
        s: 8.0,
        s2: 10.0,
        m: 12.0,
        l: 16.0,
        l2: 20.0,
        xl: 24.0,
        xxl: 32.0,
    };

    pub fn new() -> Self {
        Self::DEFAULT
    }

    pub fn spacing_tokens(&self) -> SpacingScale {
        SpacingScale {
            unit: 4.0,
            scale: [
                self.xxs, self.xs, self.xs2, self.s, self.s2, self.m, self.l, self.l2, self.xl,
            ],
        }
    }

    pub fn value(&self, sv: SpacingValue) -> f32 {
        match sv {
            SpacingValue::Xxs => self.xxs,
            SpacingValue::Xs => self.xs,
            SpacingValue::Sm => self.xs2,
            SpacingValue::Md => self.s,
            SpacingValue::Lg => self.s2,
            SpacingValue::Xl => self.m,
            SpacingValue::Xxl => self.l,
            SpacingValue::Xxxl => self.l2,
            SpacingValue::Huge => self.xl,
        }
    }

    pub fn tight(&self) -> f32 {
        self.xs
    }

    pub fn compact(&self) -> f32 {
        self.s
    }

    pub fn standard(&self) -> f32 {
        self.m
    }

    pub fn relaxed(&self) -> f32 {
        self.l
    }

    pub fn spacious(&self) -> f32 {
        self.xl
    }
}

impl fmt::Display for Spacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Spacing(xxs={}, xs={}, xs2={}, s={}, s2={}, m={}, l={}, l2={}, xl={}, xxl={})",
            self.xxs,
            self.xs,
            self.xs2,
            self.s,
            self.s2,
            self.m,
            self.l,
            self.l2,
            self.xl,
            self.xxl
        )
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
    pub fn colors(&self) -> color::ThemeColors {
        match self {
            AppTheme::Light => color::ThemeColors::light(),
            AppTheme::Dark => color::ThemeColors::dark(),
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
        ThemeStyle {
            background_color: colors.neutral.w50(),
            text_color: colors.neutral.w900(),
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
