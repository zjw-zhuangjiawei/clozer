//! Main application struct and entry point.
//!
//! Contains the App struct that coordinates state and UI rendering
//! with multi-window support.

use std::collections::BTreeMap;

use iced::{Element, Subscription, Task, Theme};

use crate::config::AppConfig;
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
    /// Creates a new App instance and opens the initial window.
    pub fn new(config: AppConfig) -> (Self, iced::Task<Message>) {
        // Initialize database
        let db_path = config.data_dir.join("data.redb");
        tracing::debug!("Initializing database at {:?}", db_path);
        let db = Db::new(&db_path).expect("Failed to create database");

        // Load AI config into generator
        let mut app_state = AppState::new(db);
        app_state.model.generator.load_from_config(&config.ai);

        // Load existing data from database
        tracing::debug!("Loading data from database");
        app_state.model.load_all();
        tracing::debug!(
            "Data loaded: {} words, {} meanings, {} tags, {} clozes",
            app_state.model.word_registry.count(),
            app_state.model.meaning_registry.count(),
            app_state.model.tag_registry.count(),
            app_state.model.cloze_registry.count(),
        );

        let app = Self {
            config,
            app_state,
            windows: BTreeMap::new(),
        };

        // Open initial main window
        let window_type = WindowType::Main;
        tracing::debug!("Opening main window");
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
        // Handle window management messages that need the window_id
        if let Message::WindowOpened(id, window_type) = message {
            let window = Window::new(window_type);
            self.windows.insert(id, window);
            return iced::Task::none();
        }

        if let Message::WindowCloseRequested(id) = message {
            return iced::window::close(id);
        }

        if let Message::WindowClosed(id) = message {
            let Some(window) = self.windows.remove(&id) else {
                unreachable!()
            };
            match window {
                Window::Main(_) => {
                    // Flush any unsaved data to database
                    tracing::debug!("Flushing dirty data on shutdown");
                    if let Err(e) = self.app_state.model.flush_all() {
                        tracing::error!("Failed to flush data on shutdown: {}", e);
                    }
                    self.config.save_to_file();
                    tracing::info!("Clozer shutting down");
                    return iced::exit();
                }
            }
        }

        // Get mutable reference to window state for UI operations
        // For simplicity, use the first window (single-window case)
        if let Some(Window::Main(window_state)) = self.windows.values_mut().next() {
            self.app_state.update(message, window_state)
        } else {
            // Fallback for no windows
            Task::none()
        }
    }

    /// Renders the application UI for the specified window.
    pub fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&id) {
            match window {
                Window::Main(window_state) => {
                    let left_panel = crate::ui::words_view(&self.app_state.model, window_state);

                    let right_panel = crate::ui::queue_view(&self.app_state.model);

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
        // Use close_requests to intercept close events before window closes
        // This allows custom handling (cleanup, confirmation dialogs, etc.)
        iced::event::listen_with(|event, _status, id| match event {
            iced::Event::Window(iced::window::Event::CloseRequested) => {
                Some(Message::WindowCloseRequested(id))
            }
            iced::Event::Window(iced::window::Event::Closed) => Some(Message::WindowClosed(id)),
            _ => None,
        })
    }
}
