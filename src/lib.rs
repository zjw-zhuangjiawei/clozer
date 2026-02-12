//! Clozer - A desktop application for creating cloze deletion cards.

pub mod app;
pub mod message;

pub use self::app::App;
pub use self::message::Message;

pub mod config;
pub mod models;
pub mod persistence;
pub mod registry;
pub mod state;
pub mod ui;
