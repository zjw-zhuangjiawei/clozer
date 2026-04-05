//! Search and filter state management.

use crate::models::Word;
use crate::models::types::{TagId, WordId};
use crate::query::{QueryAST, SortType};
use crate::registry::MeaningRegistry;

/// Search and filter state manager.
#[derive(Debug)]
pub struct SearchManager {
    /// Current search query
    pub query: String,
    /// Parsed query AST
    pub ast: QueryAST,
    /// Current sort type
    pub current_sort: SortType,
    /// Search results (word IDs with scores)
    pub search_results: Vec<(WordId, i32)>,
    /// Tag filter (None = no tag filter)
    pub tag_filter: Option<TagId>,
}

impl SearchManager {
    /// Creates a new SearchManager.
    pub fn new() -> Self {
        Self {
            query: String::new(),
            ast: QueryAST::new(),
            current_sort: SortType::default(),
            search_results: Vec::new(),
            tag_filter: None,
        }
    }

    /// Sets the search query.
    pub fn set_query(&mut self, query: String) {
        self.query = query.clone();
        self.ast = crate::query::parse::parse_query(&query);
    }

    /// Clears the search query.
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.ast = QueryAST::new();
        self.search_results.clear();
    }

    /// Sets the tag filter.
    pub fn set_tag_filter(&mut self, tag_id: Option<TagId>) {
        self.tag_filter = tag_id;
    }

    /// Clears all filters.
    pub fn clear_filters(&mut self) {
        self.query.clear();
        self.ast = QueryAST::new();
        self.search_results.clear();
        self.tag_filter = None;
    }

    /// Checks if any filters are active.
    pub fn has_active_filters(&self) -> bool {
        !self.query.is_empty() || !self.search_results.is_empty() || self.tag_filter.is_some()
    }

    /// Filters words based on current search and filter conditions.
    pub fn filter_words<'a>(
        &self,
        word_iter: impl Iterator<Item = &'a Word>,
        meaning_registry: &MeaningRegistry,
    ) -> Vec<WordId> {
        let query_lower = self.query.to_lowercase();
        let mut results: Vec<WordId> = Vec::new();

        for word in word_iter {
            if !query_lower.is_empty() {
                let matches = word.content.to_lowercase().contains(&query_lower)
                    || self.has_matching_meaning(word, &query_lower, meaning_registry);
                if !matches {
                    continue;
                }
            }

            if let Some(tag_id) = self.tag_filter {
                let has_tag = meaning_registry
                    .iter_by_word(word.id)
                    .any(|(_, m)| m.tag_ids.contains(&tag_id));
                if !has_tag {
                    continue;
                }
            }

            results.push(word.id);
        }

        results
    }

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
