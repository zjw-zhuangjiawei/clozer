//! Adaptive layout implementation.

use crate::ui::theme::Breakpoint;
use iced::Element;
use iced::widget::Column;

/// Adaptive layout builder for responsive designs.
pub struct AdaptiveLayout<'a, M> {
    nav_bar: Element<'a, M>,
    content: Element<'a, M>,
}

impl<'a, M: 'a> AdaptiveLayout<'a, M> {
    /// Create a new adaptive layout builder.
    pub fn new(nav_bar: Element<'a, M>, content: Element<'a, M>, _breakpoint: Breakpoint) -> Self {
        Self { nav_bar, content }
    }

    /// Build the adaptive layout.
    pub fn build(self) -> Element<'a, M> {
        Column::new()
            .push(self.nav_bar)
            .push(self.content)
            .spacing(5)
            .into()
    }
}

/// Create an adaptive layout with nav bar and content.
pub fn adaptive_layout<'a, M: 'a>(
    nav_bar: Element<'a, M>,
    content: Element<'a, M>,
    breakpoint: Breakpoint,
) -> Element<'a, M> {
    AdaptiveLayout::new(nav_bar, content, breakpoint).build()
}
