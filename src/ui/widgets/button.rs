use crate::ui::theme::AppTheme;
use crate::ui::theme::BorderRadiusValues;
use iced::Color;
use iced::theme::Base as ThemeBase;
use iced::theme::Mode;
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

fn base_style(bg: Color, text: Color, border: Color) -> Style {
    Style {
        background: Some(bg.into()),
        border: iced::Border {
            color: border,
            width: 1.0,
            radius: BorderRadiusValues::default().sm.into(),
        },
        text_color: text,
        ..Default::default()
    }
}

pub fn primary(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    let (bg, text, border) = match status {
        Status::Disabled => (
            semantic.interactive.primary,
            semantic.text.disabled,
            semantic.border.disabled,
        ),
        Status::Pressed => (
            semantic.interactive.primary_hover,
            semantic.text.inverse,
            semantic.interactive.primary_hover,
        ),
        Status::Hovered => (
            semantic.interactive.primary_hover,
            semantic.text.inverse,
            semantic.interactive.primary_hover,
        ),
        _ => (
            semantic.interactive.primary,
            semantic.text.inverse,
            semantic.interactive.primary,
        ),
    };

    base_style(bg, text, border)
}

pub fn secondary(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;
    let mode = theme.mode();

    let text = match status {
        Status::Disabled => semantic.text.disabled,
        _ => match mode {
            Mode::Light => semantic.text.primary,
            Mode::Dark => semantic.text.inverse,
            _ => semantic.text.primary,
        },
    };

    let (bg, border) = match status {
        Status::Disabled => (semantic.interactive.secondary, semantic.border.disabled),
        Status::Pressed => (
            semantic.interactive.secondary_hover,
            semantic.interactive.primary,
        ),
        Status::Hovered => (
            semantic.interactive.secondary_hover,
            semantic.interactive.primary_hover,
        ),
        _ => (semantic.interactive.secondary, semantic.interactive.primary),
    };

    base_style(bg, text, border)
}

pub fn tertiary(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    let text = match status {
        Status::Disabled => semantic.text.disabled,
        _ => semantic.text.link,
    };

    Style {
        background: Some(Color::TRANSPARENT.into()),
        border: iced::Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        text_color: text,
        ..Default::default()
    }
}

pub fn danger(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    let (bg, text, border) = match status {
        Status::Disabled => (
            semantic.interactive.danger,
            semantic.text.disabled,
            semantic.border.disabled,
        ),
        Status::Pressed => (
            semantic.interactive.danger_hover,
            semantic.text.inverse,
            semantic.interactive.danger_hover,
        ),
        Status::Hovered => (
            semantic.interactive.danger_hover,
            semantic.text.inverse,
            semantic.interactive.danger_hover,
        ),
        _ => (
            semantic.interactive.danger,
            semantic.text.inverse,
            semantic.interactive.danger,
        ),
    };

    base_style(bg, text, border)
}
