//! Extended breakpoint system for responsive layouts.

use super::mode::LayoutConfig;
use iced::Length;

/// Extended breakpoint system with more granular breakpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Breakpoint {
    #[default]
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

impl Breakpoint {
    pub fn from_width(width: f32) -> Self {
        match width {
            w if w < 480.0 => Breakpoint::XSmall,
            w if w < 600.0 => Breakpoint::Small,
            w if w < 900.0 => Breakpoint::Medium,
            w if w < 1200.0 => Breakpoint::Large,
            _ => Breakpoint::XLarge,
        }
    }

    pub fn recommended_layout(self) -> LayoutConfig {
        match self {
            Breakpoint::XSmall | Breakpoint::Small => LayoutConfig::adaptive(),
            Breakpoint::Medium => LayoutConfig::waterfall(2),
            Breakpoint::Large | Breakpoint::XLarge => LayoutConfig::grid(3),
        }
    }

    pub fn column_ratio(&self) -> (f32, f32) {
        match self {
            Breakpoint::XSmall | Breakpoint::Small => (0.0, 1.0),
            Breakpoint::Medium => (0.3, 0.7),
            Breakpoint::Large | Breakpoint::XLarge => (0.4, 0.6),
        }
    }

    pub fn sidebar_width(&self) -> Length {
        match self {
            Breakpoint::XSmall | Breakpoint::Small => Length::Fill,
            Breakpoint::Medium => Length::Fixed(200.0),
            Breakpoint::Large | Breakpoint::XLarge => Length::Fixed(250.0),
        }
    }

    pub fn is_single_column(&self) -> bool {
        matches!(self, Breakpoint::XSmall | Breakpoint::Small)
    }
}
