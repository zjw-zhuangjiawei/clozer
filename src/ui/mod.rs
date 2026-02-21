//! UI module with single-window sub-modules and shared components.

pub mod app;
pub mod components;
pub mod message;
pub mod nav;
pub mod queue;
pub mod settings;
pub mod state;
pub mod theme;
pub mod words;

pub use theme::{AppTheme, ThemeColors};
