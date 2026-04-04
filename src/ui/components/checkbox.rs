//! Reusable SVG checkbox button component and iced Checkbox Catalog.

use crate::ui::theme::AppTheme;
use iced::Element;
use iced::widget::checkbox::{Catalog, Status, Style, StyleFn};
use iced::widget::{Button, svg};

use crate::assets;
use crate::ui::components::button;

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
    // let colors = theme.colors();
    // let is_checked = status.is_checked();
    // let is_hovered = matches!(status, Status::Hovered { .. });
    // let is_focused = matches!(status, Status::Focused { .. });

    // let (background, border_color) = if is_checked {
    //     if is_hovered {
    //         (colors.primary_hover, colors.primary_active)
    //     } else {
    //         (colors.primary, colors.primary_active)
    //     }
    // } else if is_hovered {
    //     (colors.neutral_100, colors.border_emphasis)
    // } else {
    //     (colors.surface, colors.border)
    // };

    // Style {
    //     background: Some(background.into()),
    //     border: iced::Border {
    //         color: if is_checked {
    //             colors.primary_active
    //         } else {
    //             border_color
    //         },
    //         width: if is_focused { 2.0 } else { 1.0 },
    //         radius: 4.0.into(),
    //     },
    //     icon_color: colors.text_on_primary,
    //     text_color: Some(colors.text),
    // }

    iced::widget::checkbox::primary(&iced::Theme::Light, status)
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
    theme: AppTheme,
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
