//! Adaptive layout implementation.

use crate::ui::theme::Breakpoint;
use iced::widget::{Column, Container, Row};
use iced::{Element, FillPortion};

/// Adaptive layout builder for responsive designs.
pub struct AdaptiveLayout<'a, M> {
    nav_bar: Element<'a, M>,
    content: Element<'a, M>,
    left_ratio: f32,
}

impl<'a, M: 'a> AdaptiveLayout<'a, M> {
    /// Create a new adaptive layout builder.
    pub fn new(nav_bar: Element<'a, M>, content: Element<'a, M>, breakpoint: Breakpoint) -> Self {
        let left_ratio = breakpoint.column_ratio().0;
        Self {
            nav_bar,
            content,
            left_ratio,
        }
    }

    /// Build the adaptive layout.
    pub fn build(self) -> Element<'a, M> {
        if self.left_ratio == 0.0 {
            // Single column layout
            Column::new().push(self.nav_bar).push(self.content).into()
        } else {
            // Master-detail layout
            let left_panel = Container::new(self.content)
                .width(FillPortion((self.left_ratio * 100.0) as u16))
                .padding(20);

            Row::new()
                .push(self.nav_bar)
                .push(left_panel)
                .spacing(5)
                .into()
        }
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
