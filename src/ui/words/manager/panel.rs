//! Detail panel state management.
//!
//! Provides unified DetailPanelState enum and DetailPanelManager.

use crate::models::types::{ClozeId, MeaningId, WordId};
use crate::ui::words::manager::edit::{MeaningEditBuffer, WordEditBuffer};

/// Detail panel state - unified enum replacing (DetailSelection, EditContext).
#[derive(Debug, Clone)]
pub enum DetailPanelState {
    Empty,
    WordView { word_id: WordId },
    MeaningView { meaning_id: MeaningId },
    ClozeView { cloze_id: ClozeId },
    WordCreating,
    WordEditing { word_id: WordId },
    MeaningCreating { word_id: WordId },
    MeaningEditing { meaning_id: MeaningId },
}

impl DetailPanelState {
    pub fn is_editing(&self) -> bool {
        matches!(
            self,
            Self::WordCreating
                | Self::WordEditing { .. }
                | Self::MeaningCreating { .. }
                | Self::MeaningEditing { .. }
        )
    }
}

/// Tag dropdown target.
#[derive(Debug, Clone)]
pub enum TagDropdownTarget {
    SelectedMeanings,
    SingleMeaning(MeaningId),
}

/// State for the tag dropdown.
#[derive(Debug, Clone)]
pub struct TagDropdownState {
    pub target: TagDropdownTarget,
    pub search: String,
}

impl TagDropdownState {
    pub fn for_meaning(meaning_id: MeaningId) -> Self {
        Self {
            target: TagDropdownTarget::SingleMeaning(meaning_id),
            search: String::new(),
        }
    }

    pub fn for_batch() -> Self {
        Self {
            target: TagDropdownTarget::SelectedMeanings,
            search: String::new(),
        }
    }
}

/// Detail panel state manager.
#[derive(Debug)]
pub struct DetailPanelManager {
    state: DetailPanelState,
    pub word_buffer: WordEditBuffer,
    pub meaning_buffer: MeaningEditBuffer,
    tag_dropdown: Option<TagDropdownState>,
}

impl DetailPanelManager {
    pub fn new() -> Self {
        Self {
            state: DetailPanelState::Empty,
            word_buffer: WordEditBuffer::default(),
            meaning_buffer: MeaningEditBuffer::default(),
            tag_dropdown: None,
        }
    }

    pub fn state(&self) -> &DetailPanelState {
        &self.state
    }

    // === View State Transitions ===

    pub fn show_word(&mut self, word_id: WordId) {
        self.state = DetailPanelState::WordView { word_id };
    }

    pub fn show_meaning(&mut self, meaning_id: MeaningId) {
        self.state = DetailPanelState::MeaningView { meaning_id };
    }

    pub fn show_cloze(&mut self, cloze_id: ClozeId) {
        self.state = DetailPanelState::ClozeView { cloze_id };
    }

    pub fn close(&mut self) {
        self.state = DetailPanelState::Empty;
    }

    // === Edit State Transitions ===

    pub fn start_word_create(&mut self) {
        self.word_buffer.clear();
        self.meaning_buffer.clear();
        self.state = DetailPanelState::WordCreating;
    }

    pub fn start_word_edit(
        &mut self,
        word_id: WordId,
        content: String,
        language: Option<langtag::LangTagBuf>,
    ) {
        self.word_buffer.content = content;
        self.word_buffer.language = language.clone();
        self.word_buffer.language_input =
            language.as_ref().map(|l| l.to_string()).unwrap_or_default();
        self.state = DetailPanelState::WordEditing { word_id };
    }

    pub fn start_meaning_create(&mut self, word_id: WordId) {
        self.meaning_buffer.clear();
        self.state = DetailPanelState::MeaningCreating { word_id };
    }

    pub fn start_meaning_edit(
        &mut self,
        meaning_id: MeaningId,
        definition: String,
        pos: crate::models::PartOfSpeech,
        cefr: Option<crate::models::CefrLevel>,
    ) {
        self.meaning_buffer.definition = definition;
        self.meaning_buffer.pos = pos;
        self.meaning_buffer.cefr = cefr;
        self.state = DetailPanelState::MeaningEditing { meaning_id };
    }

    // === Tag Dropdown ===

    pub fn open_tag_dropdown(&mut self, target: TagDropdownTarget) {
        self.tag_dropdown = Some(match target {
            TagDropdownTarget::SingleMeaning(id) => TagDropdownState::for_meaning(id),
            TagDropdownTarget::SelectedMeanings => TagDropdownState::for_batch(),
        });
    }

    pub fn close_tag_dropdown(&mut self) {
        self.tag_dropdown = None;
    }

    pub fn tag_dropdown(&self) -> Option<&TagDropdownState> {
        self.tag_dropdown.as_ref()
    }

    pub fn tag_dropdown_mut(&mut self) -> Option<&mut TagDropdownState> {
        self.tag_dropdown.as_mut()
    }
}

impl Default for DetailPanelManager {
    fn default() -> Self {
        Self::new()
    }
}
