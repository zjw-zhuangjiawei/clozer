//! Search and filter state management.

use crate::models::Word;
use crate::models::types::{TagId, WordId};
use crate::registry::{ClozeRegistry, MeaningRegistry};
use crate::ui::words::ClozeFilter;

/// Search and filter state manager.
#[derive(Debug)]
pub struct SearchManager {
    /// Current search query
    pub query: String,
    /// Cloze status filter
    pub cloze_filter: ClozeFilter,
    /// Tag filter (None = no tag filter)
    pub tag_filter: Option<TagId>,
}

impl SearchManager {
    /// Creates a new SearchManager.
    pub fn new() -> Self {
        Self {
            query: String::new(),
            cloze_filter: ClozeFilter::All,
            tag_filter: None,
        }
    }

    /// Sets the search query.
    pub fn set_query(&mut self, query: String) {
        self.query = query;
    }

    /// Clears the search query.
    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    /// Sets the cloze filter.
    pub fn set_cloze_filter(&mut self, filter: ClozeFilter) {
        self.cloze_filter = filter;
    }

    /// Sets the tag filter.
    pub fn set_tag_filter(&mut self, tag_id: Option<TagId>) {
        self.tag_filter = tag_id;
    }

    /// Clears all filters.
    pub fn clear_filters(&mut self) {
        self.query.clear();
        self.cloze_filter = ClozeFilter::All;
        self.tag_filter = None;
    }

    /// Checks if any filters are active.
    pub fn has_active_filters(&self) -> bool {
        !self.query.is_empty() || self.cloze_filter != ClozeFilter::All || self.tag_filter.is_some()
    }

    /// Filters words based on current search and filter conditions.
    pub fn filter_words<'a>(
        &self,
        word_iter: impl Iterator<Item = &'a Word>,
        meaning_registry: &MeaningRegistry,
        cloze_registry: &ClozeRegistry,
    ) -> Vec<WordId> {
        let query_lower = self.query.to_lowercase();
        let mut results: Vec<WordId> = Vec::new();

        for word in word_iter {
            // Text search filter
            if !query_lower.is_empty() {
                let matches = word.content.to_lowercase().contains(&query_lower)
                    || self.has_matching_meaning(word, &query_lower, meaning_registry);
                if !matches {
                    continue;
                }
            }

            // Tag filter
            if let Some(tag_id) = self.tag_filter {
                let has_tag = meaning_registry
                    .iter_by_word(word.id)
                    .any(|(_, m)| m.tag_ids.contains(&tag_id));
                if !has_tag {
                    continue;
                }
            }

            // Cloze filter
            match self.cloze_filter {
                ClozeFilter::All => {}
                ClozeFilter::HasClozes => {
                    let has_clozes = meaning_registry
                        .iter_by_word(word.id)
                        .any(|(_, m)| cloze_registry.iter_by_meaning_id(m.id).next().is_some());
                    if !has_clozes {
                        continue;
                    }
                }
                ClozeFilter::Pending => {
                    let has_pending = meaning_registry
                        .iter_by_word(word.id)
                        .any(|(_, m)| cloze_registry.iter_by_meaning_id(m.id).next().is_none());
                    if !has_pending {
                        continue;
                    }
                }
            }

            results.push(word.id);
        }

        results
    }

    /// Checks if a word has a meaning that matches the query.
    fn has_matching_meaning(
        &self,
        word: &Word,
        query: &str,
        meaning_registry: &MeaningRegistry,
    ) -> bool {
        meaning_registry
            .iter_by_word(word.id)
            .any(|(_, m)| m.definition.to_lowercase().contains(query))
    }
}

impl Default for SearchManager {
    fn default() -> Self {
        Self::new()
    }
}
