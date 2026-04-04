use crate::ui::theme::AppTheme;
use iced::widget::overlay::menu::{Catalog, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>) -> Style {
        class(self)
    }
}

pub fn default(theme: &AppTheme) -> Style {
    iced::widget::overlay::menu::default(&iced::Theme::Light)
}
