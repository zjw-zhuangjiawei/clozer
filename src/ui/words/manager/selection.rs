//! Selection state management.

use std::collections::HashSet;

use crate::models::Word;
use crate::models::types::{ClozeId, MeaningId};
use crate::registry::MeaningRegistry;

/// Selection state manager.
#[derive(Debug)]
pub struct SelectionManager {
    /// Selected meaning IDs
    selected_meanings: HashSet<MeaningId>,
    /// Selected cloze IDs (independent of meaning selection)
    selected_clozes: HashSet<ClozeId>,
}

impl SelectionManager {
    /// Creates a new SelectionManager.
    pub fn new() -> Self {
        Self {
            selected_meanings: HashSet::new(),
            selected_clozes: HashSet::new(),
        }
    }

    /// Toggle word selection (select all meanings or deselect all).
    pub fn toggle_word(&mut self, word: &Word) {
        if self.is_word_selected(word) {
            for mid in &word.meaning_ids {
                self.selected_meanings.remove(mid);
            }
        } else {
            self.selected_meanings.extend(word.meaning_ids.iter());
        }
    }

    /// Check if a word is "fully selected" (all its meanings are selected).
    pub fn is_word_selected(&self, word: &Word) -> bool {
        if word.meaning_ids.is_empty() {
            return false;
        }
        word.meaning_ids
            .iter()
            .all(|mid| self.selected_meanings.contains(mid))
    }

    /// Check if a word is "partially selected" (some but not all meanings selected).
    pub fn is_word_partial(&self, word: &Word) -> bool {
        if word.meaning_ids.is_empty() {
            return false;
        }
        let selected_count = word
            .meaning_ids
            .iter()
            .filter(|mid| self.selected_meanings.contains(*mid))
            .count();
        selected_count > 0 && selected_count < word.meaning_ids.len()
    }

    /// Toggle a single meaning's selection.
    pub fn toggle_meaning(&mut self, meaning_id: MeaningId) {
        if self.selected_meanings.contains(&meaning_id) {
            self.selected_meanings.remove(&meaning_id);
        } else {
            self.selected_meanings.insert(meaning_id);
        }
    }

    /// Check if a meaning is selected.
    pub fn is_meaning_selected(&self, meaning_id: MeaningId) -> bool {
        self.selected_meanings.contains(&meaning_id)
    }

    /// Remove a meaning from selection.
    pub fn remove_meaning(&mut self, meaning_id: &MeaningId) {
        self.selected_meanings.remove(meaning_id);
    }

    /// Select all meanings in the registry.
    pub fn select_all_meanings(&mut self, registry: &MeaningRegistry) {
        for (id, _) in registry.iter() {
            self.selected_meanings.insert(*id);
        }
    }

    /// Clear meaning selections.
    pub fn clear_meanings(&mut self) {
        self.selected_meanings.clear();
    }

    /// Check if a cloze is selected.
    pub fn is_cloze_selected(&self, cloze_id: ClozeId) -> bool {
        self.selected_clozes.contains(&cloze_id)
    }

    /// Toggle a cloze's selection.
    pub fn toggle_cloze(&mut self, cloze_id: ClozeId) {
        if self.selected_clozes.contains(&cloze_id) {
            self.selected_clozes.remove(&cloze_id);
        } else {
            self.selected_clozes.insert(cloze_id);
        }
    }

    /// Clear cloze selections.
    pub fn clear_clozes(&mut self) {
        self.selected_clozes.clear();
    }

    /// Get the count of selected meanings.
    pub fn meaning_count(&self) -> usize {
        self.selected_meanings.len()
    }

    /// Get the count of selected clozes.
    pub fn cloze_count(&self) -> usize {
        self.selected_clozes.len()
    }

    /// Get total selection count (meanings + clozes).
    pub fn total_count(&self) -> usize {
        self.selected_meanings.len() + self.selected_clozes.len()
    }

    /// Check if there are any selections.
    pub fn has_selection(&self) -> bool {
        !self.selected_meanings.is_empty() || !self.selected_clozes.is_empty()
    }

    /// Clear all selections.
    pub fn clear_all(&mut self) {
        self.selected_meanings.clear();
        self.selected_clozes.clear();
    }

    /// Get reference to selected meanings.
    pub fn selected_meanings(&self) -> &HashSet<MeaningId> {
        &self.selected_meanings
    }

    /// Get reference to selected clozes.
    pub fn selected_clozes(&self) -> &HashSet<ClozeId> {
        &self.selected_clozes
    }
}

impl Default for SelectionManager {
    fn default() -> Self {
        Self::new()
    }
}
