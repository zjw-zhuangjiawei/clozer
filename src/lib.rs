//! Clozer - A desktop application for creating cloze deletion cards.

pub mod app;
pub mod message;
pub mod window;

pub use self::app::App;
pub use self::message::Message;
pub use self::window::{Window, WindowType};

pub mod config;
pub mod models;
pub mod persistence;
pub mod registry;
pub mod state;
pub mod ui;

pub use self::config::{AppConfig, CliConfig, EnvConfig};
