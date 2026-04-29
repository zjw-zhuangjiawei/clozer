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

pub fn primary(_theme: &AppTheme) -> Style {
    iced::widget::progress_bar::primary(&iced::Theme::Light)
}

#[allow(dead_code)]
fn styled(background: impl Into<Background>, bar: impl Into<Background>) -> Style {
    Style {
        background: background.into(),
        bar: bar.into(),
        border: border::rounded(2),
    }
}
