//! Words panel message types.
//!
//! Messages are organized hierarchically by domain:
//! - Search: Query handling
//! - Filter: Cloze status and tag filtering
//! - Selection: Meaning and cloze selection
//! - Detail: Detail panel selection and editing
//! - Word: Word CRUD operations
//! - Meaning: Meaning CRUD operations
//! - Tag: Tag operations
//! - Cloze: Cloze operations
//! - Batch: Batch operations on selections
//! - Export: Export operations

use crate::models::types::{ClozeId, MeaningId, TagId, WordId};
use crate::models::{CefrLevel, PartOfSpeech};
use crate::ui::words::state::ClozeFilter;
use strum::{Display, VariantArray};

// ============================================================================
// Root Message Enum
// ============================================================================

/// Root message enum for Words panel.
///
/// Delegates to domain-specific message handlers.
#[derive(Debug, Clone)]
pub enum WordsMessage {
    /// Search-related messages
    Search(SearchMessage),
    /// Filter-related messages
    Filter(FilterMessage),
    /// Selection-related messages
    Selection(SelectionMessage),
    /// Detail panel messages
    Detail(DetailMessage),
    /// Word CRUD messages
    Word(WordMessage),
    /// Meaning CRUD messages
    Meaning(MeaningMessage),
    /// Tag operation messages
    Tag(TagMessage),
    /// Cloze operation messages
    Cloze(ClozeMessage),
    /// Batch operation messages
    Batch(BatchMessage),
    /// Export operation messages
    Export(ExportMessage),
}

// ============================================================================
// Domain-Specific Messages
// ============================================================================

/// Search-related messages.
#[derive(Debug, Clone)]
pub enum SearchMessage {
    /// Search query changed
    QueryChanged(String),
    /// Clear search query
    Clear,
}

/// Filter-related messages.
#[derive(Debug, Clone)]
pub enum FilterMessage {
    /// Filter by cloze generation status
    ByClozeStatus(ClozeFilter),
    /// Filter by tag
    ByTag(Option<TagId>),
    /// Clear all filters
    Clear,
}

/// Selection-related messages.
#[derive(Debug, Clone)]
pub enum SelectionMessage {
    /// Toggle selection for all meanings of a word
    ToggleWord(WordId),
    /// Toggle selection for a single meaning
    ToggleMeaning(MeaningId),
    /// Toggle selection for a single cloze
    ToggleCloze(ClozeId),
    /// Select all meanings
    SelectAll,
    /// Deselect all
    DeselectAll,
}

/// Detail panel messages.
#[derive(Debug, Clone)]
pub enum DetailMessage {
    /// Select word detail
    SelectWord(WordId),
    /// Select meaning detail
    SelectMeaning(MeaningId),
    /// Select cloze detail
    SelectCloze(ClozeId),
    /// Clear detail selection
    Clear,
    /// Start editing a word
    StartEditWord(WordId),
    /// Start editing a meaning
    StartEditMeaning(MeaningId),
    /// Edit word content input
    EditWordContent(String),
    /// Edit meaning definition input
    EditMeaningDefinition(String),
    /// Edit meaning part of speech
    EditMeaningPos(PartOfSpeech),
    /// Edit meaning CEFR level
    EditMeaningCefr(Option<CefrLevel>),
    /// Save current edit
    Save,
    /// Cancel current edit
    Cancel,
}

/// Word CRUD messages.
#[derive(Debug, Clone)]
pub enum WordMessage {
    /// Create a new word
    Create { content: String },
    /// Delete a word
    Delete { id: WordId },
    /// Expand a word (show meanings)
    Expand { id: WordId },
    /// Collapse a word (hide meanings)
    Collapse { id: WordId },
    /// Expand all words
    ExpandAll,
    /// Collapse all words
    CollapseAll,
}

/// Meaning CRUD messages.
#[derive(Debug, Clone)]
pub enum MeaningMessage {
    /// Start adding meaning to a word
    AddStart { word_id: WordId },
    /// Input meaning definition
    AddInput { definition: String },
    /// Select meaning part of speech
    AddPos { pos: PartOfSpeech },
    /// Select meaning CEFR level
    AddCefr { level: Option<CefrLevel> },
    /// Save new meaning
    AddSave,
    /// Cancel adding meaning
    AddCancel,
    /// Delete a meaning
    Delete { id: MeaningId },
}

/// Tag operation messages.
#[derive(Debug, Clone)]
pub enum TagMessage {
    /// Show tag dropdown for a meaning
    ShowDropdown { meaning_id: MeaningId },
    /// Show tag dropdown for batch operation on selected meanings
    ShowBatchDropdown,
    /// Tag search query changed
    Search { query: String },
    /// Add tag to a meaning
    AddToMeaning {
        meaning_id: MeaningId,
        tag_id: TagId,
    },
    /// Add tag to all selected meanings
    AddToSelected { tag_id: TagId },
    /// Remove tag from a meaning
    RemoveFromMeaning {
        meaning_id: MeaningId,
        tag_id: TagId,
    },
    /// Quick create tag and add to meaning
    QuickCreate { meaning_id: MeaningId, name: String },
    /// Close tag dropdown
    Close,
}

/// Cloze operation messages.
#[derive(Debug, Clone)]
pub enum ClozeMessage {
    /// Delete a cloze
    Delete { id: ClozeId },
    /// Toggle cloze selection (independent of meaning selection)
    ToggleSelection { id: ClozeId },
}

/// Batch operation messages.
#[derive(Debug, Clone)]
pub enum BatchMessage {
    /// Queue all selected meanings for generation
    QueueSelected,
    /// Delete all selected meanings
    DeleteSelected,
    /// Delete all selected clozes
    DeleteSelectedClozes,
}

/// Export operation messages.
#[derive(Debug, Clone)]
pub enum ExportMessage {
    /// Export to plaintext
    ToPlaintext,
    /// Export to Typst PDF
    ToTypstPdf,
}

/// Export kind for the export dropdown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, VariantArray)]
pub enum ExportKind {
    Plaintext,
    TypstPdf,
}
