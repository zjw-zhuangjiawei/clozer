//! Main application struct and entry point.
//!
//! Contains the App struct that coordinates state and UI rendering
//! with multi-window support.

use std::collections::BTreeMap;

use iced::{Element, Subscription, Theme};

use crate::config::{AppConfig, CliConfig, EnvConfig};
use crate::message::Message;
use crate::persistence::Db;
use crate::state::AppState;
use crate::window::{Window, WindowType};

/// Main application struct with multi-window support.
#[derive(Debug)]
pub struct App {
    pub config: AppConfig,
    pub app_state: AppState,
    windows: BTreeMap<iced::window::Id, Window>,
}

impl App {
    /// TODO: Remove in production - for development only
    #[allow(dead_code)]
    pub fn with_sample_data(mut self) -> Self {
        self.app_state = self.app_state.with_sample_data();
        self
    }

    /// Creates a new App instance and opens the initial window.
    pub fn new(
        cli_args: impl IntoIterator<Item = std::ffi::OsString>,
        env_vars: impl IntoIterator<Item = (String, String)>,
    ) -> (Self, iced::Task<Message>) {
        // Load configuration
        let cli = CliConfig::load(cli_args);
        let env = EnvConfig::load(env_vars).expect("Failed to load env config");
        let config = AppConfig::load(cli, env).expect("Failed to load app config");

        // Initialize database
        let db_path = config.data_dir.join("data.redb");
        let db = Db::new(&db_path).expect("Failed to create database");

        // Create app state with database (takes ownership of db)
        let app_state = AppState::builder().db(db).build();

        let app = Self {
            config,
            app_state,
            windows: BTreeMap::new(),
        };

        // Open initial main window
        let window_type = WindowType::Main;
        let (_, open_task) = iced::window::open(window_type.window_settings());

        let task = open_task.map(move |id| Message::WindowOpened(id, window_type));

        (app, task)
    }

    /// Returns the window title.
    pub fn title(&self, id: iced::window::Id) -> String {
        self.windows
            .get(&id)
            .map(|window| match window {
                Window::Main(_) => "Clozer".to_string(),
            })
            .unwrap_or_else(|| "Clozer".to_string())
    }

    /// Updates the application state with a message.
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::WindowOpened(id, window_type) => {
                let window = Window::new(window_type);
                self.windows.insert(id, window);
                iced::Task::none()
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);
                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    iced::Task::none()
                }
            }
            // All other messages go directly to app_state
            _ => self.app_state.update(message),
        }
    }

    /// Renders the application UI for the specified window.
    pub fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&id) {
            match window {
                Window::Main(_) => {
                    let left_panel = crate::ui::words_view(
                        &self.app_state.data.word_registry,
                        &self.app_state.data.meaning_registry,
                        &self.app_state.data.cloze_registry,
                        &self.app_state.data.tag_registry,
                        &self.app_state.ui.words.word_input,
                        &self.app_state.ui.words.tag_filter,
                        &self.app_state.selection.selected_word_ids,
                        &self.app_state.selection.selected_meaning_ids,
                        &self.app_state.ui.words.expanded_word_ids,
                        &self.app_state.ui.words.meaning_inputs,
                        &self.app_state.ui.words.active_tag_dropdown,
                        &self.app_state.ui.words.meanings_tag_dropdown_state,
                        &self.app_state.ui.words.meanings_tag_search_input,
                        &self.app_state.ui.words.meanings_tag_remove_search_input,
                    );

                    let right_panel = crate::ui::queue_view(
                        &self.app_state.queue.queue_registry,
                        &self.app_state.data.meaning_registry,
                        &self.app_state.data.word_registry,
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
        } else {
            iced::widget::space().into()
        }
    }

    /// Returns the theme for the specified window.
    pub fn theme(&self, _id: iced::window::Id) -> Theme {
        Theme::Dark
    }

    /// Returns the application subscription.
    pub fn subscription(&self) -> Subscription<Message> {
        iced::window::close_events().map(Message::WindowClosed)
    }
}
