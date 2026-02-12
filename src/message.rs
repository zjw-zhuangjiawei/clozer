//! Application messages for event handling.
//!
//! Contains all message variants used in the Elm-like architecture.

use uuid::Uuid;

use crate::models::PartOfSpeech;
use crate::state::QueueGenerationResult;

/// Application messages for event handling.
#[derive(Debug, Clone)]
pub enum Message {
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

    // Cloze operations
    DeleteCloze(Uuid),
    CreateCloze(Uuid, String),       // meaning_id, sentence
    ClozeInputChanged(Uuid, String), // meaning_id, sentence

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
