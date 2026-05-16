//! Practice panel module for interactive cloze deletion exercises.

pub mod message;
pub mod state;
pub mod update;
pub mod view;

pub use self::message::PracticeMessage;
pub use self::state::PracticeState;
pub use self::update::update;
pub use self::view::view;
