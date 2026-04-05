//! Edit buffers for word and meaning forms.

use crate::models::{CefrLevel, PartOfSpeech};
use langtag::LangTagBuf;

#[derive(Debug, Clone, Default)]
pub struct WordEditBuffer {
    pub content: String,
    pub language: Option<LangTagBuf>,
    pub language_input: String,
}

impl WordEditBuffer {
    pub fn clear(&mut self) {
        self.content.clear();
        self.language = None;
        self.language_input.clear();
    }
}

#[derive(Debug, Clone, Default)]
pub struct MeaningEditBuffer {
    pub definition: String,
    pub pos: PartOfSpeech,
    pub cefr: Option<CefrLevel>,
}

impl MeaningEditBuffer {
    pub fn clear(&mut self) {
        self.definition.clear();
        self.pos = PartOfSpeech::Noun;
        self.cefr = None;
    }
}
