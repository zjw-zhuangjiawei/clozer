//! Main application struct and entry point.
//!
//! Contains the App struct that coordinates state and UI rendering
//! with multi-window support and hierarchical message routing.

use std::collections::BTreeMap;

use iced::{Element, Subscription, Task, Theme};

use crate::config::AppConfig;
use crate::message::Message;
use crate::persistence::Db;
use crate::state::AppState;
use crate::ui::main_window;
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
    ///
    /// Routes messages by type:
    /// - Window lifecycle messages are handled directly
    /// - `Message::Main(id, msg)` is routed to the correct window's update handler
    /// - Global messages (like `QueueGenerationResult`) are handled at the app level
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Window lifecycle
            Message::WindowOpened(id, window_type) => {
                let window = Window::new(window_type);
                self.windows.insert(id, window);
                iced::Task::none()
            }

            Message::WindowCloseRequested(id) => iced::window::close(id),

            Message::WindowClosed(id) => {
                let Some(window) = self.windows.remove(&id) else {
                    unreachable!()
                };
                match window {
                    Window::Main(_) => {
                        tracing::debug!("Flushing dirty data on shutdown");
                        if let Err(e) = self.app_state.model.flush_all() {
                            tracing::error!("Failed to flush data on shutdown: {}", e);
                        }
                        self.config.save_to_file();
                        tracing::info!("Clozer shutting down");
                        iced::exit()
                    }
                }
            }

            // Route to specific main window by ID
            Message::Main(window_id, msg) => {
                if let Some(Window::Main(window_state)) = self.windows.get_mut(&window_id) {
                    main_window::update(window_state, msg, &mut self.app_state.model, window_id)
                } else {
                    Task::none()
                }
            }

            // Global messages
            Message::QueueGenerationResult(result) => {
                self.app_state
                    .model
                    .queue_registry
                    .set_completed(result.item_id);
                self.app_state.model.cloze_registry.add(result.cloze);
                Task::none()
            }
        }
    }

    /// Renders the application UI for the specified window.
    pub fn view(&self, id: iced::window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&id) {
            match window {
                Window::Main(window_state) => {
                    main_window::view(window_state, &self.app_state.model)
                        .map(move |msg| Message::Main(id, msg))
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
        iced::event::listen_with(|event, _status, id| match event {
            iced::Event::Window(iced::window::Event::CloseRequested) => {
                Some(Message::WindowCloseRequested(id))
            }
            iced::Event::Window(iced::window::Event::Closed) => Some(Message::WindowClosed(id)),
            _ => None,
        })
    }
}
