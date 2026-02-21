//! Settings page update logic.

use super::message::SettingsMessage;
use crate::state::Model;
use iced::Task;
use std::sync::Arc;

/// Handles settings messages.
pub fn update(message: SettingsMessage, model: &mut Model) -> Task<SettingsMessage> {
    match message {
        SettingsMessage::LogLevelChanged(level) => {
            Arc::get_mut(&mut model.app_config).map(|c| {
                c.log_level = level;
                c.save_to_file();
            });
            Task::none()
        }

        SettingsMessage::AddProvider => {
            // TODO: Add provider form handling
            Task::none()
        }

        SettingsMessage::EditProvider(_id) => {
            // TODO: Implement provider editing
            Task::none()
        }

        SettingsMessage::DeleteProvider(id) => {
            if let Some(config) = Arc::get_mut(&mut model.app_config) {
                config.ai.providers.retain(|p| p.id != id);
                // Also remove models associated with this provider
                config.ai.models.retain(|m| m.provider_id != id);
                config.save_to_file();
            }
            Task::none()
        }

        SettingsMessage::SaveProvider => {
            // TODO: Implement provider saving
            Task::none()
        }

        SettingsMessage::CancelEditProvider => {
            // TODO: Cancel provider editing
            Task::none()
        }

        SettingsMessage::ProviderNameChanged(_name) => {
            // TODO: Handle name change
            Task::none()
        }

        SettingsMessage::ProviderTypeChanged(_type) => {
            // TODO: Handle type change
            Task::none()
        }

        SettingsMessage::ProviderBaseUrlChanged(_url) => {
            // TODO: Handle URL change
            Task::none()
        }

        SettingsMessage::ProviderApiKeyChanged(_key) => {
            // TODO: Handle API key change
            Task::none()
        }

        SettingsMessage::AddModel => {
            // TODO: Add model form handling
            Task::none()
        }

        SettingsMessage::EditModel(_id) => {
            // TODO: Implement model editing
            Task::none()
        }

        SettingsMessage::DeleteModel(id) => {
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

        SettingsMessage::SaveModel => {
            // TODO: Implement model saving
            Task::none()
        }

        SettingsMessage::CancelEditModel => {
            // TODO: Cancel model editing
            Task::none()
        }

        SettingsMessage::ModelNameChanged(_name) => {
            // TODO: Handle name change
            Task::none()
        }

        SettingsMessage::ModelProviderIdChanged(_provider_id) => {
            // TODO: Handle provider ID change
            Task::none()
        }

        SettingsMessage::ModelModelIdChanged(_model_id) => {
            // TODO: Handle model ID change
            Task::none()
        }

        SettingsMessage::SelectModel(id) => {
            if let Some(config) = Arc::get_mut(&mut model.app_config) {
                config.ai.selected_model_id = Some(id);
                // Reload generator with new config
                model.generator.load_from_config(&config.ai);
                config.save_to_file();
            }
            Task::none()
        }
    }
}
