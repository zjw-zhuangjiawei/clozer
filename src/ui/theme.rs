//! Centralized theme definitions and colors.
//!
//! Provides consistent styling across all UI components with support
//! for light and dark themes. The theme system is built on design tokens
//! that ensure visual consistency and accessibility compliance.

use super::design_tokens::{DesignTokens, SemanticColors};
use iced::{Color, Length, Theme};
use std::fmt;
use strum::{Display, VariantArray};

pub use super::design_tokens::{
    CategoryColor as ThemeCategoryColor, DesignTokens as ThemeDesignTokens, RadiusVariant,
    Severity as ThemeSeverity, SpacingScale, SpacingValue,
    TypographyVariant as ThemeTypographyVariant,
};

pub mod prelude {
    pub use super::{
        AppTheme, AppThemeColors, Breakpoint, ButtonSize, DesignTokensHelper, FontSize, Spacing,
        SpacingValue, ThemeColors,
    };
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

    pub fn to_typography_variant(&self) -> ThemeTypographyVariant {
        match self {
            FontSize::Caption => ThemeTypographyVariant::Caption,
            FontSize::Footnote => ThemeTypographyVariant::Footnote,
            FontSize::Body => ThemeTypographyVariant::Body,
            FontSize::Subtitle => ThemeTypographyVariant::Subtitle,
            FontSize::Title => ThemeTypographyVariant::Title,
            FontSize::Heading => ThemeTypographyVariant::Heading,
            FontSize::Display => ThemeTypographyVariant::Display,
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

impl AppTheme {
    pub fn to_iced_theme(self) -> Theme {
        match self {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
        }
    }

    pub fn colors(&self) -> ThemeColors {
        match self {
            AppTheme::Light => ThemeColors::light(),
            AppTheme::Dark => ThemeColors::dark(),
        }
    }

    pub fn design_tokens(&self) -> DesignTokens {
        match self {
            AppTheme::Light => DesignTokens::light(),
            AppTheme::Dark => DesignTokens::dark(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AppTheme {
    #[default]
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub primary: Color,
    pub secondary: Color,
    pub danger: Color,
    pub success: Color,

    pub background: Color,
    pub surface: Color,
    pub surface_elevated: Color,
    pub surface_hover: Color,

    pub text: Color,
    pub text_secondary: Color,

    pub border: Color,
    pub border_focus: Color,

    pub pos_badge_bg: Color,
    pub pos_badge_text: Color,
}

impl ThemeColors {
    pub fn light() -> Self {
        Self {
            primary: Color::from_rgb(0.2, 0.4, 0.6),
            secondary: Color::from_rgb(0.5, 0.5, 0.5),
            danger: Color::from_rgb(0.8, 0.2, 0.2),
            success: Color::from_rgb(0.2, 0.6, 0.3),

            background: Color::from_rgb(0.98, 0.98, 0.98),
            surface: Color::from_rgb(1.0, 1.0, 1.0),
            surface_elevated: Color::from_rgb(0.95, 0.95, 0.95),
            surface_hover: Color::from_rgb(0.92, 0.92, 0.92),

            text: Color::from_rgb(0.1, 0.1, 0.1),
            text_secondary: Color::from_rgb(0.4, 0.4, 0.4),

            border: Color::from_rgb(0.85, 0.85, 0.85),
            border_focus: Color::from_rgb(0.2, 0.4, 0.6),

            pos_badge_bg: Color::from_rgb(0.2, 0.4, 0.6),
            pos_badge_text: Color::WHITE,
        }
    }

    pub fn dark() -> Self {
        Self {
            primary: Color::from_rgb(0.4, 0.6, 0.8),
            secondary: Color::from_rgb(0.6, 0.6, 0.6),
            danger: Color::from_rgb(0.9, 0.3, 0.3),
            success: Color::from_rgb(0.3, 0.7, 0.4),

            background: Color::from_rgb(0.1, 0.1, 0.12),
            surface: Color::from_rgb(0.15, 0.15, 0.18),
            surface_elevated: Color::from_rgb(0.2, 0.2, 0.24),
            surface_hover: Color::from_rgb(0.25, 0.25, 0.28),

            text: Color::from_rgb(0.9, 0.9, 0.9),
            text_secondary: Color::from_rgb(0.6, 0.6, 0.6),

            border: Color::from_rgb(0.3, 0.3, 0.35),
            border_focus: Color::from_rgb(0.4, 0.6, 0.8),

            pos_badge_bg: Color::from_rgb(0.4, 0.6, 0.8),
            pos_badge_text: Color::from_rgb(0.1, 0.1, 0.12),
        }
    }

    pub fn from_semantic(semantic: &SemanticColors) -> Self {
        Self {
            primary: semantic.primary.scale(500),
            secondary: semantic.secondary.scale(500),
            danger: semantic.danger.scale(500),
            success: semantic.success.scale(500),

            background: semantic.background.base,
            surface: semantic.background.surface,
            surface_elevated: semantic.background.elevated,
            surface_hover: semantic.neutral.scale(200),

            text: semantic.text.primary,
            text_secondary: semantic.text.secondary,

            border: semantic.border.default,
            border_focus: semantic.focus.primary,

            pos_badge_bg: semantic.primary.scale(500),
            pos_badge_text: semantic.text.on_primary,
        }
    }
}

impl From<AppTheme> for Option<Theme> {
    fn from(value: AppTheme) -> Self {
        Some(value.to_iced_theme())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ApplicationStyle {
    pub button_size: ButtonSize,
    pub spacing: Spacing,
}

impl ApplicationStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn button_padding(&self) -> (f32, f32) {
        self.button_size.padding()
    }

    pub fn button_text_size(&self) -> f32 {
        self.button_size.font_size().px()
    }
}

pub trait DesignTokensHelper {
    fn primary(&self) -> Color;
    fn secondary(&self) -> Color;
    fn danger(&self) -> Color;
    fn success(&self) -> Color;
    fn background(&self) -> Color;
    fn surface(&self) -> Color;
    fn surface_elevated(&self) -> Color;
    fn text(&self) -> Color;
    fn text_secondary(&self) -> Color;
    fn border(&self) -> Color;
    fn border_focus(&self) -> Color;
    fn pos_badge_bg(&self) -> Color;
    fn pos_badge_text(&self) -> Color;
}

impl DesignTokensHelper for AppTheme {
    fn primary(&self) -> Color {
        self.colors().primary
    }

    fn secondary(&self) -> Color {
        self.colors().secondary
    }

    fn danger(&self) -> Color {
        self.colors().danger
    }

    fn success(&self) -> Color {
        self.colors().success
    }

    fn background(&self) -> Color {
        self.colors().background
    }

    fn surface(&self) -> Color {
        self.colors().surface
    }

    fn surface_elevated(&self) -> Color {
        self.colors().surface_elevated
    }

    fn text(&self) -> Color {
        self.colors().text
    }

    fn text_secondary(&self) -> Color {
        self.colors().text_secondary
    }

    fn border(&self) -> Color {
        self.colors().border
    }

    fn border_focus(&self) -> Color {
        self.colors().border_focus
    }

    fn pos_badge_bg(&self) -> Color {
        self.colors().pos_badge_bg
    }

    fn pos_badge_text(&self) -> Color {
        self.colors().pos_badge_text
    }
}

pub struct AppThemeColors {
    colors: ThemeColors,
}

impl AppThemeColors {
    pub fn from_theme(theme: AppTheme) -> Self {
        Self {
            colors: theme.colors(),
        }
    }

    pub fn primary(&self) -> Color {
        self.colors.primary
    }
    pub fn secondary(&self) -> Color {
        self.colors.secondary
    }
    pub fn danger(&self) -> Color {
        self.colors.danger
    }
    pub fn success(&self) -> Color {
        self.colors.success
    }
    pub fn background(&self) -> Color {
        self.colors.background
    }
    pub fn surface(&self) -> Color {
        self.colors.surface
    }
    pub fn surface_elevated(&self) -> Color {
        self.colors.surface_elevated
    }
    pub fn surface_hover(&self) -> Color {
        self.colors.surface_hover
    }
    pub fn text(&self) -> Color {
        self.colors.text
    }
    pub fn text_secondary(&self) -> Color {
        self.colors.text_secondary
    }
    pub fn border(&self) -> Color {
        self.colors.border
    }
    pub fn border_focus(&self) -> Color {
        self.colors.border_focus
    }
    pub fn pos_badge_bg(&self) -> Color {
        self.colors.pos_badge_bg
    }
    pub fn pos_badge_text(&self) -> Color {
        self.colors.pos_badge_text
    }
}
