//! Application messages for event handling.
//!
//! Contains all message variants used in the Elm-like architecture.
//! The Message enum is flat - all application messages are at the top level.

use uuid::Uuid;

use crate::models::PartOfSpeech;
use crate::state::QueueGenerationResult;
use crate::window::WindowType;

/// Application messages for event handling.
///
/// This is a flat enum containing all window management messages
/// and application operation messages.
#[derive(Debug, Clone)]
pub enum Message {
    // Window management
    WindowOpened(iced::window::Id, WindowType),
    WindowClosed(iced::window::Id),

    // Word operations
    CreateWord(String),
    DeleteWord(Uuid),
    DeleteSelected,
    ToggleWordExpand(Uuid),

    // Meaning operations
    CreateMeaning(Uuid, String, PartOfSpeech), // word_id, definition, pos
    SaveMeaning(Uuid),                         // word_id (create from input)
    ToggleMeaning(Uuid),                       // meaning_id
    CancelMeaningInput(Uuid),                  // word_id
    ToggleMeaningInput(Uuid),                  // word_id
    DeleteMeaning(Uuid),
    MeaningDefInputChanged(Uuid, String), // word_id, definition
    MeaningPosSelected(Uuid, PartOfSpeech), // word_id, pos

    // Tag operations
    CreateTag(String),
    DeleteTag(Uuid),
    AddTagToMeaning(Uuid, Uuid),         // meaning_id, tag_id
    RemoveTagFromMeaning(Uuid, Uuid),    // meaning_id, tag_id
    WordsMeaningToggleTagDropdown(Uuid), // meaning_id
    WordsMeaningTagSearchChanged(String),
    AddTagToMeaningSearch(Uuid, String), // meaning_id, tag_name

    // Batch tag operations for selected meanings
    BatchAddTagToSelectedMeanings(Uuid),      // tag_id
    BatchRemoveTagFromSelectedMeanings(Uuid), // tag_id
    ToggleMeaningsAddTagDropdown,             // Toggle Add Tag dropdown
    ToggleMeaningsRemoveTagDropdown,          // Toggle Remove Tag dropdown
    MeaningsTagSearchChanged(String),         // Search for add
    MeaningsTagRemoveSearchChanged(String),   // Search for remove

    // Selection - Words
    ToggleWord(Uuid),
    SelectAllWords,
    DeselectAllWords,

    // Selection - Tags
    ToggleTag(Uuid),
    SelectAllTags,
    DeselectAllTags,

    // UI - Words
    WordsInputChanged(String),
    WordsTagFilterChanged(String),
    WordsClearTagFilter,

    // UI - Tags
    TagsInputChanged(String),
    TagsToggleCollapse(Uuid),
    TagsSelectTag(Uuid),
    TagsDeselectTag(Uuid),

    // Queue
    QueueSelectToggle(Uuid),
    QueueSelectAll,
    QueueDeselectAll,
    QueueSelected,
    QueueProcess,
    QueueClearCompleted,
    QueueRemove(Uuid),
    QueueGenerationResult(QueueGenerationResult),
}
