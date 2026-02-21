//! Application UI view and update functions.
//!
//! Contains the view() and update() functions that compose words, queue,
//! and settings panels into the single main window.

pub use crate::ui::message::MainWindowMessage;
pub use crate::ui::nav::NavItem;
pub use crate::ui::state::MainWindowState;

use crate::message::Message;
use crate::state::Model;
use iced::{Element, Task};

/// Renders the main window by composing words, queue, and settings panels.
pub fn view<'a>(state: &'a MainWindowState, model: &'a Model) -> Element<'a, MainWindowMessage> {
    // Navigation bar
    let nav_buttons: Vec<Element<'a, MainWindowMessage>> =
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
                    .on_press(MainWindowMessage::Navigate(*item));
                button.into()
            })
            .collect();

    let nav_bar = iced::widget::Row::with_children(nav_buttons).spacing(10);

    // Content based on current navigation view
    let content: Element<'a, MainWindowMessage> = match state.current_view {
        NavItem::Words => {
            let left_panel = crate::ui::words::view(state, model).map(MainWindowMessage::Words);
            // Hide queue panel when in Words view, show full width
            iced::widget::column![left_panel]
                .spacing(20)
                .padding(20)
                .into()
        }
        NavItem::Queue => {
            let queue_panel = crate::ui::queue::view(model).map(MainWindowMessage::Queue);
            iced::widget::column![queue_panel]
                .spacing(20)
                .padding(20)
                .into()
        }
        NavItem::Settings => {
            let settings_panel = crate::ui::settings::view::view(model).map(MainWindowMessage::Settings);
            iced::widget::column![settings_panel]
                .spacing(20)
                .padding(20)
                .into()
        }
    };

    iced::widget::column![nav_bar, content].into()
}

/// Dispatches main window messages to the appropriate sub-panel update handler.
///
/// Returns `Task<Message>` because queue processing produces global messages.
/// The caller (App::update) is responsible for wrapping words tasks with the window ID.
pub fn update(
    state: &mut MainWindowState,
    message: MainWindowMessage,
    model: &mut Model,
    _window_id: iced::window::Id,
) -> Task<Message> {
    match message {
        MainWindowMessage::Words(msg) => crate::ui::words::update(state, msg, model)
            .map(move |m| Message::Main(MainWindowMessage::Words(m))),
        MainWindowMessage::Queue(msg) => {
            // Queue update returns Task<Message> directly (for QueueGenerationResult)
            crate::ui::queue::update(msg, model)
        }
        MainWindowMessage::Settings(msg) => crate::ui::settings::update::update(msg, model)
            .map(move |m| Message::Main(MainWindowMessage::Settings(m))),
        MainWindowMessage::Navigate(nav_item) => {
            state.current_view = nav_item;
            Task::none()
        }
    }
}
