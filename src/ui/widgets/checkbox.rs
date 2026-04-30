//! Reusable SVG checkbox button component and iced Checkbox Catalog.

use crate::ui::theme::AppTheme;
use iced::Element;
use iced::widget::checkbox::{Catalog, Status, Style, StyleFn};
use iced::widget::{Button, svg};

use crate::assets;
use crate::ui::widgets::button;

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

    let is_checked = matches!(
        status,
        Status::Active { is_checked: true }
            | Status::Hovered { is_checked: true }
            | Status::Disabled { is_checked: true }
    );
    let is_disabled = matches!(status, Status::Disabled { .. });
    let is_hovered = matches!(status, Status::Hovered { .. });

    let (background, border_color) = if is_checked {
        if is_hovered {
            (
                semantic.interactive.primary_hover.into(),
                semantic.interactive.primary_hover,
            )
        } else {
            (
                semantic.interactive.primary.into(),
                semantic.interactive.primary,
            )
        }
    } else if is_disabled {
        (semantic.surface.base.into(), semantic.border.disabled)
    } else if is_hovered {
        (semantic.surface.raised.into(), semantic.border.hover)
    } else {
        (semantic.surface.base.into(), semantic.border.default)
    };

    Style {
        background,
        border: iced::Border {
            color: border_color,
            width: 1.0,
            radius: 4.0.into(),
        },
        icon_color: if is_checked {
            semantic.text.inverse
        } else {
            semantic.text.primary
        },
        text_color: Some(semantic.text.primary),
    }
}

/// Checkbox selection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxState {
    Unchecked,
    Checked,
    Indeterminate,
}

impl From<bool> for CheckboxState {
    fn from(checked: bool) -> Self {
        if checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        }
    }
}

/// Creates an SVG checkbox button with the given state.
pub fn svg_checkbox<'a, M: Clone + 'a>(
    state: impl Into<CheckboxState>,
    on_toggle: M,
) -> Element<'a, M, AppTheme> {
    let icon_name = match state.into() {
        CheckboxState::Checked => "check_box_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg",
        CheckboxState::Unchecked => {
            "check_box_outline_blank_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
        }
        CheckboxState::Indeterminate => {
            "indeterminate_check_box_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
        }
    };

    let handle = assets::get_svg(icon_name)
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));

    Button::new(
        svg(handle)
            .width(iced::Length::Fixed(20.0))
            .height(iced::Length::Fixed(20.0)),
    )
    .style(button::tertiary)
    .padding([2, 6])
    .on_press(on_toggle)
    .width(iced::Length::Fixed(30.0))
    .into()
}
