//! Settings window messages.

use iced::window::Id;

/// Messages for the settings window.
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    // Placeholder for future messages
    Close,
}

impl SettingsMessage {
    /// Convert to app-level message for window close.
    pub fn to_app_message(self, window_id: Id) -> crate::Message {
        match self {
            SettingsMessage::Close => crate::Message::WindowCloseRequested(window_id),
        }
    }
}
