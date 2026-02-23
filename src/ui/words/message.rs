//! Words panel message types.

use crate::models::{CefrLevel, PartOfSpeech};
use crate::ui::words::state::ClozeFilter;
use strum::{Display, VariantArray};
use uuid::Uuid;

/// Export kind for the export dropdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum ExportKind {
    Plaintext,
    TypstPdf,
}

/// Messages for the words panel.
#[derive(Debug, Clone)]
pub enum WordsMessage {
    // Search & Filter
    SearchChanged(String),
    FilterByClozeStatus(ClozeFilter),
    FilterByTag(Option<Uuid>),
    ClearFilter,

    // Selection (for batch operations)
    ToggleWordSelection(Uuid),
    ToggleMeaningSelection(Uuid),
    SelectAll,
    DeselectAll,

    // Detail panel (toggle by clicking)
    ToggleWordDetail(Uuid),
    ToggleMeaningDetail(Uuid),
    ToggleClozeDetail(Uuid),
    ClearDetailSelection,

    // Detail panel editing
    StartEditWord(Uuid),
    StartEditMeaning(Uuid),
    EditWordInput(String),
    EditMeaningDefinition(String),
    EditMeaningPos(PartOfSpeech),
    EditMeaningCefr(Option<CefrLevel>),
    SaveEdit,
    CancelEdit,

    // Expand/Collapse
    ToggleWordExpand(Uuid),
    ExpandAll,
    CollapseAll,

    // Word operations
    CreateWord(String),
    DeleteWord(Uuid),

    // Meaning operations
    AddMeaningStart(Uuid),
    AddMeaningInput(String),
    AddMeaningPosSelected(PartOfSpeech),
    AddMeaningCefrSelected(Option<CefrLevel>),
    AddMeaningSave,
    AddMeaningCancel,
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
    DeleteCloze(Uuid),
    ToggleClozeSelection(Uuid),

    // Batch operations
    QueueSelected,
    DeleteSelected,
    DeleteSelectedClozes,

    // Export operations
    ExportSelected(ExportKind),
}
