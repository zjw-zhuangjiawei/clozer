use crate::ui::theme::AppTheme;
use iced::widget::slider::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &AppTheme, _status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    iced::widget::slider::Style {
        rail: iced::widget::slider::Rail {
            backgrounds: (semantic.surface.raised.into(), semantic.surface.base.into()),
            width: 2.0,
            border: iced::Border::default(),
        },
        handle: iced::widget::slider::Handle {
            shape: iced::widget::slider::HandleShape::Circle { radius: 5.0 },
            background: semantic.interactive.primary.into(),
            border_width: 0.0,
            border_color: semantic.interactive.primary_hover,
        },
    }
}
