//! Application state management.
//!
//! Contains Model (data + business logic) and sub-modules.

pub mod generator;
pub mod model;
pub mod queue;

pub use self::generator::{Generator, GeneratorState};
pub use self::model::Model;
pub use self::queue::{QueueGenerationResult, process};
