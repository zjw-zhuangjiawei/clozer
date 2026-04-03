//! Centralized theme definitions and colors.
//!
//! Provides consistent styling across all UI components with support
//! for light and dark themes.

use iced::{Color, Length, Theme};
use std::fmt;
use strum::{Display, VariantArray};

/// Font size variants for consistent typography.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, VariantArray)]
pub enum FontSize {
    /// Caption size (11px) - auxiliary text, secondary labels
    #[default]
    Caption,
    /// Footnote size (12px) - badges, section titles, secondary info
    Footnote,
    /// Body size (14px) - main text, button labels, most UI text
    Body,
    /// Subtitle size (16px) - small headings, dialog titles
    Subtitle,
    /// Title size (18px) - panel titles, detail page titles
    Title,
    /// Heading size (20px) - page main headings
    Heading,
    /// Display size (24px) - empty state text
    Display,
}

impl FontSize {
    /// Returns the font size in pixels.
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

    /// Returns the line height in pixels.
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
}

/// Button size variants with consistent padding and text sizing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, VariantArray)]
pub enum ButtonSize {
    /// Small button (24px height) - inline actions, dense lists
    #[default]
    Small,
    /// Medium button (30px height) - secondary actions, toolbar
    Medium,
    /// Standard button (36px height) - default size, most buttons
    Standard,
    /// Large button (48px height) - primary actions, CTA
    Large,
}

impl ButtonSize {
    /// Returns the padding (horizontal, vertical) for this button size.
    pub fn padding(self) -> (f32, f32) {
        match self {
            ButtonSize::Small => (8.0, 3.0),
            ButtonSize::Medium => (12.0, 6.0),
            ButtonSize::Standard => (16.0, 8.0),
            ButtonSize::Large => (24.0, 12.0),
        }
    }

    /// Returns the text size for this button size.
    pub fn font_size(self) -> FontSize {
        match self {
            ButtonSize::Small => FontSize::Footnote,
            ButtonSize::Medium => FontSize::Body,
            ButtonSize::Standard => FontSize::Body,
            ButtonSize::Large => FontSize::Subtitle,
        }
    }

    /// Returns the recommended height for this button.
    /// Height = line_height + v_padding * 2
    /// Example: Small = 18 + 3 * 2 = 24px
    pub fn height(self) -> f32 {
        let line_height = self.font_size().line_height();
        let (_, v_padding) = self.padding();
        line_height + v_padding * 2.0
    }

    /// Convert to iced::Padding for direct use.
    pub fn to_iced_padding(self) -> iced::Padding {
        let (h, v) = self.padding();
        iced::Padding::from([v, h])
    }
}

/// Spacing constants for consistent layout spacing.
#[derive(Debug, Clone, Copy, Default)]
pub struct Spacing {
    /// Extra small spacing (4px)
    pub xs: f32,
    /// Small spacing (8px)
    pub s: f32,
    /// Medium spacing (12px)
    pub m: f32,
    /// Large spacing (16px)
    pub l: f32,
    /// Extra large spacing (24px)
    pub xl: f32,
    /// Extra extra large spacing (32px)
    pub xxl: f32,
}

impl Spacing {
    /// Default spacing constants.
    pub const DEFAULT: Self = Self {
        xs: 4.0,
        s: 8.0,
        m: 12.0,
        l: 16.0,
        xl: 24.0,
        xxl: 32.0,
    };
}

impl fmt::Display for Spacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Spacing(xs={}, s={}, m={}, l={}, xl={}, xxl={})",
            self.xs, self.s, self.m, self.l, self.xl, self.xxl
        )
    }
}

/// Dimension constants for UI components.
#[derive(Debug, Clone, Copy, Default)]
pub struct Dimensions {
    /// Input field height for single-line inputs (36px).
    pub input_height: f32,
    /// Input field height for multi-line text areas (72px).
    pub textarea_min_height: f32,
    /// Icon sizes.
    pub icon: IconSize,
    /// Border dimensions.
    pub border: BorderSize,
    /// Corner radius values.
    pub radius: RadiusSize,
}

impl Dimensions {
    /// Default dimensions.
    pub const DEFAULT: Self = Self {
        input_height: 36.0,
        textarea_min_height: 72.0,
        icon: IconSize::Small,       // 16px - default icon size
        border: BorderSize::DEFAULT, // thin=1, medium=2, thick=3
        radius: RadiusSize::Small,   // 4px - default radius
    };
}

/// Icon size variants.
#[derive(Debug, Clone, Copy, Default)]
pub enum IconSize {
    /// Small icon (16px) - inline icons
    #[default]
    Small,
    /// Medium icon (20px) - toolbar, navigation
    Medium,
    /// Large icon (24px) - primary actions, empty states
    Large,
    /// Extra large icon (32px) - special emphasis
    XLarge,
}

impl IconSize {
    /// Returns the icon size in pixels.
    pub const fn px(self) -> f32 {
        match self {
            IconSize::Small => 16.0,
            IconSize::Medium => 20.0,
            IconSize::Large => 24.0,
            IconSize::XLarge => 32.0,
        }
    }
}

