//! Role layer - Color roles grouped by purpose
//!
//! Role layer provides stateless functions for selecting colors
//! from scales based on color mode. It sits between the Scale layer
//! and the Semantic layer.

pub mod background;
pub mod border;
pub mod foreground;
pub mod interactive;

pub use background::BackgroundRole;
pub use border::BorderRole;
pub use foreground::ForegroundRole;
pub use interactive::InteractiveRole;
