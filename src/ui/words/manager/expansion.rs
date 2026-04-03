//! Expansion state management.

use std::collections::HashSet;

use crate::models::types::WordId;

/// Expansion state manager for words tree.
#[derive(Debug)]
pub struct ExpansionManager {
    /// Expanded word IDs (words whose meanings are visible)
    expanded_words: HashSet<WordId>,
}

impl ExpansionManager {
    /// Creates a new ExpansionManager.
    pub fn new() -> Self {
        Self {
            expanded_words: HashSet::new(),
        }
    }

    /// Toggle word expansion.
    pub fn toggle(&mut self, word_id: WordId) {
        if self.expanded_words.contains(&word_id) {
            self.expanded_words.remove(&word_id);
        } else {
            self.expanded_words.insert(word_id);
        }
    }

    /// Expand a word.
    pub fn expand(&mut self, word_id: WordId) {
        self.expanded_words.insert(word_id);
    }

    /// Collapse a word.
    pub fn collapse(&mut self, word_id: WordId) {
        self.expanded_words.remove(&word_id);
    }

    /// Check if a word is expanded.
    pub fn is_expanded(&self, word_id: WordId) -> bool {
        self.expanded_words.contains(&word_id)
    }

    /// Expand all words.
    pub fn expand_all(&mut self, word_ids: impl IntoIterator<Item = WordId>) {
        self.expanded_words.extend(word_ids);
    }

    /// Collapse all words.
    pub fn collapse_all(&mut self) {
        self.expanded_words.clear();
    }
}

impl Default for ExpansionManager {
    fn default() -> Self {
        Self::new()
    }
}
