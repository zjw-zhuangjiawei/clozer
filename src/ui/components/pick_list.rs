use crate::ui::theme::AppTheme;
use iced::widget::pick_list::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> StyleFn<'a, Self> {
        Box::new(default)
    }

    fn style(&self, class: &StyleFn<'_, Self>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(_theme: &AppTheme, status: Status) -> Style {
    iced::widget::pick_list::default(&iced::Theme::Light, status)
}
