//! Words panel message types.

use crate::models::PartOfSpeech;
use crate::ui::main_window::words::state::ClozeFilter;
use uuid::Uuid;

/// Messages for the words panel.
#[derive(Debug, Clone)]
pub enum WordsMessage {
    // Search & Filter
    SearchChanged(String),
    FilterByClozeStatus(ClozeFilter),
    FilterByTag(Option<Uuid>),
    ClearFilter,

    // Selection
    ToggleWordSelection(Uuid),
    ToggleMeaningSelection(Uuid),
    SelectAll,
    DeselectAll,

    // Expand/Collapse
    ToggleWordExpand(Uuid),
    ToggleClozeExpand(Uuid),
    ExpandAll,
    CollapseAll,

    // Word operations
    CreateWord(String),
    EditWordStart(Uuid),
    EditWordInput(String),
    EditWordSave(Uuid),
    EditWordCancel,
    DeleteWord(Uuid),

    // Meaning operations
    AddMeaningStart(Uuid),
    AddMeaningInput(String),
    AddMeaningPosSelected(PartOfSpeech),
    AddMeaningSave,
    AddMeaningCancel,
    EditMeaningStart(Uuid),
    EditMeaningInput(String),
    EditMeaningSave(Uuid),
    EditMeaningCancel,
    DeleteMeaning(Uuid),

    // Tag operations
    ShowTagDropdown(Uuid),
    ShowBatchTagDropdown,
    TagSearchChanged(String),
    AddTagToMeaning(Uuid, Uuid),
    AddTagToSelected(Uuid),
    RemoveTagFromMeaning(Uuid, Uuid),
    QuickCreateTag(Uuid, String), // Create tag and add to meaning
    CloseTagDropdown,

    // Cloze operations
    RegenerateCloze(Uuid),
    DeleteCloze(Uuid),
    ToggleClozeSelection(Uuid),

    // Batch operations
    QueueSelected,
    DeleteSelected,
    DeleteSelectedClozes,
}
