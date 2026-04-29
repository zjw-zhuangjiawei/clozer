//! Main application struct and entry point.
//!
//! Contains the App struct that coordinates state and UI rendering
//! with single-window support and flat message routing.

use iced::{Element, Subscription, Task};

use crate::config::AppConfig;
use crate::message::Message;
use crate::persistence::Db;
use crate::state::AppState;
use crate::ui::AppTheme;
use crate::ui::words::WordsMessage;
use crate::ui::{self, state::MainWindowState};

/// Main application struct with single-window support.
#[derive(Debug)]
pub struct App {
    pub config: AppConfig,
    pub app_state: AppState,
    pub window_state: MainWindowState,
}

impl App {
    /// Creates a new App instance.
    pub fn new(config: AppConfig) -> (Self, iced::Task<Message>) {
        // Initialize database
        let db_path = config.data_dir.join("data.redb");
        tracing::debug!("Initializing database at {:?}", db_path);
        let db = Db::new(&db_path).expect("Failed to create database");

        // Create app state with config
        let mut app_state = AppState::new(db, config.clone());
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

        // Initialize window state with theme from config
        let window_state = MainWindowState {
            theme: config.theme,
            ..MainWindowState::new()
        };

        let app = Self {
            config,
            app_state,
            window_state,
        };

        (app, Task::none())
    }

    /// Returns the window title.
    pub fn title(&self) -> String {
        "Clozer".to_string()
    }

    /// Updates the application state with a message.
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Words panel - already wrapped in ui::app::update_words
            Message::Words(msg) => {
                ui::app::update_words(&mut self.window_state, msg, &mut self.app_state.model)
            }

            // Queue panel - already returns Task<Message>
            Message::Queue(msg) => ui::queue::update(msg, &mut self.app_state.model),

            // Settings panel - wrapped in ui::app::update_settings
            Message::Settings(msg) => {
                ui::app::update_settings(&mut self.window_state, msg, &mut self.app_state.model)
            }

            // Navigation
            Message::Navigate(nav_item) => {
                self.window_state.current_view = nav_item;
                Task::none()
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

            // Close requested - exit the application
            Message::CloseRequested => {
                self.on_exit();
                iced::exit()
            }

            // Window resize for responsive layout
            Message::WindowResized(width) => {
                self.window_state.window_width = width;
                Task::none()
            }

            // Theme change
            Message::ThemeChanged(theme) => {
                self.window_state.theme = theme;
                self.config.theme = theme;

                self.config.save_to_file();
                tracing::info!("Theme changed to: {:?}", theme);
                Task::none()
            }

            // Tab pressed - forward to words panel for suggestion acceptance
            Message::TabPressed => {
                use crate::ui::nav::NavItem;
                if self.window_state.current_view == NavItem::Words {
                    ui::app::update_words(
                        &mut self.window_state,
                        WordsMessage::SuggestionAccepted,
                        &mut self.app_state.model,
                    )
                } else {
                    Task::none()
                }
            }
        }
    }

    /// Renders the application UI.
    pub fn view(&self) -> Element<'_, Message, AppTheme> {
        ui::app::view(&self.window_state, &self.app_state.model)
    }

    /// Returns the theme.
    pub fn theme(&self) -> AppTheme {
        self.window_state.theme
    }

    /// Returns the application subscription.
    pub fn subscription(&self) -> Subscription<Message> {
        // Listen for window close request and resize events
        // Also listen for Tab key to handle search suggestion acceptance
        iced::event::listen_with(|event, _status, _id| match event {
            iced::Event::Window(iced::window::Event::CloseRequested) => {
                Some(Message::CloseRequested)
            }
            iced::Event::Window(iced::window::Event::Resized(size)) => {
                Some(Message::WindowResized(size.width as u16))
            }
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed {
                key: iced::keyboard::Key::Named(iced::keyboard::key::Named::Tab),
                ..
            }) => Some(Message::TabPressed),
            _ => None,
        })
    }

    /// Called when the application is closing.
    pub fn on_exit(&mut self) {
        tracing::debug!("Flushing dirty data on shutdown");
        if let Err(e) = self.app_state.model.flush_all() {
            tracing::error!("Failed to flush data on shutdown: {}", e);
        }
        self.config.save_to_file();
        tracing::info!("Clozer shutting down");
    }

    /// Runs the application with the given configuration.
    pub fn run(config: AppConfig) {
        let _ = iced::application(move || App::new(config.clone()), App::update, App::view)
            .window(iced::window::Settings {
                exit_on_close_request: false,
                ..Default::default()
            })
            .title(App::title)
            .subscription(App::subscription)
            .theme(App::theme)
            .run();
    }
}
