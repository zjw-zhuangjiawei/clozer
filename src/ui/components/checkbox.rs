//! Reusable SVG checkbox button component.

use iced::Element;
use iced::widget::{Button, button, svg};

use crate::assets;

/// Creates an SVG checkbox button that toggles between checked and unchecked states.
///
/// # Parameters
///
/// - `selected`: Whether the checkbox is checked
/// - `on_toggle`: Message to emit when the checkbox is toggled
///
/// # Returns
///
/// An `Element` containing the checkbox button
pub fn svg_checkbox<'a, M: Clone + 'a>(selected: bool, on_toggle: M) -> Element<'a, M> {
    let icon_name = if selected {
        "check_box_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    } else {
        "check_box_outline_blank_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
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
