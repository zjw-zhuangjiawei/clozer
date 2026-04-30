use crate::ui::theme::AppTheme;
use iced::widget::toggler::{Catalog, Status, Style, StyleFn};

impl Catalog for AppTheme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &AppTheme, status: Status) -> Style {
    let colors = theme.colors();
    let semantic = &colors.semantic;

    let is_toggled = matches!(
        status,
        Status::Active { is_toggled: true }
            | Status::Hovered { is_toggled: true }
            | Status::Disabled { is_toggled: true }
    );

    let (bg, fg) = if is_toggled {
        (
            semantic.interactive.primary.into(),
            semantic.text.inverse.into(),
        )
    } else {
        (semantic.surface.raised.into(), semantic.surface.base.into())
    };

    let disabled = matches!(status, Status::Disabled { .. });

    iced::widget::toggler::Style {
        background: bg,
        background_border_width: 1.0,
        background_border_color: if is_toggled {
            semantic.interactive.primary
        } else if disabled {
            semantic.border.disabled
        } else {
            semantic.border.default
        },
        foreground: fg,
        foreground_border_width: 1.0,
        foreground_border_color: if is_toggled {
            semantic.interactive.primary_hover
        } else if disabled {
            semantic.border.disabled
        } else {
            semantic.border.default
        },
        text_color: Some(semantic.text.primary),
        border_radius: None,
        padding_ratio: 0.22,
    }
}
