//! Extended breakpoint system for responsive layouts.

use super::mode::LayoutConfig;

/// Extended breakpoint system with more granular breakpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Breakpoint {
    /// Extra small screens (< 480px)
    XSmall,
    /// Small screens (480px - 600px)
    Small,
    /// Medium screens (600px - 900px)
    Medium,
    /// Large screens (900px - 1200px)
    Large,
    /// Extra large screens (> 1200px)
    XLarge,
}

impl Breakpoint {
    /// Determine breakpoint from window width.
    pub fn from_width(width: f32) -> Self {
        match width {
            w if w < 480.0 => Breakpoint::XSmall,
            w if w < 600.0 => Breakpoint::Small,
            w if w < 900.0 => Breakpoint::Medium,
            w if w < 1200.0 => Breakpoint::Large,
            _ => Breakpoint::XLarge,
        }
    }

    /// Get recommended layout configuration for this breakpoint.
    pub fn recommended_layout(self) -> LayoutConfig {
        match self {
            Breakpoint::XSmall | Breakpoint::Small => {
                // Single column layout
                LayoutConfig::adaptive()
            }
            Breakpoint::Medium => {
                // Two-column waterfall
                LayoutConfig::waterfall(2)
            }
            Breakpoint::Large | Breakpoint::XLarge => {
                // Three-column grid
                LayoutConfig::grid(3)
            }
        }
    }

    /// Get sidebar-to-content ratio for this breakpoint.
    /// Returns (sidebar_fraction, content_fraction).
    ///
    /// Ratios:
    /// - XSmall/Small (mobile): single column (0.0, 1.0)
    /// - Medium (tablet): 3:7 ratio (0.3, 0.7)
    /// - Large/XLarge (desktop): 4:6 ratio (0.4, 0.6)
    pub fn column_ratio(&self) -> (f32, f32) {
        match self {
            Breakpoint::XSmall | Breakpoint::Small => (0.0, 1.0),
            Breakpoint::Medium => (0.3, 0.7),
            Breakpoint::Large | Breakpoint::XLarge => (0.4, 0.6),
        }
    }

    /// Check if content should be shown in single column.
    pub fn is_single_column(&self) -> bool {
        matches!(self, Breakpoint::XSmall | Breakpoint::Small)
    }
}

impl Default for Breakpoint {
    fn default() -> Self {
        Breakpoint::Medium
    }
}
