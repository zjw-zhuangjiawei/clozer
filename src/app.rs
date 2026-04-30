//! Main application struct and entry point.
//!
//! Contains the App struct that coordinates model (data/business) and
//! ui (presentation) layers with flat message routing.

use std::sync::Arc;

use iced::{Element, Subscription, Task};

use crate::config::AppConfig;
use crate::message::Message;
use crate::persistence::Db;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::state::UiState;
use crate::ui::words::WordsMessage;
use crate::ui::{self, compositor};

/// Main application struct with single-window support.
#[derive(Debug)]
pub struct App {
    pub config: AppConfig,
    pub model: Model,
    pub ui: UiState,
}

impl App {
    /// Creates a new App instance.
    pub fn new(config: AppConfig) -> (Self, iced::Task<Message>) {
        // Initialize database
        let db_path = config.data_dir.join("data.redb");
        tracing::debug!("Initializing database at {:?}", db_path);
        let db = Db::new(&db_path).expect("Failed to create database");

        // Create model with config
        let mut model = Model::new(db, config.clone());
        model.generator.load_from_config(&config.ai);

        // Load existing data from database
        tracing::debug!("Loading data from database");
        model.load_all();
        tracing::debug!(
            "Data loaded: {} words, {} meanings, {} tags, {} clozes",
            model.word_registry.count(),
            model.meaning_registry.count(),
            model.tag_registry.count(),
            model.cloze_registry.count(),
        );

        // Initialize UI state with theme from config
        let ui = UiState {
            theme: config.theme,
            ..UiState::new()
        };

        let app = Self { config, model, ui };

        (app, Task::none())
    }

    /// Returns the window title.
    pub fn title(&self) -> String {
        "Clozer".to_string()
    }

    /// Updates the application state with a message.
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            // Words panel
            Message::Words(msg) => {
                compositor::update_words(&mut self.ui.words, msg, &mut self.model)
            }

            // Queue panel
            Message::Queue(msg) => ui::queue::update(msg, &mut self.model),

            // Tags panel
            Message::Tags(msg) => match msg {
                crate::ui::tags::TagsMessage::NavigateToMeanings(tag_id) => {
                    if let Some(tag) = self.model.tag_registry.get(tag_id) {
                        self.ui.current_view = crate::ui::nav::NavItem::Words;
                        let tag_name = tag.name.clone();
                        self.ui.words.search.set_query(format!("#{}", tag_name));
                        self.ui.words.search.execute(
                            &self.model.word_registry,
                            &self.model.meaning_registry,
                            &self.model.cloze_registry,
                            &self.model.queue_registry,
                            &self.model.tag_registry,
                        );
                    }
                    Task::none()
                }
                other => crate::ui::tags::update(&mut self.ui.tags, other, &mut self.model)
                    .map(Message::Tags),
            },

            // Settings panel
            Message::Settings(msg) => {
                compositor::update_settings(&mut self.ui.settings, msg, &mut self.model)
            }

            // Navigation
            Message::Navigate(nav_item) => {
                self.ui.current_view = nav_item;
                Task::none()
            }

            // Global messages
            Message::QueueGenerationResult(result) => match result {
                crate::state::QueueGenerationResult::Success { item_id, cloze } => {
                    self.model.queue_registry.set_completed(item_id);
                    self.model.cloze_registry.add(cloze);
                    self.ui.push_notification(
                        crate::ui::notification::NotificationLevel::Info,
                        "Cloze generated successfully",
                    );
                    Task::none()
                }
                crate::state::QueueGenerationResult::Failed { item_id, error } => {
                    self.model.queue_registry.set_failed(item_id, error.clone());
                    self.ui.push_notification(
                        crate::ui::notification::NotificationLevel::Error,
                        format!("Generation failed: {}", error),
                    );
                    Task::none()
                }
            },

            // Notification management
            Message::PushNotification(notification) => {
                self.ui
                    .push_notification(notification.level, notification.message);
                Task::none()
            }
            Message::DismissNotification(id) => {
                self.ui.dismiss_notification(id);
                Task::none()
            }
            Message::NotificationTick => {
                self.ui.clean_expired();
                Task::none()
            }

            // Close requested - exit the application
            Message::CloseRequested => {
                self.on_exit();
                iced::exit()
            }

            // Window resize for responsive layout
            Message::WindowResized(width) => {
                self.ui.window_width = width;
                Task::none()
            }

            // Theme change
            Message::ThemeChanged(theme) => {
                self.ui.theme = theme;
                if let Some(c) = Arc::get_mut(&mut self.model.app_config) {
                    c.theme = theme;
                    c.save_to_file();
                }
                tracing::info!("Theme changed to: {:?}", theme);
                Task::none()
            }

            // Tab pressed - forward to words panel for suggestion acceptance
            Message::TabPressed => {
                use crate::ui::nav::NavItem;
                if self.ui.current_view == NavItem::Words {
                    compositor::update_words(
                        &mut self.ui.words,
                        WordsMessage::SuggestionAccepted,
                        &mut self.model,
                    )
                } else {
                    Task::none()
                }
            }
        }
    }

    /// Renders the application UI.
    pub fn view(&self) -> Element<'_, Message, AppTheme> {
        compositor::view(&self.ui, &self.model)
    }

    /// Returns the theme.
    pub fn theme(&self) -> AppTheme {
        self.ui.theme
    }

    /// Returns the application subscription.
    pub fn subscription(&self) -> Subscription<Message> {
        let event_sub = iced::event::listen_with(|event, _status, _id| match event {
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
        });

        let tick =
            iced::time::every(std::time::Duration::from_secs(2)).map(|_| Message::NotificationTick);

        Subscription::batch(vec![event_sub, tick])
    }

    /// Called when the application is closing.
    pub fn on_exit(&mut self) {
        tracing::debug!("Flushing dirty data on shutdown");
        if let Err(e) = self.model.flush_all() {
            tracing::error!("Failed to flush data on shutdown: {}", e);
        }
        if let Some(c) = Arc::get_mut(&mut self.model.app_config) {
            c.save_to_file();
        }
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
