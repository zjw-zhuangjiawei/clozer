//! Reusable SVG checkbox button component.

use iced::Element;
use iced::widget::{Button, button, svg};

use crate::assets;

/// Checkbox selection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxState {
    /// Unchecked (empty)
    Unchecked,
    /// Checked (filled)
    Checked,
    /// Indeterminate (partial selection)
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
///
/// # Parameters
///
/// - `state`: The checkbox state (Checked, Unchecked, or Indeterminate)
/// - `on_toggle`: Message to emit when the checkbox is toggled
///
/// # Returns
///
/// An `Element` containing the checkbox button
pub fn svg_checkbox<'a, M: Clone + 'a>(
    state: impl Into<CheckboxState>,
    on_toggle: M,
) -> Element<'a, M> {
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
    .style(button::secondary)
    .padding([2, 6])
    .on_press(on_toggle)
    .width(iced::Length::Fixed(30.0))
    .into()
}
