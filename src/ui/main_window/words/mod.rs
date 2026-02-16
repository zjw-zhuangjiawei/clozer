//! Words panel module.

pub mod message;
pub mod state;
pub mod update;
pub mod view;

pub use self::message::WordsMessage;
pub use self::state::{MeaningInputState, TagDropdownState, TagsUiState, WordsUiState};
pub use self::update::update;
pub use self::view::view;
