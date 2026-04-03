//! Settings module for the embedded settings page.

pub mod handlers;
pub mod message;
pub mod state;
pub mod update;
pub mod view;

pub use self::message::SettingsMessage;
pub use self::state::SettingsState;
