//! Settings page messages.

use crate::config::LogLevel;
use crate::config::file::ai::ProviderTypeDto;
use uuid::Uuid;

/// Messages for the settings embedded page.
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    // General settings
    LogLevelChanged(LogLevel),

    // AI Provider operations
    AddProvider,
    EditProvider(Uuid),
    DeleteProvider(Uuid),
    ProviderNameChanged(String),
    ProviderTypeChanged(ProviderTypeDto),
    ProviderBaseUrlChanged(String),
    ProviderApiKeyChanged(String),
    SaveProvider,
    CancelEditProvider,

    // AI Model operations
    AddModel,
    EditModel(Uuid),
    DeleteModel(Uuid),
    ModelNameChanged(String),
    ModelProviderIdChanged(Uuid),
    ModelModelIdChanged(String),
    SaveModel,
    CancelEditModel,

    // Model selection
    SelectModel(Uuid),
}
