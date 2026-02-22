//! Settings page state.

use crate::config::file::ai::{ModelConfig, ProviderConfig};

/// State for the embedded settings page.
#[derive(Debug, Default)]
pub struct SettingsUiState {
    // Editing state for providers
    pub editing_provider: Option<ProviderConfig>,
    pub is_adding_provider: bool,

    // Editing state for models
    pub editing_model: Option<ModelConfig>,
    pub is_adding_model: bool,
}

impl SettingsUiState {
    /// Creates a new SettingsUiState.
    pub fn new() -> Self {
        Self::default()
    }
}
