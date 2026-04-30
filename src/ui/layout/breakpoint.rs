//! Breakpoint system for responsive layouts.

/// Breakpoint based on window width for responsive UI adaptation.
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

    /// Column ratio for master-detail panels: (left_ratio, right_ratio).
    /// When 0.0/1.0, the panel uses single-column mode.
    pub fn column_ratio(&self) -> (f32, f32) {
        match self {
            Breakpoint::XSmall | Breakpoint::Small => (0.0, 1.0),
            Breakpoint::Medium => (0.3, 0.7),
            Breakpoint::Large | Breakpoint::XLarge => (0.4, 0.6),
        }
    }

    /// Whether to use single-column layout (no detail panel).
    pub fn is_single_column(&self) -> bool {
        matches!(self, Breakpoint::XSmall | Breakpoint::Small)
    }

    /// Sidebar shows full width (icon + text) only on large+ screens.
    pub fn sidebar_expanded(&self) -> bool {
        matches!(self, Breakpoint::Large | Breakpoint::XLarge)
    }

    /// Sidebar panel width in pixels.
    pub fn sidebar_panel_width(&self) -> f32 {
        match self {
            Breakpoint::XSmall | Breakpoint::Small => 0.0,
            Breakpoint::Medium => 52.0,
            Breakpoint::Large => 200.0,
            Breakpoint::XLarge => 220.0,
        }
    }

    /// Use bottom tab bar instead of sidebar on very narrow screens.
    pub fn use_bottom_bar(&self) -> bool {
        matches!(self, Breakpoint::XSmall | Breakpoint::Small)
    }
}
