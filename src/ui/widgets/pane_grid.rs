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

pub fn default(theme: &AppTheme) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    Style {
        hovered_region: iced::widget::pane_grid::Highlight {
            background: semantic.interactive.secondary.into(),
            border: iced::Border {
                color: semantic.interactive.primary,
                width: 1.0,
                radius: 0.0.into(),
            },
        },
        picked_split: iced::widget::pane_grid::Line {
            color: semantic.border.default,
            width: 1.0,
        },
        hovered_split: iced::widget::pane_grid::Line {
            color: semantic.border.hover,
            width: 1.0,
        },
    }
}
