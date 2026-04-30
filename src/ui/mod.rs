//! UI module with single-window sub-modules and shared components.

pub mod compositor;
pub mod notification;

pub mod design_tokens;
pub mod layout;
pub mod nav;
pub mod queue;
pub mod settings;
pub mod state;
pub mod tags;
pub mod theme;
pub mod widgets;
pub mod words;

pub use theme::{AppTheme, ThemeColors};
