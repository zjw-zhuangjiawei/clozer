//! Settings window state.

use crate::ui::settings_window::SettingsMessage;

/// State for the settings window.
#[derive(Debug, Default)]
pub struct SettingsUiState {
    // Placeholder for future settings state
    _marker: std::marker::PhantomData<SettingsMessage>,
}

impl SettingsUiState {
    /// Creates a new SettingsUiState.
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
