//! Settings page view.

use super::message::SettingsMessage;
use crate::state::Model;
use iced::Element;
use iced::widget::{button, row, text};

/// Renders the settings page.
pub fn view<'a>(model: &'a Model) -> Element<'a, SettingsMessage> {
    let ai_config = &model.app_config.ai;

    // General Settings Section
    let general_section = iced::widget::Column::new()
        .push(text("General").size(18))
        .push(
            row![
                text("Log Level:"),
                text(format!("{:?}", model.app_config.log_level)),
            ]
            .spacing(10),
        )
        .spacing(10);

    // AI Providers Section
    let providers_list: Vec<Element<'_, SettingsMessage>> = ai_config
        .providers
        .iter()
        .map(|p| {
            iced::widget::Column::new()
                .push(
                    row![
                        text(&p.name).width(iced::Length::Fill),
                        text(format!("{:?}", p.provider_type)),
                        button("Edit").on_press(SettingsMessage::EditProvider(p.id)),
                        button("Delete").on_press(SettingsMessage::DeleteProvider(p.id)),
                    ]
                    .spacing(10),
                )
                .into()
        })
        .collect();

    let providers_section = iced::widget::Column::new()
        .push(text("AI Providers").size(18))
        .push(iced::widget::Column::with_children(providers_list).spacing(5))
        .push(button("Add Provider").on_press(SettingsMessage::AddProvider))
        .spacing(10);

    // AI Models Section
    let models_list: Vec<Element<'_, SettingsMessage>> = ai_config
        .models
        .iter()
        .map(|m| {
            let provider_name = ai_config
                .providers
                .iter()
                .find(|p| p.id == m.provider_id)
                .map(|p| p.name.as_str())
                .unwrap_or("<unknown>");
            iced::widget::Column::new()
                .push(
                    row![
                        text(&m.name).width(iced::Length::Fill),
                        text(provider_name),
                        text(&m.model_id),
                        button("Edit").on_press(SettingsMessage::EditModel(m.id)),
                        button("Delete").on_press(SettingsMessage::DeleteModel(m.id)),
                    ]
                    .spacing(10),
                )
                .into()
        })
        .collect();

    let models_section = iced::widget::Column::new()
        .push(text("AI Models").size(18))
        .push(iced::widget::Column::with_children(models_list).spacing(5))
        .push(button("Add Model").on_press(SettingsMessage::AddModel))
        .spacing(10);

    // Selected Model Section
    let selected_model_text = if let Some(selected_id) = ai_config.selected_model_id {
        let selected_name = ai_config
            .models
            .iter()
            .find(|m| m.id == selected_id)
            .map(|m| m.name.as_str())
            .unwrap_or("<unknown>");
        text(selected_name)
    } else {
        text("<none>")
    };

    let selected_model_section = iced::widget::Column::new()
        .push(text("Active Model").size(18))
        .push(row![text("Selected:"), selected_model_text].spacing(10))
        .spacing(10);

    // Data Directory Section
    let data_dir_section = iced::widget::Column::new()
        .push(text("Data").size(18))
        .push(text(format!(
            "Data Directory: {:?}",
            model.app_config.data_dir
        )))
        .spacing(10);

    // Main settings column
    iced::widget::Column::new()
        .push(text("Settings").size(24))
        .push(general_section)
        .push(providers_section)
        .push(models_section)
        .push(selected_model_section)
        .push(data_dir_section)
        .spacing(20)
        .padding(20)
        .into()
}
