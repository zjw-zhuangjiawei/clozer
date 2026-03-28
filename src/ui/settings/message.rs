//! Settings panel message types.
//!
//! Messages are organized hierarchically by domain:
//! - General: General settings
//! - Provider: AI provider CRUD
//! - Model: AI model CRUD and selection

use crate::config::LogLevel;
use crate::config::file::ai::ProviderTypeDto;
use uuid::Uuid;

/// Root message enum for Settings panel.
///
/// Delegates to domain-specific message handlers.
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    /// General settings messages
    General(GeneralSettingsMessage),
    /// AI Provider messages
    Provider(ProviderMessage),
    /// AI Model messages
    Model(ModelMessage),
}

/// General settings messages.
#[derive(Debug, Clone)]
pub enum GeneralSettingsMessage {
    /// Log level changed
    LogLevelChanged(LogLevel),
}

/// AI Provider messages.
#[derive(Debug, Clone)]
pub enum ProviderMessage {
    /// Start adding a new provider
    Add,
    /// Start editing an existing provider
    Edit(Uuid),
    /// Delete a provider
    Delete(Uuid),
    /// Save provider (new or edited)
    Save,
    /// Cancel provider editing
    Cancel,
    /// Provider name changed
    NameChanged(String),
    /// Provider type changed
    TypeChanged(ProviderTypeDto),
    /// Provider base URL changed
    BaseUrlChanged(String),
    /// Provider API key changed
    ApiKeyChanged(String),
}

/// AI Model messages.
#[derive(Debug, Clone)]
pub enum ModelMessage {
    /// Start adding a new model
    Add,
    /// Start editing an existing model
    Edit(Uuid),
    /// Delete a model
    Delete(Uuid),
    /// Save model (new or edited)
    Save,
    /// Cancel model editing
    Cancel,
    /// Model name changed
    NameChanged(String),
    /// Model provider ID changed
    ProviderIdChanged(Uuid),
    /// Model ID changed (from provider)
    ModelIdChanged(String),
    /// Select a model as active
    Select(Uuid),
}