/// Border size variants.
#[derive(Debug, Clone, Copy)]
pub struct BorderSize {
    /// Thin border width (1px) - standard borders
    pub thin: f32,
    /// Medium border width (2px) - emphasis borders
    pub medium: f32,
    /// Thick border width (3px) - strong emphasis
    pub thick: f32,
}

impl Default for BorderSize {
    fn default() -> Self {
        Self {
            thin: 1.0,
            medium: 2.0,
            thick: 3.0,
        }
    }
}

impl BorderSize {
    /// Returns the default border sizes.
    pub const DEFAULT: Self = Self {
        thin: 1.0,
        medium: 2.0,
        thick: 3.0,
    };
}

/// Radius size variants.
#[derive(Debug, Clone, Copy, Default)]
pub enum RadiusSize {
    /// No radius (0px) - sharp corners
    None,
    /// Small radius (4px) - inputs, small buttons
    #[default]
    Small,
    /// Medium radius (8px) - cards, panels
    Medium,
    /// Large radius (12px) - modals, large elements
    Large,
    /// Full radius - circular elements
    Full,
}

impl RadiusSize {
    /// Returns the radius in pixels.
    pub const fn px(self) -> f32 {
        match self {
            RadiusSize::None => 0.0,
            RadiusSize::Small => 4.0,
            RadiusSize::Medium => 8.0,
            RadiusSize::Large => 12.0,
            RadiusSize::Full => 999.0,
        }
    }
}

/// Application theme variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AppTheme {
    /// Light theme (default)
    #[default]
    Light,
    /// Dark theme
    Dark,
}

impl AppTheme {
    /// Returns the Iced theme for this app theme.
    pub fn to_iced_theme(self) -> Theme {
        match self {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
        }
    }

    /// Returns the color palette for this theme.
    pub fn colors(self) -> ThemeColors {
        match self {
            AppTheme::Light => ThemeColors::light(),
            AppTheme::Dark => ThemeColors::dark(),
        }
    }
}

/// Color palette for a theme.
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    /// Primary brand color
    pub primary: Color,
    /// Secondary/muted color
    pub secondary: Color,
    /// Danger/error color
    pub danger: Color,
    /// Success color
    pub success: Color,

    // Background colors
    /// Main background
    pub background: Color,
    /// Surface background (cards, panels)
    pub surface: Color,
    /// Elevated surface background
    pub surface_elevated: Color,
    /// Hover state background
    pub surface_hover: Color,

    // Text colors
    /// Primary text color
    pub text: Color,
    /// Secondary/muted text color
    pub text_secondary: Color,

    // Border colors
    /// Standard border color
    pub border: Color,
    /// Focused border color
    pub border_focus: Color,

    // Component-specific colors
    /// POS badge background
    pub pos_badge_bg: Color,
    /// POS badge text
    pub pos_badge_text: Color,
}

impl ThemeColors {
    /// Light theme color palette.
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

    /// Dark theme color palette.
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
}

impl From<AppTheme> for Option<Theme> {
    fn from(value: AppTheme) -> Self {
        Some(value.to_iced_theme())
    }
}

/// Responsive breakpoint for layout adaptation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, VariantArray)]
pub enum Breakpoint {
    /// Mobile devices (width < 600px)
    Mobile,
    /// Tablet devices (600px <= width < 1024px)
    Tablet,
    /// Desktop devices (width >= 1024px)
    #[default]
    Desktop,
}

impl Breakpoint {
    /// Determine breakpoint from window width.
    pub fn from_width(width: f32) -> Self {
        if width < 600.0 {
            Breakpoint::Mobile
        } else if width < 1024.0 {
            Breakpoint::Tablet
        } else {
            Breakpoint::Desktop
        }
    }

    /// Returns the sidebar-to-content ratio for this breakpoint.
    /// Returns (sidebar_fraction, content_fraction).
    ///
    /// Ratios:
    /// - Desktop (>1024px): 4:6
    /// - Tablet (600-1024px): 3:7
    /// - Mobile (<=600px): single column
    pub fn column_ratio(self) -> (f32, f32) {
        match self {
            Breakpoint::Mobile => (0.0, 1.0),  // Single column, no sidebar
            Breakpoint::Tablet => (0.3, 0.7),  // 3:7 ratio
            Breakpoint::Desktop => (0.4, 0.6), // 4:6 ratio
        }
    }

    /// Returns the sidebar width as a Length for this breakpoint.
    pub fn sidebar_width(self) -> Length {
        match self {
            Breakpoint::Mobile => Length::Fill,
            Breakpoint::Tablet => Length::Fixed(200.0),
            Breakpoint::Desktop => Length::Fixed(250.0),
        }
    }

    /// Check if content should be shown in single column.
    pub fn is_single_column(self) -> bool {
        matches!(self, Breakpoint::Mobile)
    }
}

/// Default application styles for components.
#[derive(Debug, Clone, Copy, Default)]
pub struct ApplicationStyle {
    /// Default button size
    pub button_size: ButtonSize,
    /// Default spacing
    pub spacing: Spacing,
}

impl ApplicationStyle {
    /// Create a new application style with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the default button padding.
    pub fn button_padding(&self) -> (f32, f32) {
        self.button_size.padding()
    }

    /// Returns the default button text size.
    pub fn button_text_size(&self) -> f32 {
        self.button_size.font_size().px()
    }
}
