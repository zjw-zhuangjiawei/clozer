use crate::message::Message;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::layout::breakpoint::Breakpoint;
use crate::ui::nav::NavItem;
use crate::ui::notification::Notification as NotificationData;
use crate::ui::settings::state::SettingsState;
use crate::ui::sidebar;
use crate::ui::state::UiState;
use crate::ui::status_bar::status_bar;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::widget::{Column, Row};
use iced::{Element, Task};

pub fn view<'a>(state: &'a UiState, model: &'a Model) -> Element<'a, Message, AppTheme> {
    let breakpoint = Breakpoint::from_width(state.window_width as f32);

    let sidebar_element = sidebar::sidebar(state, breakpoint);

    let content: Element<'a, Message, AppTheme> = match state.current_view {
        NavItem::Words => {
            crate::ui::words::explorer::view(&state.words, model, breakpoint).map(Message::Words)
        }
        NavItem::Queue => crate::ui::queue::view(model).map(Message::Queue),
        NavItem::Tags => crate::ui::tags::view(&state.tags, model).map(Message::Tags),
        NavItem::Settings => {
            crate::ui::settings::view::view(&state.settings, model).map(Message::Settings)
        }
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
                    .push(sidebar_element)
                    .push(content)
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill),
            )
            .push(status_bar(&state.notifications))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

pub fn update_words(
    state: &mut WordsState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<Message> {
    crate::ui::words::update(state, message, model).map(|msg| match msg {
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
        _ => crate::ui::settings::handlers::update(state, message, _model).map(Message::Settings),
    }
}
