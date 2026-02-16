//! Reusable SVG checkbox button component.

use iced::Element;
use iced::widget::{Button, button, svg};

/// Creates an SVG checkbox button that toggles between checked and unchecked states.
pub fn svg_checkbox<'a, M: Clone + 'a>(selected: bool, on_toggle: M) -> Element<'a, M> {
    let icon_path = if selected {
        "assets/icon/check_box_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    } else {
        "assets/icon/check_box_outline_blank_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    };

    Button::new(
        svg(icon_path)
            .width(iced::Length::Fixed(20.0))
            .height(iced::Length::Fixed(20.0)),
    )
    .style(button::secondary)
    .padding([2, 6])
    .on_press(on_toggle)
    .width(iced::Length::Fixed(30.0))
    .into()
}
