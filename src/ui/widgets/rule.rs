use crate::ui::theme::AppTheme;
use iced::widget::rule::{Catalog, FillMode, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

/// The default styling of a [`Rule`].
pub fn default(theme: &AppTheme) -> Style {
    let colors = theme.colors();

    Style {
        color: colors.semantic.border.default,
        radius: 0.0.into(),
        fill_mode: FillMode::Full,
        snap: true,
    }
}
