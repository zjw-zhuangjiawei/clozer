use crate::state::Model;
use crate::ui::settings::SettingsState;
use crate::ui::settings::message::{
    GeneralSettingsMessage, ModelMessage, ProviderMessage, SettingsMessage,
};
use iced::Task;
use std::sync::Arc;
use uuid::Uuid;

/// Handle general settings messages.
pub fn general(
    _state: &mut SettingsState,
    message: GeneralSettingsMessage,
    model: &mut Model,
) -> Task<SettingsMessage> {
    match message {
        GeneralSettingsMessage::LogLevelChanged(level) => {
            if let Some(c) = Arc::get_mut(&mut model.app_config) {
                c.log_level = level;
                c.save_to_file();
            }
        }
    }
    Task::none()
}

/// Handle provider messages.
pub fn provider(
    state: &mut SettingsState,
    message: ProviderMessage,
    model: &mut Model,
) -> Task<SettingsMessage> {
    match message {
        ProviderMessage::Add => {
            state.provider_edit = crate::ui::settings::state::ProviderEditState::start_new();
        }
        ProviderMessage::Edit(id) => {
            let uuid = Uuid::from(id);
            if let Some(config) = Arc::get_mut(&mut model.app_config)
                && let Some(provider) = config.ai.providers.iter().find(|p| p.id == uuid)
            {
                state.provider_edit =
                    crate::ui::settings::state::ProviderEditState::start_edit(id, provider.clone());
            }
        }
        ProviderMessage::Delete(id) => {
            let uuid = Uuid::from(id);
            if let Some(config) = Arc::get_mut(&mut model.app_config) {
                config.ai.providers.retain(|p| p.id != uuid);
                config.ai.models.retain(|m| m.provider_id != uuid);
                config.save_to_file();
            }
        }
        ProviderMessage::Save => {
            if let Some(config) = Arc::get_mut(&mut model.app_config) {
                let is_new = state.provider_edit.is_new;
                let provider = state.provider_edit.data.clone();

                if is_new {
                    config.ai.providers.push(provider);
                } else if let Some(editing_id) = state.provider_edit.editing_id
                    && let Some(existing) = config
                        .ai
                        .providers
                        .iter_mut()
                        .find(|p| p.id == editing_id.0)
                {
                    *existing = provider;
                }
                config.save_to_file();
            }
            state.provider_edit.cancel();
        }
        ProviderMessage::Cancel => {
            state.provider_edit.cancel();
        }
        ProviderMessage::NameChanged(name) => {
            state.provider_edit.data.name = name;
        }
        ProviderMessage::TypeChanged(typ) => {
            state.provider_edit.data.provider_type = typ;
        }
        ProviderMessage::BaseUrlChanged(url) => {
            state.provider_edit.data.base_url = Some(url);
        }
        ProviderMessage::ApiKeyChanged(key) => {
            state.provider_edit.data.api_key = Some(key);
        }
    }
    Task::none()
}

/// Handle model messages.
pub fn model_handler(
    state: &mut SettingsState,
    message: ModelMessage,
    model: &mut Model,
) -> Task<SettingsMessage> {
    match message {
        ModelMessage::Add => {
            state.model_edit = crate::ui::settings::state::ModelEditState::start_new();
        }
        ModelMessage::Edit(id) => {
            let uuid = Uuid::from(id);
            if let Some(config) = Arc::get_mut(&mut model.app_config)
                && let Some(m) = config.ai.models.iter().find(|m| m.id == uuid)
            {
                state.model_edit =
                    crate::ui::settings::state::ModelEditState::start_edit(id, m.clone());
            }
        }
        ModelMessage::Delete(id) => {
            let uuid = Uuid::from(id);
            if let Some(config) = Arc::get_mut(&mut model.app_config) {
                config.ai.models.retain(|m| m.id != uuid);
                if config.ai.selected_model_id == Some(uuid) {
                    config.ai.selected_model_id = None;
                }
                config.save_to_file();
            }
        }
        ModelMessage::Save => {
            if let Some(config) = Arc::get_mut(&mut model.app_config) {
                let is_new = state.model_edit.is_new;
                let model_config = state.model_edit.data.clone();

                if is_new {
                    config.ai.models.push(model_config);
                } else if let Some(editing_id) = state.model_edit.editing_id
                    && let Some(existing) =
                        config.ai.models.iter_mut().find(|m| m.id == editing_id.0)
                {
                    *existing = model_config;
                }
                config.save_to_file();
            }
            state.model_edit.cancel();
        }
        ModelMessage::Cancel => {
            state.model_edit.cancel();
        }
        ModelMessage::NameChanged(name) => {
            state.model_edit.data.name = name;
        }
        ModelMessage::ProviderIdChanged(provider_id) => {
            state.model_edit.data.provider_id = provider_id.0;
        }
        ModelMessage::ModelIdChanged(model_id) => {
            state.model_edit.data.model_id = model_id;
        }
        ModelMessage::Select(id) => {
            let uuid = Uuid::from(id);
            if let Some(config) = Arc::get_mut(&mut model.app_config) {
                config.ai.selected_model_id = Some(uuid);
                model.generator.load_from_config(&config.ai);
                config.save_to_file();
            }
        }
    }
    Task::none()
}

/// Handle all settings-related messages.
pub fn update(
    state: &mut SettingsState,
    message: SettingsMessage,
    model: &mut Model,
) -> Task<SettingsMessage> {
    use SettingsMessage::*;
    match message {
        General(msg) => general(state, msg, model),
        Provider(msg) => provider(state, msg, model),
        Model(msg) => model_handler(state, msg, model),
        ThemeChanged(_) => Task::none(),
    }
}
