//! Main window module.
//!
//! Contains the main window's view, update, state, and message types,
//! composed from words and queue sub-panels.

pub mod message;
pub mod queue;
pub mod state;
pub mod words;

pub use self::message::MainWindowMessage;
pub use self::state::MainWindowState;

use crate::message::Message;
use crate::state::Model;
use iced::{Element, Task};

/// Renders the main window by composing words and queue panels.
pub fn view<'a>(state: &'a MainWindowState, model: &'a Model) -> Element<'a, MainWindowMessage> {
    let left_panel = words::view(state, model).map(MainWindowMessage::Words);
    let right_panel = queue::view(model).map(MainWindowMessage::Queue);

    iced::widget::row![
        iced::widget::column![left_panel]
            .spacing(20)
            .padding(20)
            .width(iced::Length::FillPortion(2)),
        iced::widget::column![right_panel]
            .width(iced::Length::FillPortion(1))
            .padding(10),
    ]
    .into()
}

/// Dispatches main window messages to the appropriate sub-panel update handler.
///
/// Returns `Task<Message>` because queue processing produces global messages.
/// The caller (App::update) is responsible for wrapping words tasks with the window ID.
pub fn update(
    state: &mut MainWindowState,
    message: MainWindowMessage,
    model: &mut Model,
    window_id: iced::window::Id,
) -> Task<Message> {
    match message {
        MainWindowMessage::Words(msg) => words::update(state, msg, model)
            .map(move |m| Message::Main(window_id, MainWindowMessage::Words(m))),
        MainWindowMessage::Queue(msg) => {
            // Queue update returns Task<Message> directly (for QueueGenerationResult)
            queue::update(msg, model)
        }
    }
}
