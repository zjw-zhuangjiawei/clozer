//! Application state management.
//!
//! Contains Model (data + business logic) and AppState (orchestrator).

pub mod generator;
pub mod model;
pub mod queue;

pub use self::generator::{Generator, GeneratorState};
pub use self::model::Model;
pub use self::queue::{QueueGenerationResult, process};

use crate::config::AppConfig;

/// AppState holding Model (data + business logic only).
///
/// Update logic has been moved to per-window update modules in `ui/`.
#[derive(Debug)]
pub struct AppState {
    pub model: Model,
}

impl AppState {
    /// Creates a new AppState with the given database and config.
    pub fn new(db: crate::persistence::Db, config: AppConfig) -> Self {
        Self {
            model: Model::new(db, config),
        }
    }
}
