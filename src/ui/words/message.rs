//! Words panel message types.
//!
//! Messages are flattened - each operation is a direct variant with all needed data.

use crate::models::types::{ClozeId, MeaningId, TagId, WordId};
use crate::models::{CefrLevel, PartOfSpeech};
use crate::ui::words::state::ClozeFilter;

/// Flattened message enum for Words panel.
///
/// All operations are direct variants, no nested message types.
#[derive(Debug, Clone)]
pub enum WordsMessage {
    // Search
    /// Search query changed
    SearchQueryChanged(String),
    /// Clear search query
    SearchCleared,

    // Filter
    /// Filter by cloze generation status
    ClozeFilterChanged(ClozeFilter),
    /// Filter by tag
    TagFilterChanged(Option<TagId>),
    /// Clear all filters
    FiltersCleared,

    // Selection
    /// Toggle selection for all meanings of a word
    WordToggled(WordId),
    /// Toggle selection for a single meaning
    MeaningToggled(MeaningId),
    /// Toggle selection for a single cloze
    ClozeToggled(ClozeId),
    /// Select all meanings
    SelectAllTriggered,
    /// Deselect all
    DeselectAllTriggered,

    // Detail panel selection
    /// Select word detail
    WordSelected(WordId),
    /// Select meaning detail
    MeaningSelected(MeaningId),
    /// Select cloze detail
    ClozeSelected(ClozeId),
    /// Clear detail selection
    DetailClosed,

    // Detail panel editing - start operations
    /// Start creating a new word
    NewWordStarted,
    /// Start adding a meaning to a word
    AddMeaningStarted(WordId),
    /// Start editing a word
    EditWordStarted(WordId),
    /// Start editing a meaning
    EditMeaningStarted(MeaningId),

    // Detail panel editing - field updates
    /// Edit word content input
    EditWordContentChanged(String),
    /// Edit word language (for editing existing word)
    EditWordLanguageChanged(Option<langtag::LangTagBuf>),
    /// Edit new word content (for NewWord context)
    EditNewWordContentChanged(String),
    /// Edit new word language (for NewWord context)
    EditNewWordLanguageChanged(Option<langtag::LangTagBuf>),
    /// Edit meaning definition input
    EditMeaningDefinitionChanged(String),
    /// Edit meaning part of speech
    EditMeaningPosChanged(PartOfSpeech),
    /// Edit meaning CEFR level
    EditMeaningCefrChanged(Option<CefrLevel>),

    // Detail panel editing - save/cancel
    /// Save current edit (for Word/Meaning edit contexts)
    EditSaved,
    /// Save new word (for NewWord context)
    NewWordSaved,
    /// Save new meaning (for NewMeaning context)
    NewMeaningSaved,
    /// Cancel current edit
    EditCancelled,

    // Word CRUD
    /// Create a new word (from inline input)
    WordCreated { content: String },
    /// Delete a word
    WordDeleted(WordId),
    /// Expand a word (show meanings)
    WordExpanded(WordId),
    /// Collapse a word (hide meanings)
    WordCollapsed(WordId),
    /// Expand all words
    WordsExpandedAll,
    /// Collapse all words
    WordsCollapsedAll,

    // Meaning CRUD
    /// Start adding meaning to a word (inline form)
    MeaningAddStarted { word_id: WordId },
    /// Input meaning definition (inline form)
    MeaningAddInput { definition: String },
    /// Select meaning part of speech (inline form)
    MeaningAddPos { pos: PartOfSpeech },
    /// Select meaning CEFR level (inline form)
    MeaningAddCefr { level: Option<CefrLevel> },
    /// Save new meaning (inline form)
    MeaningAddSaved,
    /// Cancel adding meaning (inline form)
    MeaningAddCancelled,
    /// Delete a meaning
    MeaningDeleted(MeaningId),

    // Tag operations
    /// Show tag dropdown for a meaning
    TagDropdownOpened { for_meaning: MeaningId },
    /// Show tag dropdown for batch operation on selected meanings
    TagBatchDropdownOpened,
    /// Tag search query changed
    TagSearchChanged(String),
    /// Add tag to a meaning
    TagAddedToMeaning {
        meaning_id: MeaningId,
        tag_id: TagId,
    },
    /// Add tag to all selected meanings
    TagAddedToSelected { tag_id: TagId },
    /// Remove tag from a meaning
    TagRemovedFromMeaning {
        meaning_id: MeaningId,
        tag_id: TagId,
    },
    /// Quick create tag and add to meaning
    TagQuickCreated { meaning_id: MeaningId, name: String },
    /// Close tag dropdown
    TagDropdownClosed,

    // Cloze operations
    /// Delete a cloze
    ClozeDeleted(ClozeId),

    // Batch operations
    /// Queue all selected meanings for generation
    MeaningsQueuedForGeneration,
    /// Delete all selected meanings
    MeaningsDeleted,
    /// Delete all selected clozes
    ClozesDeleted,

    // Export operations
    /// Export to plaintext
    ExportPlaintext,
}
