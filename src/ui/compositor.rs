use crate::message::Message;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::layout::breakpoint::Breakpoint;
use crate::ui::nav::NavItem;
use crate::ui::notification::Notification as NotificationData;
use crate::ui::practice::{
    PracticeMessage, PracticeState, update as practice_update, view as practice_view,
};
use crate::ui::settings::state::SettingsState;
use crate::ui::sidebar;
use crate::ui::state::UiState;
use crate::ui::status_bar::status_bar;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::widget::{Column, Container, Row};
use iced::{Element, Length, Task};

pub fn view<'a>(state: &'a UiState, model: &'a Model) -> Element<'a, Message, AppTheme> {
    let breakpoint = Breakpoint::from_width(state.window_width as f32);

    let sidebar_element = sidebar::sidebar(state, breakpoint);
    let sidebar_panel = Container::new(sidebar_element)
        .style(|theme: &AppTheme| iced::widget::container::Style {
            background: Some(theme.colors().semantic.surface.raised.into()),
            ..Default::default()
        })
        .width(Length::Fixed(breakpoint.sidebar_panel_width()))
        .height(Length::Fill);

    let content: Element<'a, Message, AppTheme> = match state.current_view {
        NavItem::Words => {
            crate::ui::words::explorer::view(&state.words, model, breakpoint, &state.i18n)
                .map(Message::Words)
        }
        NavItem::Queue => crate::ui::queue::view(model, &state.i18n).map(Message::Queue),
        NavItem::Tags => crate::ui::tags::view(&state.tags, model, &state.i18n).map(Message::Tags),
        NavItem::Practice => {
            practice_view(&state.practice, model, &state.i18n).map(Message::Practice)
        }
        NavItem::Settings => crate::ui::settings::view::view(&state.settings, model, &state.i18n)
            .map(Message::Settings),
    };

    if breakpoint.use_bottom_bar() {
        let bottom = sidebar::bottom_tab_bar(state);
        Column::new()
            .push(content)
            .push(bottom)
            .push(status_bar(&state.notifications))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    } else {
        Column::new()
            .push(
                Row::new()
                    .push(sidebar_panel)
                    .push(content)
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .push(status_bar(&state.notifications))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn update_words(
    state: &mut WordsState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<Message> {
    crate::ui::words::update(state, message, model).map(|msg| match msg {
        WordsMessage::Notify { level, message } => {
            let notification = match level {
                crate::ui::words::message::NotificationLevel::Error => {
                    NotificationData::error(0, message)
                }
                crate::ui::words::message::NotificationLevel::Warning => {
                    NotificationData::warning(0, message)
                }
                crate::ui::words::message::NotificationLevel::Info => {
                    NotificationData::info(0, message)
                }
            };
            Message::PushNotification(notification)
        }
        WordsMessage::ExportFailed(err) => Message::PushNotification(NotificationData::error(
            0,
            format!("Export failed: {}", err),
        )),
        other => Message::Words(other),
    })
}

pub fn update_settings(
    state: &mut SettingsState,
    message: crate::ui::settings::SettingsMessage,
    _model: &mut Model,
) -> Task<Message> {
    use crate::ui::settings::SettingsMessage;
    match message {
        SettingsMessage::ThemeChanged(theme) => {
            Task::done(crate::message::Message::ThemeChanged(theme))
        }
        SettingsMessage::General(ref msg) => {
            if let crate::ui::settings::message::GeneralSettingsMessage::LocaleChanged(locale) = msg
            {
                return Task::done(crate::message::Message::LocaleChanged(*locale));
            }
            crate::ui::settings::handlers::update(state, message, _model).map(Message::Settings)
        }
        _ => crate::ui::settings::handlers::update(state, message, _model).map(Message::Settings),
    }
}

pub fn update_practice(
    state: &mut PracticeState,
    message: PracticeMessage,
    model: &mut Model,
) -> Task<Message> {
    practice_update(state, message, model).map(|msg| match msg {
        PracticeMessage::Notify { level, message } => {
            let notification = match level {
                crate::ui::practice::message::NotificationLevel::Error => {
                    NotificationData::error(0, message)
                }
                crate::ui::practice::message::NotificationLevel::Warning => {
                    NotificationData::warning(0, message)
                }
                crate::ui::practice::message::NotificationLevel::Info => {
                    NotificationData::info(0, message)
                }
            };
            Message::PushNotification(notification)
        }
        other => Message::Practice(other),
    })
}
