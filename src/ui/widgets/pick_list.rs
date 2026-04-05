use crate::ui::theme::AppTheme;
use iced::Color;
use iced::widget::pick_list::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

fn base_style(
    bg: Color,
    text: Color,
    placeholder: Color,
    handle: Color,
    border_color: Color,
) -> Style {
    Style {
        background: bg.into(),
        text_color: text,
        placeholder_color: placeholder,
        handle_color: handle,
        border: iced::Border {
            color: border_color,
            width: 1.0,
            radius: 2.0.into(),
        },
    }
}

pub fn primary(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    match status {
        Status::Opened { is_hovered: true } => base_style(
            semantic.interactive.primary_hover,
            semantic.text.inverse,
            semantic.text.disabled,
            semantic.text.inverse,
            semantic.interactive.primary_hover,
        ),
        Status::Hovered => base_style(
            semantic.interactive.primary_hover,
            semantic.text.inverse,
            semantic.text.disabled,
            semantic.text.inverse,
            semantic.interactive.primary_hover,
        ),
        _ => base_style(
            semantic.interactive.primary,
            semantic.text.inverse,
            semantic.text.disabled,
            semantic.text.inverse,
            semantic.interactive.primary,
        ),
    }
}

pub fn secondary(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    match status {
        Status::Opened { is_hovered: true } => base_style(
            semantic.interactive.secondary_hover,
            semantic.text.primary,
            semantic.text.secondary,
            semantic.text.secondary,
            semantic.interactive.primary_hover,
        ),
        Status::Hovered => base_style(
            semantic.interactive.secondary_hover,
            semantic.text.primary,
            semantic.text.secondary,
            semantic.text.secondary,
            semantic.interactive.primary_hover,
        ),
        _ => base_style(
            semantic.interactive.secondary,
            semantic.text.primary,
            semantic.text.secondary,
            semantic.text.secondary,
            semantic.interactive.primary,
        ),
    }
}
