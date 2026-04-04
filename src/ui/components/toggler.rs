use crate::ui::theme::AppTheme;
use iced::widget::toggler::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &AppTheme, status: Status) -> Style {
    iced::widget::toggler::default(&iced::Theme::Light, status)
}
