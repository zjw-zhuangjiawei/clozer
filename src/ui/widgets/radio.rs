use crate::ui::theme::AppTheme;
use iced::widget::radio::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(_theme: &AppTheme, status: Status) -> Style {
    iced::widget::radio::default(&iced::Theme::Light, status)
}
