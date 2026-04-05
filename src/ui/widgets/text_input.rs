use crate::ui::theme::AppTheme;
use iced::widget::text_input::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// The default style of a [`TextInput`].
pub fn default(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;
    let border_radius = iced::border::radius(6.0);

    match status {
        Status::Disabled => Style {
            background: semantic.surface.base.into(),
            border: iced::Border {
                color: semantic.border.disabled,
                width: 1.0,
                radius: border_radius,
            },
            icon: semantic.text.disabled,
            placeholder: semantic.text.disabled,
            value: semantic.text.disabled,
            selection: semantic.interactive.primary,
        },
        Status::Focused { is_hovered: _ } => Style {
            background: semantic.surface.raised.into(),
            border: iced::Border {
                color: semantic.interactive.primary,
                width: 2.0,
                radius: border_radius,
            },
            icon: semantic.text.tertiary,
            placeholder: semantic.text.tertiary,
            value: semantic.text.primary,
            selection: semantic.interactive.primary,
        },
        Status::Hovered => Style {
            background: semantic.surface.raised.into(),
            border: iced::Border {
                color: semantic.border.hover,
                width: 1.0,
                radius: border_radius,
            },
            icon: semantic.text.tertiary,
            placeholder: semantic.text.tertiary,
            value: semantic.text.primary,
            selection: semantic.interactive.primary,
        },
        Status::Active => Style {
            background: semantic.surface.raised.into(),
            border: iced::Border {
                color: semantic.border.default,
                width: 1.0,
                radius: border_radius,
            },
            icon: semantic.text.tertiary,
            placeholder: semantic.text.tertiary,
            value: semantic.text.primary,
            selection: semantic.interactive.primary,
        },
    }
}
