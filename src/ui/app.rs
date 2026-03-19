//! Application UI view and update functions.
//!
//! Contains the view() and update functions that compose words, queue,
//! and settings panels into the single main window.

pub use crate::ui::nav::NavItem;
pub use crate::ui::state::MainWindowState;

use crate::message::Message;
use crate::state::Model;
use crate::ui::words::message::WordsMessage;
use iced::{Element, Task};

/// Renders the main window by composing words, queue, and settings panels.
pub fn view<'a>(state: &'a MainWindowState, model: &'a Model) -> Element<'a, Message> {
    // Navigation bar
    let nav_buttons: Vec<Element<'a, Message>> =
        [NavItem::Words, NavItem::Queue, NavItem::Settings]
            .iter()
            .map(|item| {
                let is_active = state.current_view == *item;
                let label = item.label();
                let button = iced::widget::button(iced::widget::text(label))
                    .style(if is_active {
                        iced::widget::button::primary
                    } else {
                        iced::widget::button::secondary
                    })
                    .on_press(Message::Navigate(*item));
                button.into()
            })
            .collect();

    let nav_bar = iced::widget::Row::with_children(nav_buttons).spacing(10);

    // Content based on current navigation view
    let content: Element<'a, Message> = match state.current_view {
        NavItem::Words => {
            let left_panel = crate::ui::words::view(state, model).map(Message::Words);
            // Hide queue panel when in Words view, show full width
            iced::widget::column![left_panel]
                .spacing(20)
                .padding(20)
                .into()
        }
        NavItem::Queue => {
            let queue_panel = crate::ui::queue::view(model).map(Message::Queue);
            iced::widget::column![queue_panel]
                .spacing(20)
                .padding(20)
                .into()
        }
        NavItem::Settings => {
            let settings_panel = crate::ui::settings::view::view(model).map(Message::Settings);
            iced::widget::column![settings_panel]
                .spacing(20)
                .padding(20)
                .into()
        }
    };

    iced::widget::column![nav_bar, content].into()
}

/// Handles words panel update - returns Task<Message> for async operations.
pub fn update_words(
    state: &mut MainWindowState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<Message> {
    crate::ui::words::update(state, message, model).map(Message::Words)
}

/// Handles settings panel update - returns Task<Message> for async operations.
pub fn update_settings(
    _state: &mut MainWindowState,
    message: crate::ui::settings::SettingsMessage,
    model: &mut Model,
) -> Task<Message> {
    crate::ui::settings::update::update(message, model).map(Message::Settings)
}
