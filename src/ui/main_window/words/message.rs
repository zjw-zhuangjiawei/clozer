//! Words panel message types.

use crate::models::PartOfSpeech;
use uuid::Uuid;

/// Messages for the words panel.
#[derive(Debug, Clone)]
pub enum WordsMessage {
    // Word CRUD
    InputChanged(String),
    CreateWord(String),
    DeleteWord(Uuid),
    DeleteSelected,
    ToggleWordExpand(Uuid),

    // Selection
    ToggleWord(Uuid),
    ToggleMeaning(Uuid),
    SelectAllWords,
    DeselectAllWords,

    // Filtering
    TagFilterChanged(String),
    ClearTagFilter,

    // Meaning input
    ToggleMeaningInput(Uuid),
    MeaningDefInputChanged(Uuid, String),
    MeaningPosSelected(Uuid, PartOfSpeech),
    SaveMeaning(Uuid),
    CancelMeaningInput(Uuid),
    DeleteMeaning(Uuid),

    // Per-meaning tag operations
    MeaningToggleTagDropdown(Uuid),
    MeaningTagSearchChanged(String),
    AddTagToMeaningSearch(Uuid, String),
    AddTagToMeaning(Uuid, Uuid),
    RemoveTagFromMeaning(Uuid, Uuid),

    // Batch tag operations
    ToggleBatchAddTagDropdown,
    ToggleBatchRemoveTagDropdown,
    BatchTagSearchChanged(String),
    BatchTagRemoveSearchChanged(String),
    BatchAddTagToSelectedMeanings(Uuid),
    BatchRemoveTagFromSelectedMeanings(Uuid),

    // Tag CRUD (triggered from words view)
    CreateTag(String),
    DeleteTag(Uuid),

    // Queue trigger
    QueueSelected,
}
