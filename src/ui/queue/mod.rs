//! Queue panel module.

pub mod message;
pub mod update;
pub mod view;

pub use self::message::QueueMessage;
pub use self::update::update;
pub use self::view::view;
