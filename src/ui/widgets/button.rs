use crate::ui::theme::AppTheme;
use iced::Color;
use iced::widget::button::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn primary(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;
    let (bg, text) = match status {
        Status::Pressed => (semantic.interactive.primary_hover, semantic.text.inverse),
        Status::Hovered => (semantic.interactive.primary_hover, semantic.text.inverse),
        _ => (semantic.interactive.primary, semantic.text.inverse),
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

pub fn secondary(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;
    let (bg, border, text) = match status {
        Status::Pressed => (
            semantic.interactive.secondary_hover,
            semantic.interactive.primary,
            semantic.text.inverse,
        ),
        Status::Hovered => (
            semantic.interactive.secondary_hover,
            semantic.interactive.primary_hover,
            semantic.text.inverse,
        ),
        _ => (
            semantic.interactive.secondary,
            semantic.interactive.primary,
            semantic.text.inverse,
        ),
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

pub fn tertiary(theme: &AppTheme, _status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    Style {
        background: Some(Color::TRANSPARENT.into()),
        border: iced::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        text_color: semantic.text.link,
        ..Default::default()
    }
}

pub fn danger(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;
    let (bg, text) = match status {
        Status::Pressed => (semantic.interactive.danger_hover, semantic.text.inverse),
        Status::Hovered => (semantic.interactive.danger_hover, semantic.text.inverse),
        _ => (semantic.interactive.danger, semantic.text.inverse),
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
