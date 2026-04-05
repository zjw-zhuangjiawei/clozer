use crate::ui::theme::AppTheme;
use iced::widget::pane_grid::{Catalog, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}

pub fn default(_theme: &AppTheme) -> Style {
    iced::widget::pane_grid::default(&iced::Theme::Light)
}
