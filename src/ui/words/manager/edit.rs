//! Edit session state management.

use langtag::LangTagBuf;

use crate::models::types::{MeaningId, WordId};
use crate::models::{CefrLevel, PartOfSpeech};

/// Detail panel editing context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditContext {
    /// Not editing anything
    #[default]
    None,
    /// Creating a new word
    NewWord,
    /// Creating a new meaning for a word
    NewMeaning(WordId),
    /// Editing a word
    Word(WordId),
    /// Editing a meaning
    Meaning(MeaningId),
}

impl EditContext {
    /// Clear editing context.
    pub fn clear(&mut self) {
        *self = EditContext::None;
    }
}

/// Buffer for storing edits in progress.
#[derive(Debug, Clone, Default)]
pub struct EditBuffer {
    /// Word content being edited
    pub word_content: String,
    /// Word language being edited
    pub word_language: Option<LangTagBuf>,
    /// Meaning definition being edited
    pub meaning_definition: String,
    /// Meaning part of speech being edited
    pub meaning_pos: PartOfSpeech,
    /// Meaning CEFR level being edited
    pub meaning_cefr: Option<CefrLevel>,
}

impl EditBuffer {
    /// Clear all fields for new word editing
    pub fn clear_new_word(&mut self) {
        self.word_content.clear();
        self.word_language = None;
        self.meaning_definition.clear();
        self.meaning_pos = PartOfSpeech::Noun;
        self.meaning_cefr = None;
    }

    /// Clear all fields for new meaning editing
    pub fn clear_new_meaning(&mut self) {
        self.meaning_definition.clear();
        self.meaning_pos = PartOfSpeech::Noun;
        self.meaning_cefr = None;
    }
}

/// Edit session state manager.
#[derive(Debug)]
pub struct EditManager {
    /// Current editing context
    context: EditContext,
    /// Edit buffer
    buffer: EditBuffer,
}

impl EditManager {
    /// Creates a new EditManager.
    pub fn new() -> Self {
        Self {
            context: EditContext::None,
            buffer: EditBuffer::default(),
        }
    }

    /// Start creating a new word.
    pub fn start_new_word(&mut self) {
        self.context = EditContext::NewWord;
        self.buffer.clear_new_word();
    }

    /// Start adding meaning to a word.
    pub fn start_add_meaning(&mut self, word_id: WordId) {
        self.context = EditContext::NewMeaning(word_id);
        self.buffer.clear_new_meaning();
    }

    /// Start editing a word.
    pub fn start_edit_word(
        &mut self,
        word_id: WordId,
        content: String,
        language: Option<LangTagBuf>,
    ) {
        self.context = EditContext::Word(word_id);
        self.buffer.word_content = content;
        self.buffer.word_language = language;
    }

    /// Start editing a meaning.
    pub fn start_edit_meaning(
        &mut self,
        meaning_id: MeaningId,
        definition: String,
        pos: PartOfSpeech,
        cefr: Option<CefrLevel>,
    ) {
        self.context = EditContext::Meaning(meaning_id);
        self.buffer.meaning_definition = definition;
        self.buffer.meaning_pos = pos;
        self.buffer.meaning_cefr = cefr;
    }

    /// Clear editing context.
    pub fn clear_context(&mut self) {
        self.context = EditContext::None;
    }

    /// Get current editing context.
    pub fn context(&self) -> EditContext {
        self.context
    }

    /// Update word content.
    pub fn update_word_content(&mut self, content: String) {
        self.buffer.word_content = content;
    }

    /// Update word language.
    pub fn update_word_language(&mut self, language: Option<LangTagBuf>) {
        self.buffer.word_language = language;
    }

    /// Update meaning definition.
    pub fn update_meaning_definition(&mut self, definition: String) {
        self.buffer.meaning_definition = definition;
    }

    /// Update meaning part of speech.
    pub fn update_meaning_pos(&mut self, pos: PartOfSpeech) {
        self.buffer.meaning_pos = pos;
    }

    /// Update meaning CEFR level.
    pub fn update_meaning_cefr(&mut self, cefr: Option<CefrLevel>) {
        self.buffer.meaning_cefr = cefr;
    }

    /// Get edit buffer.
    pub fn buffer(&self) -> &EditBuffer {
        &self.buffer
    }
}

impl Default for EditManager {
    fn default() -> Self {
        Self::new()
    }
}
