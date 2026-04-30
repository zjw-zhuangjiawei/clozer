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
    let colors = theme.colors();
    let semantic = &colors.semantic;

    iced::widget::overlay::menu::Style {
        background: semantic.surface.elevated.into(),
        border: iced::Border {
            color: semantic.border.default,
            width: 1.0,
            radius: 4.0.into(),
        },
        text_color: semantic.text.primary,
        selected_text_color: semantic.interactive.primary,
        selected_background: semantic.interactive.secondary.into(),
        shadow: iced::Shadow::default(),
    }
}
