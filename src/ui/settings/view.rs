use super::message::{ModelMessage, ProviderMessage, SettingsMessage};
use super::state::SettingsState;
use crate::config::file::ai::{AiConfig, ProviderTypeDto};
use crate::models::types::{ModelId, ProviderId};
use crate::state::Model;
use crate::ui::theme::{AppTheme, ButtonSize, FontSize, Spacing};
use iced::Element;
use iced::widget::{Button, Column, PickList, Row, TextInput, rule, scrollable, text};
use strum::VariantArray;
use uuid::Uuid;

use crate::ui::widgets::button;

struct ProviderOption {
    id: ProviderId,
    name: String,
}

impl std::fmt::Display for ProviderOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl PartialEq for ProviderOption {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Clone for ProviderOption {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

impl ProviderOption {
    fn from_config(p: &crate::config::file::ai::ProviderConfig) -> Self {
        Self {
            id: ProviderId::from(p.id),
            name: p.name.clone(),
        }
    }
}

pub fn view<'a>(
    state: &'a SettingsState,
    model: &'a Model,
) -> Element<'a, SettingsMessage, AppTheme> {
    let ai_config = &model.app_config.ai;

    let theme_picker = Row::new()
        .push(text("Theme:"))
        .push(
            PickList::new(
                AppTheme::VARIANTS,
                Some(model.app_config.theme),
                SettingsMessage::ThemeChanged,
            )
            .width(iced::Length::Fixed(120.0)),
        )
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    let log_level_row = Row::new()
        .push(text("Log Level:"))
        .push(text(format!("{:?}", model.app_config.log_level)))
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    let general_section = Column::new()
        .push(text("General").size(FontSize::Title.px()))
        .push(theme_picker)
        .push(log_level_row)
        .spacing(Spacing::DEFAULT.s);

    let providers_section =
        if state.provider_edit.is_new || state.provider_edit.editing_id.is_some() {
            render_provider_form(state)
        } else {
            render_provider_list(ai_config)
        };

    let models_section = if state.model_edit.is_new || state.model_edit.editing_id.is_some() {
        render_model_form(state, ai_config)
    } else {
        render_model_list(ai_config)
    };

    let (selected_name, _) = ai_config
        .selected_model_id
        .and_then(|id| ai_config.models.iter().find(|m| m.id == id))
        .map(|m| (m.name.clone(), true))
        .unwrap_or(("<none>".to_string(), false));

    let selected_model_section = Column::new()
        .push(text("Active Model").size(FontSize::Title.px()))
        .push(
            Row::new()
                .push(text("Selected: "))
                .push(text(selected_name))
                .spacing(Spacing::DEFAULT.s),
        )
        .spacing(Spacing::DEFAULT.s);

    let data_dir_section = Column::new()
        .push(text("Data").size(FontSize::Title.px()))
        .push(text(format!("Directory: {:?}", model.app_config.data_dir)))
        .spacing(Spacing::DEFAULT.s);

    let content = Column::new()
        .push(text("Settings").size(FontSize::Display.px()))
        .push(general_section)
        .push(rule::horizontal(1))
        .push(providers_section)
        .push(rule::horizontal(1))
        .push(models_section)
        .push(rule::horizontal(1))
        .push(selected_model_section)
        .push(rule::horizontal(1))
        .push(data_dir_section)
        .spacing(Spacing::DEFAULT.l)
        .padding(Spacing::DEFAULT.l);

    scrollable(content).into()
}

fn render_provider_list(ai_config: &AiConfig) -> Column<'static, SettingsMessage, AppTheme> {
    let items: Vec<Element<'static, SettingsMessage, AppTheme>> = ai_config
        .providers
        .iter()
        .map(|p| {
            let name = p.name.clone();
            let pt = format!("{:?}", p.provider_type);
            let edit_id = ProviderId::from(p.id);
            let delete_id = ProviderId::from(p.id);

            Row::new()
                .push(text(name).width(iced::Length::Fill))
                .push(text(pt))
                .push(
                    Button::new(text("Edit"))
                        .style(button::secondary)
                        .padding(ButtonSize::Small.to_iced_padding())
                        .on_press(SettingsMessage::Provider(ProviderMessage::Edit(edit_id))),
                )
                .push(
                    Button::new(text("Delete"))
                        .style(button::danger)
                        .padding(ButtonSize::Small.to_iced_padding())
                        .on_press(SettingsMessage::Provider(ProviderMessage::Delete(
                            delete_id,
                        ))),
                )
                .spacing(Spacing::DEFAULT.s)
                .align_y(iced::Alignment::Center)
                .into()
        })
        .collect();

    Column::new()
        .push(text("AI Providers").size(FontSize::Title.px()))
        .push(Column::with_children(items).spacing(Spacing::DEFAULT.xs))
        .push(
            Button::new(text("Add Provider"))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(SettingsMessage::Provider(ProviderMessage::Add)),
        )
        .spacing(Spacing::DEFAULT.s)
}

