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

pub fn default(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    let is_selected = matches!(
        status,
        Status::Active { is_selected: true } | Status::Hovered { is_selected: true }
    );

    let (dot_color, border_color) = if is_selected {
        (semantic.interactive.primary, semantic.interactive.primary)
    } else if matches!(status, Status::Hovered { .. }) {
        (
            semantic.interactive.primary_hover,
            semantic.interactive.primary_hover,
        )
    } else {
        (semantic.surface.raised, semantic.border.default)
    };

    iced::widget::radio::Style {
        background: semantic.surface.raised.into(),
        dot_color,
        border_width: 1.0,
        border_color,
        text_color: Some(semantic.text.primary),
    }
}
