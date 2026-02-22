//! Words panel module.

pub mod detail_view;
pub mod message;
pub mod state;
pub mod update;
pub mod view;

pub use self::detail_view::view as detail_view;
pub use self::message::WordsMessage;
pub use self::state::{
    ClozeFilter, DetailSelection, FilterState, MeaningInputState, TagDropdownState,
    TagDropdownTarget, TagsUiState, WordsUiState,
};
pub use self::update::update;
pub use self::view::view;
