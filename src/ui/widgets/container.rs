use crate::ui::design_tokens::BorderRadiusValues;
use crate::ui::theme::AppTheme;
use iced::widget::container::{Catalog, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|_theme| Style::default())
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// Raised card — background + 1px border + md radius.
pub fn card(theme: &AppTheme) -> Style {
    let semantic = &theme.colors().semantic;
    Style {
        background: Some(semantic.surface.raised.into()),
        border: iced::Border {
            color: semantic.border.default,
            width: 1.0,
            radius: BorderRadiusValues::default().lg.into(),
        },
        ..Default::default()
    }
}

/// Compact badge — elevated surface + 1px border + sm radius.
pub fn badge(theme: &AppTheme) -> Style {
    let semantic = &theme.colors().semantic;
    Style {
        background: Some(semantic.surface.elevated.into()),
        border: iced::Border {
            color: semantic.border.default,
            width: 1.0,
            radius: BorderRadiusValues::default().sm.into(),
        },
        ..Default::default()
    }
}
