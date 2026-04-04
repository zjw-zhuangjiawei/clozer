//! Adaptive layout implementation.

use super::breakpoint::Breakpoint;
use crate::ui::AppTheme;
use crate::ui::theme::Spacing;
use iced::Element;
use iced::widget::Column;

/// Adaptive layout builder for responsive designs.
pub struct AdaptiveLayout<'a, M, T> {
    nav_bar: Element<'a, M, T>,
    content: Element<'a, M, T>,
}

impl<'a, M: 'a, T: 'a> AdaptiveLayout<'a, M, T> {
    /// Create a new adaptive layout builder.
    pub fn new(
        nav_bar: Element<'a, M, T>,
        content: Element<'a, M, T>,
        _breakpoint: Breakpoint,
    ) -> Self {
        Self { nav_bar, content }
    }

    /// Build the adaptive layout.
    pub fn build(self) -> Element<'a, M, T> {
        Column::new()
            .push(self.nav_bar)
            .push(self.content)
            .spacing(Spacing::DEFAULT.xs2)
            .into()
    }
}

/// Create an adaptive layout with nav bar and content.
pub fn adaptive_layout<'a, M: 'a, T: 'a>(
    nav_bar: Element<'a, M, T>,
    content: Element<'a, M, T>,
    breakpoint: Breakpoint,
) -> Element<'a, M, T> {
    AdaptiveLayout::new(nav_bar, content, breakpoint).build()
}
