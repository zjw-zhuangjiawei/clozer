//! Settings page update logic.

use super::message::{GeneralSettingsMessage, ModelMessage, ProviderMessage, SettingsMessage};
use crate::state::Model;
use iced::Task;
use std::sync::Arc;

/// Handles settings messages.
pub fn update(message: SettingsMessage, model: &mut Model) -> Task<SettingsMessage> {
    match message {
        SettingsMessage::General(msg) => match msg {
            GeneralSettingsMessage::LogLevelChanged(level) => {
                Arc::get_mut(&mut model.app_config).map(|c| {
                    c.log_level = level;
                    c.save_to_file();
                });
                Task::none()
            }
        },

        SettingsMessage::Provider(msg) => match msg {
            ProviderMessage::Add => {
                // TODO: Add provider form handling
                Task::none()
            }

            ProviderMessage::Edit(_id) => {
                // TODO: Implement provider editing
                Task::none()
            }

            ProviderMessage::Delete(id) => {
                if let Some(config) = Arc::get_mut(&mut model.app_config) {
                    config.ai.providers.retain(|p| p.id != id);
                    // Also remove models associated with this provider
                    config.ai.models.retain(|m| m.provider_id != id);
                    config.save_to_file();
                }
                Task::none()
            }

            ProviderMessage::Save => {
                // TODO: Implement provider saving
                Task::none()
            }

            ProviderMessage::Cancel => {
                // TODO: Cancel provider editing
                Task::none()
            }

            ProviderMessage::NameChanged(_name) => {
                // TODO: Handle name change
                Task::none()
            }

            ProviderMessage::TypeChanged(_type) => {
                // TODO: Handle type change
                Task::none()
            }

            ProviderMessage::BaseUrlChanged(_url) => {
                // TODO: Handle URL change
                Task::none()
            }

            ProviderMessage::ApiKeyChanged(_key) => {
                // TODO: Handle API key change
                Task::none()
            }
        },

        SettingsMessage::Model(msg) => match msg {
            ModelMessage::Add => {
                // TODO: Add model form handling
                Task::none()
            }

            ModelMessage::Edit(_id) => {
                // TODO: Implement model editing
                Task::none()
            }

            ModelMessage::Delete(id) => {
                if let Some(config) = Arc::get_mut(&mut model.app_config) {
                    config.ai.models.retain(|m| m.id != id);
                    // Clear selection if deleted model was selected
                    if config.ai.selected_model_id == Some(id) {
                        config.ai.selected_model_id = None;
                    }
                    config.save_to_file();
                }
                Task::none()
            }

            ModelMessage::Save => {
                // TODO: Implement model saving
                Task::none()
            }

            ModelMessage::Cancel => {
                // TODO: Cancel model editing
                Task::none()
            }

            ModelMessage::NameChanged(_name) => {
                // TODO: Handle name change
                Task::none()
            }

            ModelMessage::ProviderIdChanged(_provider_id) => {
                // TODO: Handle provider ID change
                Task::none()
            }

            ModelMessage::ModelIdChanged(_model_id) => {
                // TODO: Handle model ID change
                Task::none()
            }

            ModelMessage::Select(id) => {
                if let Some(config) = Arc::get_mut(&mut model.app_config) {
                    config.ai.selected_model_id = Some(id);
                    // Reload generator with new config
                    model.generator.load_from_config(&config.ai);
                    config.save_to_file();
                }
                Task::none()
            }
        },
    }
}
