use crate::ui::theme::AppTheme;
use iced::Background;
use iced::border;
use iced::widget::progress_bar::{Catalog, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn primary(theme: &AppTheme) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    Style {
        background: semantic.surface.raised.into(),
        bar: semantic.interactive.primary.into(),
        border: iced::border::rounded(2),
    }
}

#[allow(dead_code)]
fn styled(background: impl Into<Background>, bar: impl Into<Background>) -> Style {
    Style {
        background: background.into(),
        bar: bar.into(),
        border: border::rounded(2),
    }
}
