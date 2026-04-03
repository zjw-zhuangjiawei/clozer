//! Layout mode definitions for configurable layout systems.

use strum::{Display, VariantArray};

/// Layout mode enumeration for different layout strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, VariantArray)]
pub enum LayoutMode {
    /// Adaptive layout (single column or master-detail based on breakpoint)
    #[default]
    Adaptive,
    /// Grid layout (multiple columns evenly distributed)
    Grid,
    /// Waterfall layout (staggered arrangement for varying height content)
    Waterfall,
}

/// Layout configuration for the application.
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    /// Layout mode
    pub mode: LayoutMode,
    /// Number of columns (for Grid/Waterfall modes)
    pub columns: u8,
    /// Column spacing in pixels
    pub column_spacing: f32,
    /// Row spacing in pixels
    pub row_spacing: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            mode: LayoutMode::Adaptive,
            columns: 2,
            column_spacing: 16.0,
            row_spacing: 16.0,
        }
    }
}

impl LayoutConfig {
    /// Create an adaptive layout configuration.
    pub fn adaptive() -> Self {
        Self::default()
    }

    /// Create a grid layout configuration with specified columns.
    pub fn grid(columns: u8) -> Self {
        Self {
            mode: LayoutMode::Grid,
            columns,
            ..Self::default()
        }
    }

    /// Create a waterfall layout configuration with specified columns.
    pub fn waterfall(columns: u8) -> Self {
        Self {
            mode: LayoutMode::Waterfall,
            columns,
            ..Self::default()
        }
    }
}
