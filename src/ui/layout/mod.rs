//! UI Layout system supporting multiple configurable layout modes.
//!
//! This module provides a flexible layout system with support for:
//! - Adaptive layout (single column or master-detail based on breakpoint)
//! - Grid layout (multi-column evenly distributed)
//! - Waterfall layout (staggered arrangement for varying heights)
//!
//! # Example
//!
//! ```
//! use clozer::ui::layout::{LayoutConfig, LayoutMode, Breakpoint};
//!
//! // Create a grid layout configuration
//! let config = LayoutConfig::grid(3);
//!
//! // Get recommended layout for a breakpoint
//! let breakpoint = Breakpoint::from_width(1200.0);
//! let recommended = breakpoint.recommended_layout();
//! ```

pub mod adaptive;
pub mod breakpoint;
pub mod grid;
pub mod mode;
pub mod waterfall;

pub use adaptive::{AdaptiveLayout, adaptive_layout};
pub use breakpoint::Breakpoint;
pub use grid::{GridLayout, grid_layout};
pub use mode::{LayoutConfig, LayoutMode};
pub use waterfall::{WaterfallLayout, waterfall_layout};

use iced::Element;

pub fn build_layout<'a, M: 'a, T: 'a>(
    config: &LayoutConfig,
    nav_bar: Element<'a, M, T>,
    content: Element<'a, M, T>,
    breakpoint: Breakpoint,
) -> Element<'a, M, T> {
    match config.mode {
        LayoutMode::Adaptive => adaptive::adaptive_layout(nav_bar, content, breakpoint),
        LayoutMode::Grid => {
            // Grid mode: content should already be grid items
            grid::grid_layout(config.columns, vec![content])
        }
        LayoutMode::Waterfall => {
            // Waterfall mode: content should already be waterfall items
            waterfall::waterfall_layout(config.columns, vec![content])
        }
    }
}

/// Get default layout configuration for a breakpoint.
pub fn default_layout_for_breakpoint(breakpoint: Breakpoint) -> LayoutConfig {
    breakpoint.recommended_layout()
}
