//! Tags panel module for managing tags.

pub mod message;
pub mod state;
pub mod update;
pub mod view;

pub use self::message::TagsMessage;
pub use self::state::TagsState;
pub use self::update::update;
pub use self::view::view;
