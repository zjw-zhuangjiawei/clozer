//! Main application struct and entry point.
//!
//! Contains the App struct that coordinates state and UI rendering.

use iced::Element;

use crate::message::Message;
use crate::state::AppState;

/// Main application struct.
#[derive(Debug)]
pub struct App {
    state: AppState,
}

impl App {
    /// Creates a new App instance.
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    /// Creates a new App with sample data loaded.
    pub fn with_sample_data(mut self) -> Self {
        self.state = self.state.with_sample_data();
        self
    }

    /// Returns the application title.
    pub fn title(&self) -> String {
        String::from("Clozer")
    }

    /// Updates the application state with a message.
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        self.state.update(message)
    }

    /// Renders the application UI.
    pub fn view(&self) -> Element<'_, Message> {
        let left_panel = crate::ui::words_view(
            &self.state.data.word_registry,
            &self.state.data.meaning_registry,
            &self.state.data.cloze_registry,
            &self.state.data.tag_registry,
            &self.state.ui.words.word_input,
            &self.state.ui.words.tag_filter,
            &self.state.selection.selected_word_ids,
            &self.state.selection.selected_meaning_ids,
            &self.state.ui.words.expanded_word_ids,
            &self.state.ui.words.meaning_inputs,
            &self.state.ui.words.active_tag_dropdown,
            &self.state.ui.words.meanings_tag_dropdown_state,
            &self.state.ui.words.meanings_tag_search_input,
            &self.state.ui.words.meanings_tag_remove_search_input,
        );

        let right_panel = crate::ui::queue_view(
            &self.state.queue.queue_registry,
            &self.state.data.meaning_registry,
            &self.state.data.word_registry,
        );

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
}
