//! Centralized theme definitions and colors.
//!
//! Provides consistent styling across all UI components with support
//! for light and dark themes.

use iced::{Color, Theme};

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