fn render_provider_form(state: &SettingsState) -> Column<'static, SettingsMessage, AppTheme> {
    let edit = &state.provider_edit;
    let title = if edit.is_new {
        "Add Provider"
    } else {
        "Edit Provider"
    };

    let name = edit.data.name.clone();
    let provider_type = edit.data.provider_type;
    let base_url = edit.data.base_url.clone().unwrap_or_default();
    let api_key = edit.data.api_key.clone().unwrap_or_default();

    let name_input = TextInput::new("Provider Name", &name)
        .on_input(|s| SettingsMessage::Provider(ProviderMessage::NameChanged(s)))
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    let type_picker = Row::new()
        .push(text("Type:"))
        .push(
            PickList::new(ProviderTypeDto::VARIANTS, Some(provider_type), |t| {
                SettingsMessage::Provider(ProviderMessage::TypeChanged(t))
            })
            .width(iced::Length::Fixed(140.0)),
        )
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    let base_url_input = TextInput::new("Base URL (optional)", &base_url)
        .on_input(|s| SettingsMessage::Provider(ProviderMessage::BaseUrlChanged(s)))
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    let api_key_input = TextInput::new("API Key", &api_key)
        .on_input(|s| SettingsMessage::Provider(ProviderMessage::ApiKeyChanged(s)))
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s)
        .secure(true);

    let buttons = Row::new()
        .push(
            Button::new(text("Save"))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(SettingsMessage::Provider(ProviderMessage::Save)),
        )
        .push(
            Button::new(text("Cancel"))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(SettingsMessage::Provider(ProviderMessage::Cancel)),
        )
        .spacing(Spacing::DEFAULT.s);

    Column::new()
        .push(text(title).size(FontSize::Title.px()))
        .push(name_input)
        .push(type_picker)
        .push(base_url_input)
        .push(api_key_input)
        .push(buttons)
        .spacing(Spacing::DEFAULT.s)
}

fn render_model_list(ai_config: &AiConfig) -> Column<'static, SettingsMessage, AppTheme> {
    let items: Vec<Element<'static, SettingsMessage, AppTheme>> = ai_config
        .models
        .iter()
        .map(|m| {
            let name = m.name.clone();
            let provider_name = ai_config
                .providers
                .iter()
                .find(|p| p.id == m.provider_id)
                .map(|p| p.name.clone())
                .unwrap_or("<unknown>".to_string());
            let model_id = m.model_id.clone();
            let edit_id = ModelId::from(m.id);
            let delete_id = ModelId::from(m.id);
            let select_id = ModelId::from(m.id);
            let is_selected = ai_config.selected_model_id == Some(m.id);

            let select_element: Element<'static, SettingsMessage, AppTheme> = if is_selected {
                text("Active").into()
            } else {
                Button::new(text("Select"))
                    .style(button::secondary)
                    .padding(ButtonSize::Small.to_iced_padding())
                    .on_press(SettingsMessage::Model(ModelMessage::Select(select_id)))
                    .into()
            };

            Row::new()
                .push(text(name).width(iced::Length::Fill))
                .push(text(provider_name))
                .push(text(model_id))
                .push(select_element)
                .push(
                    Button::new(text("Edit"))
                        .style(button::secondary)
                        .padding(ButtonSize::Small.to_iced_padding())
                        .on_press(SettingsMessage::Model(ModelMessage::Edit(edit_id))),
                )
                .push(
                    Button::new(text("Delete"))
                        .style(button::danger)
                        .padding(ButtonSize::Small.to_iced_padding())
                        .on_press(SettingsMessage::Model(ModelMessage::Delete(delete_id))),
                )
                .spacing(Spacing::DEFAULT.s)
                .align_y(iced::Alignment::Center)
                .into()
        })
        .collect();

    Column::new()
        .push(text("AI Models").size(FontSize::Title.px()))
        .push(Column::with_children(items).spacing(Spacing::DEFAULT.xs))
        .push(
            Button::new(text("Add Model"))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(SettingsMessage::Model(ModelMessage::Add)),
        )
        .spacing(Spacing::DEFAULT.s)
}

fn render_model_form(
    state: &SettingsState,
    ai_config: &AiConfig,
) -> Column<'static, SettingsMessage, AppTheme> {
    let edit = &state.model_edit;
    let title = if edit.is_new {
        "Add Model"
    } else {
        "Edit Model"
    };

    let model_name = edit.data.name.clone();
    let model_id_value = edit.data.model_id.clone();

    let name_input = TextInput::new("Model Name", &model_name)
        .on_input(|s| SettingsMessage::Model(ModelMessage::NameChanged(s)))
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    let model_id_input = TextInput::new("Model ID (e.g. gpt-4)", &model_id_value)
        .on_input(|s| SettingsMessage::Model(ModelMessage::ModelIdChanged(s)))
        .width(iced::Length::Fill)
        .padding(Spacing::DEFAULT.s);

    let provider_options: Vec<ProviderOption> = ai_config
        .providers
        .iter()
        .map(ProviderOption::from_config)
        .collect();

    let current_provider = if edit.data.provider_id == Uuid::nil() {
        provider_options.first().cloned()
    } else {
        provider_options
            .iter()
            .find(|po| po.id.0 == edit.data.provider_id)
            .cloned()
    };

    let provider_picker = Row::new()
        .push(text("Provider:"))
        .push(
            PickList::new(provider_options, current_provider, |po| {
                SettingsMessage::Model(ModelMessage::ProviderIdChanged(po.id))
            })
            .width(iced::Length::Fixed(160.0)),
        )
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    let buttons = Row::new()
        .push(
            Button::new(text("Save"))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(SettingsMessage::Model(ModelMessage::Save)),
        )
        .push(
            Button::new(text("Cancel"))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(SettingsMessage::Model(ModelMessage::Cancel)),
        )
        .spacing(Spacing::DEFAULT.s);

    Column::new()
        .push(text(title).size(FontSize::Title.px()))
        .push(name_input)
        .push(provider_picker)
        .push(model_id_input)
        .push(buttons)
        .spacing(Spacing::DEFAULT.s)
}
