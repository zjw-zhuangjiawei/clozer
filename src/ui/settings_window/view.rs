//! Settings window view.

use iced::Element;
use iced::widget::{button, column, text};

use super::message::SettingsMessage;
use super::state::SettingsUiState;

/// Renders the settings window.
pub fn view(_state: &SettingsUiState) -> Element<'_, SettingsMessage> {
    column![
        text("Settings"),
        button("Close").on_press(SettingsMessage::Close),
    ]
    .spacing(20)
    .padding(20)
    .into()
}
