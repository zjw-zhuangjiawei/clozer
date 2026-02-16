//! Window types and state management.
//!
//! Contains WindowType enum and WindowState for UI presentation.

use std::collections::{BTreeMap, BTreeSet};

use uuid::Uuid;

use crate::models::PartOfSpeech;

/// Window type enum for future extensibility.
///
/// Currently only Main is implemented. Additional window types
/// can be added here (e.g., Settings, ClozeBrowser).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowType {
    Main,
    // Future: Settings,
    // Future: ClozeBrowser,
}

impl WindowType {
    /// Returns the window settings for this window type.
    pub fn window_settings(&self) -> iced::window::Settings {
        match self {
            WindowType::Main => iced::window::Settings {
                exit_on_close_request: false,
                ..Default::default()
            },
        }
    }
}

/// Window content enum containing state for each window type.
#[derive(Debug)]
pub enum Window {
    Main(WindowState),
}

impl Window {
    /// Creates a new Window with the specified type.
    pub fn new(window_type: WindowType) -> Self {
        match window_type {
            WindowType::Main => Window::Main(WindowState::new()),
        }
    }
}

/// Dropdown state for tag operations.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TagDropdownState {
    #[default]
    None,
    Add,
    Remove,
}

/// Input state for creating/editing meanings.
#[derive(Debug, Clone)]
pub struct MeaningInputState {
    pub definition: String,
    pub pos: PartOfSpeech,
    pub visible: bool,
}

impl Default for MeaningInputState {
    fn default() -> Self {
        Self {
            definition: String::new(),
            pos: PartOfSpeech::Noun,
            visible: false,
        }
    }
}

/// UI state for the words view.
#[derive(Debug, Default)]
pub struct WordsUiState {
    pub word_input: String,
    pub meaning_inputs: BTreeMap<Uuid, MeaningInputState>,
    pub tag_filter: String,
    pub active_tag_dropdown: Option<Uuid>,
    pub meanings_tag_dropdown_state: TagDropdownState,
    pub meanings_tag_search_input: String,
    pub meanings_tag_remove_search_input: String,
}

impl WordsUiState {
    pub fn new() -> Self {
        Self {
            word_input: String::new(),
            meaning_inputs: BTreeMap::new(),
            tag_filter: String::new(),
            active_tag_dropdown: None,
            meanings_tag_dropdown_state: TagDropdownState::None,
            meanings_tag_search_input: String::new(),
            meanings_tag_remove_search_input: String::new(),
        }
    }
}

/// UI state for the tags view.
#[derive(Debug, Default)]
pub struct TagsUiState {
    pub input: String,
    pub collapsed_ids: BTreeSet<Uuid>,
}

impl TagsUiState {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            collapsed_ids: BTreeSet::new(),
        }
    }
}

/// UI state for the queue view.
#[derive(Debug, Default)]
pub struct QueueUiState {}

impl QueueUiState {
    pub fn new() -> Self {
        Self {}
    }
}

/// State for the main application window.
///
/// Contains all UI state: selection, expansion, inputs, and dropdowns.
#[derive(Debug, Default)]
pub struct WindowState {
    // Selection
    pub selected_word_ids: BTreeSet<Uuid>,
    pub selected_meaning_ids: BTreeSet<Uuid>,
    pub selected_tag_ids: BTreeSet<Uuid>,
    // Expansion
    pub expanded_word_ids: BTreeSet<Uuid>,
    // UI State
    pub words_ui: WordsUiState,
    pub tags_ui: TagsUiState,
    pub queue_ui: QueueUiState,
}

impl WindowState {
    /// Creates a new WindowState.
    pub fn new() -> Self {
        Self {
            selected_word_ids: BTreeSet::new(),
            selected_meaning_ids: BTreeSet::new(),
            selected_tag_ids: BTreeSet::new(),
            expanded_word_ids: BTreeSet::new(),
            words_ui: WordsUiState::new(),
            tags_ui: TagsUiState::new(),
            queue_ui: QueueUiState::new(),
        }
    }
}
