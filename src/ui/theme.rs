//! Centralized theme definitions and colors.
//!
//! Provides consistent styling across all UI components with support
//! for light and dark themes. The theme system is built on design tokens
//! that ensure visual consistency and accessibility compliance.

use clap::ValueEnum;
use iced::theme::{Base as ThemeBase, Mode, Palette, Style as ThemeStyle};
use iced::widget::button::Style;
use iced::{Color, Length};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{Display, VariantArray};

pub type StyleFn<'a> = Box<dyn Fn(&AppTheme, iced::widget::button::Status) -> Style + 'a>;

pub use super::design_tokens::{
    BorderRadiusValues, DimensionTokens, FontFamily, FontWeights, IconSizeValues, IconSizeVariant,
    InputHeightTokens, LineHeights, RadiusVariant, ScaleFactor, ScaleSystem, SpacingScale,
    SpacingValue, TouchTargetSize, TypographyScale, TypographySizes, TypographyVariant,
};

pub mod prelude {
    pub use super::{AppTheme, ButtonSize, FontSize, Spacing, ThemeColors};
}

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
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, ValueEnum, Display,
)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum AppTheme {
    #[default]
    Light,
    Dark,
}

impl AppTheme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            AppTheme::Light => ThemeColors::light(),
            AppTheme::Dark => ThemeColors::dark(),
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
            background_color: colors.background,
            text_color: colors.text,
        }
    }

    fn palette(&self) -> Option<Palette> {
        let colors = self.colors();
        Some(Palette {
            background: colors.background,
            text: colors.text,
            primary: colors.primary,
            success: colors.success,
            warning: colors.secondary,
            danger: colors.danger,
        })
    }

    fn name(&self) -> &str {
        match self {
            AppTheme::Light => "clozer_light",
            AppTheme::Dark => "clozer_dark",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub primary: Color,
    pub primary_hover: Color,
    pub primary_active: Color,
    pub secondary: Color,
    pub neutral_100: Color,
    pub neutral_200: Color,
    pub danger: Color,
    pub danger_hover: Color,
    pub danger_active: Color,
    pub success: Color,

    pub background: Color,
    pub surface: Color,
    pub surface_elevated: Color,
    pub surface_hover: Color,

    pub text: Color,
    pub text_secondary: Color,
    pub text_on_primary: Color,

    pub border: Color,
    pub border_emphasis: Color,
    pub border_focus: Color,

    pub pos_badge_bg: Color,
    pub pos_badge_text: Color,
}

impl ThemeColors {
    pub fn light() -> Self {
        Self {
            primary: Color::from_rgb(0.30, 0.56, 0.57),
            primary_hover: Color::from_rgb(0.25, 0.46, 0.47),
            primary_active: Color::from_rgb(0.20, 0.36, 0.37),
            secondary: Color::from_rgb(0.45, 0.45, 0.45),
            neutral_100: Color::from_rgb(0.96, 0.96, 0.96),
            neutral_200: Color::from_rgb(0.91, 0.91, 0.91),
            danger: Color::from_rgb(0.70, 0.25, 0.25),
            danger_hover: Color::from_rgb(0.58, 0.18, 0.18),
            danger_active: Color::from_rgb(0.45, 0.12, 0.12),
            success: Color::from_rgb(0.22, 0.62, 0.38),

            background: Color::from_rgb(0.98, 0.98, 0.98),
            surface: Color::from_rgb(1.0, 1.0, 1.0),
            surface_elevated: Color::from_rgb(0.96, 0.96, 0.96),
            surface_hover: Color::from_rgb(0.92, 0.92, 0.92),

            text: Color::from_rgb(0.1, 0.1, 0.1),
            text_secondary: Color::from_rgb(0.4, 0.4, 0.4),
            text_on_primary: Color::from_rgb(1.0, 1.0, 1.0),

            border: Color::from_rgb(0.85, 0.85, 0.85),
            border_emphasis: Color::from_rgb(0.65, 0.65, 0.65),
            border_focus: Color::from_rgb(0.30, 0.56, 0.57),

            pos_badge_bg: Color::from_rgb(0.82, 0.82, 0.82),
            pos_badge_text: Color::from_rgb(0.32, 0.32, 0.32),
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: Color::from_rgb(0.40, 0.60, 0.62),
            primary_hover: Color::from_rgb(0.50, 0.70, 0.72),
            primary_active: Color::from_rgb(0.60, 0.78, 0.80),
            secondary: Color::from_rgb(0.62, 0.62, 0.68),
            neutral_100: Color::from_rgb(0.25, 0.25, 0.29),
            neutral_200: Color::from_rgb(0.35, 0.35, 0.40),
            danger: Color::from_rgb(0.88, 0.38, 0.38),
            danger_hover: Color::from_rgb(0.98, 0.52, 0.52),
            danger_active: Color::from_rgb(1.0, 0.70, 0.70),
            success: Color::from_rgb(0.25, 0.70, 0.45),

            background: Color::from_rgb(0.1, 0.1, 0.12),
            surface: Color::from_rgb(0.15, 0.15, 0.18),
            surface_elevated: Color::from_rgb(0.20, 0.20, 0.24),
            surface_hover: Color::from_rgb(0.25, 0.25, 0.28),

            text: Color::from_rgb(0.92, 0.92, 0.95),
            text_secondary: Color::from_rgb(0.65, 0.65, 0.7),
            text_on_primary: Color::from_rgb(0.1, 0.1, 0.12),

            border: Color::from_rgb(0.3, 0.3, 0.35),
            border_emphasis: Color::from_rgb(0.45, 0.45, 0.5),
            border_focus: Color::from_rgb(0.40, 0.60, 0.62),

            pos_badge_bg: Color::from_rgb(0.35, 0.35, 0.40),
            pos_badge_text: Color::from_rgb(0.92, 0.92, 0.95),
        }
    }
}
