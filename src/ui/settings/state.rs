//! Settings panel state types.
//!
//! State is organized with focused sub-states:
//! - ProviderEditState: Provider editing state
//! - ModelEditState: Model editing state

use crate::config::file::ai::{ModelConfig, ProviderConfig};
use uuid::Uuid;

// ============================================================================
// Sub-states
// ============================================================================

/// Editing state for providers.
#[derive(Debug, Clone)]
pub struct ProviderEditState {
    /// Currently editing provider ID (None = adding new)
    pub editing_id: Option<Uuid>,
    /// Editing provider data
    pub data: ProviderConfig,
    /// Whether this is a new provider
    pub is_new: bool,
}

impl Default for ProviderEditState {
    fn default() -> Self {
        Self {
            editing_id: None,
            data: ProviderConfig {
                id: Uuid::new_v4(),
                name: String::new(),
                provider_type: crate::config::file::ai::ProviderTypeDto::OpenAI,
                base_url: None,
                api_key: None,
            },
            is_new: false,
        }
    }
}

impl ProviderEditState {
    /// Start adding a new provider.
    pub fn start_new() -> Self {
        Self {
            editing_id: None,
            data: ProviderConfig {
                id: Uuid::new_v4(),
                name: String::new(),
                provider_type: crate::config::file::ai::ProviderTypeDto::OpenAI,
                base_url: None,
                api_key: None,
            },
            is_new: true,
        }
    }

    /// Start editing an existing provider.
    pub fn start_edit(id: Uuid, data: ProviderConfig) -> Self {
        Self {
            editing_id: Some(id),
            data,
            is_new: false,
        }
    }

    /// Check if currently editing.
    pub fn is_editing(&self) -> bool {
        self.editing_id.is_some()
    }

    /// Cancel editing.
    pub fn cancel(&mut self) {
        self.editing_id = None;
        self.data = ProviderConfig {
            id: Uuid::new_v4(),
            name: String::new(),
            provider_type: crate::config::file::ai::ProviderTypeDto::OpenAI,
            base_url: None,
            api_key: None,
        };
        self.is_new = false;
    }
}

/// Editing state for models.
#[derive(Debug, Clone)]
pub struct ModelEditState {
    /// Currently editing model ID (None = adding new)
    pub editing_id: Option<Uuid>,
    /// Editing model data
    pub data: ModelConfig,
    /// Whether this is a new model
    pub is_new: bool,
}

impl Default for ModelEditState {
    fn default() -> Self {
        Self {
            editing_id: None,
            data: ModelConfig {
                id: Uuid::new_v4(),
                name: String::new(),
                provider_id: Uuid::nil(),
                model_id: String::new(),
            },
            is_new: false,
        }
    }
}

impl ModelEditState {
    /// Start adding a new model.
    pub fn start_new() -> Self {
        Self {
            editing_id: None,
            data: ModelConfig {
                id: Uuid::new_v4(),
                name: String::new(),
                provider_id: Uuid::nil(),
                model_id: String::new(),
            },
            is_new: true,
        }
    }

    /// Start editing an existing model.
    pub fn start_edit(id: Uuid, data: ModelConfig) -> Self {
        Self {
            editing_id: Some(id),
            data,
            is_new: false,
        }
    }

    /// Check if currently editing.
    pub fn is_editing(&self) -> bool {
        self.editing_id.is_some()
    }

    /// Cancel editing.
    pub fn cancel(&mut self) {
        self.editing_id = None;
        self.data = ModelConfig {
            id: Uuid::new_v4(),
            name: String::new(),
            provider_id: Uuid::nil(),
            model_id: String::new(),
        };
        self.is_new = false;
    }
}

// ============================================================================
// Main Panel State
// ============================================================================

/// Settings panel state.
#[derive(Debug, Default)]
pub struct SettingsState {
    /// Provider editing state
    pub provider_edit: ProviderEditState,
    /// Model editing state
    pub model_edit: ModelEditState,
}

/// Alias for backward compatibility.
pub type SettingsUiState = SettingsState;

impl SettingsState {
    /// Creates a new SettingsState.
    pub fn new() -> Self {
        Self::default()
    }
}
