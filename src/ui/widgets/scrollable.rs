use crate::ui::theme::AppTheme;
use iced::border;
use iced::widget::container;
use iced::widget::scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style, StyleFn};
use iced::{Color, Shadow, Vector};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`Scrollable`].
pub fn default(_theme: &AppTheme, status: Status) -> Style {
    iced::widget::scrollable::default(&iced::Theme::Light, status)
}
