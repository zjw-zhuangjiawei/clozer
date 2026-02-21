//! Settings page state.

use crate::config::file::ai::{ModelConfig, ProviderConfig, ProviderTypeDto};

/// State for the embedded settings page.
#[derive(Debug, Default)]
pub struct SettingsUiState {
    // Editing state for providers
    pub editing_provider: Option<ProviderConfig>,
    pub is_adding_provider: bool,

    // Editing state for models
    pub editing_model: Option<ModelConfig>,
    pub is_adding_model: bool,

    // Form inputs for provider
    pub provider_name_input: String,
    pub provider_type_input: ProviderTypeDto,
    pub provider_base_url_input: String,
    pub provider_api_key_input: String,

    // Form inputs for model
    pub model_name_input: String,
    pub model_provider_id_input: Option<uuid::Uuid>,
    pub model_model_id_input: String,
}

impl SettingsUiState {
    /// Creates a new SettingsUiState.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset provider form inputs.
    pub fn reset_provider_form(&mut self) {
        self.provider_name_input.clear();
        self.provider_type_input = ProviderTypeDto::OpenAI;
        self.provider_base_url_input.clear();
        self.provider_api_key_input.clear();
        self.editing_provider = None;
        self.is_adding_provider = false;
    }

    /// Reset model form inputs.
    pub fn reset_model_form(&mut self) {
        self.model_name_input.clear();
        self.model_provider_id_input = None;
        self.model_model_id_input.clear();
        self.editing_model = None;
        self.is_adding_model = false;
    }
}
