use crate::ui::theme::AppTheme;
use iced::Color;
use iced::widget::button::{Catalog, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: iced::widget::button::Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &AppTheme, status: iced::widget::button::Status) -> Style {
    let colors = theme.colors();
    let (bg, text) = match status {
        iced::widget::button::Status::Pressed => (colors.primary_active, colors.text_on_primary),
        iced::widget::button::Status::Hovered => (colors.primary_hover, colors.text_on_primary),
        _ => (colors.primary, colors.text_on_primary),
    };
    Style {
        background: Some(bg.into()),
        border: iced::Border {
            color: bg,
            width: 1.0,
            radius: 6.0.into(),
        },
        text_color: text,
        ..Default::default()
    }
}

pub fn secondary(theme: &AppTheme, status: iced::widget::button::Status) -> Style {
    let colors = theme.colors();
    let (bg, border, text) = match status {
        iced::widget::button::Status::Pressed => {
            (colors.neutral_200, colors.border_emphasis, colors.text)
        }
        iced::widget::button::Status::Hovered => {
            (colors.neutral_100, colors.border_emphasis, colors.text)
        }
        _ => (Color::TRANSPARENT, colors.border, colors.text),
    };
    Style {
        background: Some(bg.into()),
        border: iced::Border {
            color: border,
            width: 1.0,
            radius: 6.0.into(),
        },
        text_color: text,
        ..Default::default()
    }
}

pub fn tertiary(theme: &AppTheme, status: iced::widget::button::Status) -> Style {
    let colors = theme.colors();

    Style {
        background: Some(Color::TRANSPARENT.into()),
        border: iced::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        text_color: colors.text_secondary,
        ..Default::default()
    }
}

pub fn danger(theme: &AppTheme, status: iced::widget::button::Status) -> Style {
    let colors = theme.colors();
    let (bg, text) = match status {
        iced::widget::button::Status::Pressed => (colors.danger_active, colors.text_on_primary),
        iced::widget::button::Status::Hovered => (colors.danger_hover, colors.text_on_primary),
        _ => (colors.danger, colors.text_on_primary),
    };
    Style {
        background: Some(bg.into()),
        border: iced::Border {
            color: bg,
            width: 1.0,
            radius: 6.0.into(),
        },
        text_color: text,
        ..Default::default()
    }
}
