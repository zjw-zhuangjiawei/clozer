//! Search and filter state management.

use crate::models::types::WordId;
use crate::query::{Query, QueryEngine, SortType, TagResolver};
use crate::registry::{ClozeRegistry, MeaningRegistry, QueueRegistry, TagRegistry, WordRegistry};

/// Search and filter state manager.
///
/// This struct manages the search query and provides a unified interface
/// for executing queries using the QueryEngine.
#[derive(Debug)]
pub struct SearchManager {
    /// Current search query string
    pub query: String,

    /// Current sort type
    pub sort: SortType,

    /// Whether the query has changed and needs re-execution
    dirty: bool,

    /// Cached search results
    cached_results: Option<Vec<(WordId, i32)>>,
}

impl SearchManager {
    /// Creates a new SearchManager.
    pub fn new() -> Self {
        Self {
            query: String::new(),
            sort: SortType::default(),
            dirty: true,
            cached_results: None,
        }
    }

    /// Sets the search query.
    ///
    /// Marks the manager as dirty so the query will be re-executed
    /// on the next call to `execute()`.
    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.dirty = true;
        self.cached_results = None;
    }

    /// Clears the search query.
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.dirty = true;
        self.cached_results = None;
    }

    /// Sets the sort type.
    pub fn set_sort(&mut self, sort: SortType) {
        self.sort = sort;
        self.dirty = true;
        // Don't clear cached results, just mark as dirty
    }

    /// Clears all filters.
    pub fn clear_filters(&mut self) {
        self.query.clear();
        self.sort = SortType::default();
        self.dirty = true;
        self.cached_results = None;
    }

    /// Checks if any filters are active.
    pub fn has_active_filters(&self) -> bool {
        !self.query.is_empty()
    }

    /// Executes the query and returns the results.
    ///
    /// This method uses lazy evaluation - the query is only executed
    /// when the query string or sort type has changed.
    pub fn execute(
        &mut self,
        word_registry: &WordRegistry,
        meaning_registry: &MeaningRegistry,
        cloze_registry: &ClozeRegistry,
        queue_registry: &QueueRegistry,
        tag_registry: &TagRegistry,
    ) -> &[(WordId, i32)] {
        if self.dirty || self.cached_results.is_none() {
            let mut resolver = TagResolver::new(tag_registry);
            let query = if self.query.trim().is_empty() {
                // Empty query matches everything
                Query::empty()
            } else {
                crate::query::parse::parse_query(&self.query, &mut resolver)
            };

            let engine = QueryEngine::new(
                word_registry,
                meaning_registry,
                cloze_registry,
                queue_registry,
            );

            self.cached_results = Some(engine.execute(&query));
            self.dirty = false;
        }

        self.cached_results
            .as_ref()
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Gets the cached results without executing.
    ///
    /// Returns `None` if the query hasn't been executed yet.
    pub fn get_results(&self) -> Option<&[(WordId, i32)]> {
        self.cached_results.as_ref().map(|v| v.as_slice())
    }

    /// Returns the IDs of matching words (without scores).
    pub fn matching_ids(&self) -> Vec<WordId> {
        self.cached_results
            .as_ref()
            .map(|results| results.iter().map(|(id, _)| *id).collect())
            .unwrap_or_default()
    }

    /// Returns true if the query has results.
    pub fn has_results(&self) -> bool {
        self.cached_results
            .as_ref()
            .map(|r| !r.is_empty())
            .unwrap_or(false)
    }
}

impl Default for SearchManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("hello".to_string(), "hello", true, true; "set query")]
    #[test_case("".to_string(), "", false, true; "clear query")]
    fn test_search_manager_query_state(
        initial_query: String,
        expected_query: &str,
        expected_has_filters: bool,
        expected_dirty: bool,
    ) {
        let mut manager = SearchManager::new();
        manager.set_query(initial_query);
        assert_eq!(manager.query, expected_query);
        assert_eq!(manager.has_active_filters(), expected_has_filters);
        assert_eq!(manager.dirty, expected_dirty);
    }

    #[test_case(SortType::AZ; "set sort")]
    fn test_search_manager_sort(sort: SortType) {
        let mut manager = SearchManager::new();
        manager.set_sort(sort);
        assert_eq!(manager.sort, sort);
    }

    #[test_case("hello".to_string(), SortType::AZ; "clear filters after set")]
    fn test_search_manager_clear_filters(query: String, sort: SortType) {
        let mut manager = SearchManager::new();
        manager.set_query(query);
        manager.set_sort(sort);
        manager.clear_filters();
        assert!(manager.query.is_empty());
        assert_eq!(manager.sort, SortType::BestMatch);
        assert!(!manager.has_active_filters());
    }
}
