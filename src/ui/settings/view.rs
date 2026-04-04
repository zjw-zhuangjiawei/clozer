//! Settings page view.

use super::message::{ModelMessage, ProviderMessage, SettingsMessage};
use crate::state::Model;
use crate::ui::theme::{AppTheme, ButtonSize, FontSize, Spacing};
use iced::Element;
use iced::widget::{Button, row, text};

use crate::ui::widgets::button;

/// Renders the settings page.
pub fn view<'a>(model: &'a Model) -> Element<'a, SettingsMessage, AppTheme> {
    let ai_config = &model.app_config.ai;

    // General Settings Section
    // let general_section = iced::widget::Column::new()
    //     .push(text("General").size(FontSize::Title.px()))
    //     .push(
    //         row![
    //             text("Theme:"),
    //             PickList::new(
    //                 vec![AppTheme::Light, AppTheme::Dark],
    //                 SettingsMessage::ThemeChanged,
    //             )
    //             .width(iced::Length::Fixed(120.0)),
    //         ]
    //         .spacing(Spacing::DEFAULT.s),
    //     )
    //     .push(
    //         row![
    //             text("Log Level:"),
    //             text(format!("{:?}", model.app_config.log_level)),
    //         ]
    //         .spacing(Spacing::DEFAULT.s),
    //     )
    //     .spacing(Spacing::DEFAULT.s);

    // AI Providers Section
    let providers_list: Vec<Element<'_, SettingsMessage, AppTheme>> = ai_config
        .providers
        .iter()
        .map(|p| {
            iced::widget::Column::new()
                .push(
                    row![
                        text(&p.name).width(iced::Length::Fill),
                        text(format!("{:?}", p.provider_type)),
                        Button::new(text("Edit"))
                            .style(button::secondary)
                            .padding(ButtonSize::Standard.to_iced_padding())
                            .on_press(SettingsMessage::Provider(ProviderMessage::Edit(p.id))),
                        Button::new(text("Delete"))
                            .style(button::danger)
                            .padding(ButtonSize::Standard.to_iced_padding())
                            .on_press(SettingsMessage::Provider(ProviderMessage::Delete(p.id))),
                    ]
                    .spacing(Spacing::DEFAULT.s),
                )
                .into()
        })
        .collect();

    let providers_section = iced::widget::Column::new()
        .push(text("AI Providers").size(FontSize::Title.px()))
        .push(iced::widget::Column::with_children(providers_list).spacing(Spacing::DEFAULT.xs))
        .push(
            Button::new(text("Add Provider"))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(SettingsMessage::Provider(ProviderMessage::Add)),
        )
        .spacing(Spacing::DEFAULT.s);

    // AI Models Section
    let models_list: Vec<Element<'_, SettingsMessage, AppTheme>> = ai_config
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
                        Button::new(text("Edit"))
                            .style(button::secondary)
                            .padding(ButtonSize::Standard.to_iced_padding())
                            .on_press(SettingsMessage::Model(ModelMessage::Edit(m.id))),
                        Button::new(text("Delete"))
                            .style(button::danger)
                            .padding(ButtonSize::Standard.to_iced_padding())
                            .on_press(SettingsMessage::Model(ModelMessage::Delete(m.id))),
                    ]
                    .spacing(Spacing::DEFAULT.s),
                )
                .into()
        })
        .collect();

    let models_section = iced::widget::Column::new()
        .push(text("AI Models").size(FontSize::Title.px()))
        .push(iced::widget::Column::with_children(models_list).spacing(Spacing::DEFAULT.xs))
        .push(
            Button::new(text("Add Model"))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(SettingsMessage::Model(ModelMessage::Add)),
        )
        .spacing(Spacing::DEFAULT.s);

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
        .push(text("Active Model").size(FontSize::Title.px()))
        .push(row![text("Selected:"), selected_model_text].spacing(Spacing::DEFAULT.s))
        .spacing(Spacing::DEFAULT.s);

    // Data Directory Section
    let data_dir_section = iced::widget::Column::new()
        .push(text("Data").size(FontSize::Title.px()))
        .push(text(format!(
            "Data Directory: {:?}",
            model.app_config.data_dir
        )))
        .spacing(Spacing::DEFAULT.s);

    // Main settings column
    iced::widget::Column::new()
        .push(text("Settings").size(FontSize::Display.px()))
        // .push(general_section)
        .push(providers_section)
        .push(models_section)
        .push(selected_model_section)
        .push(data_dir_section)
        .spacing(Spacing::DEFAULT.l)
        .padding(Spacing::DEFAULT.l)
        .into()
}
