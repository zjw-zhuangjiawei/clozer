use crate::models::types::WordId;
use crate::query::ast::{Condition, Query, SortType, StatusFilter};
use crate::registry::{ClozeRegistry, MeaningRegistry, QueueRegistry, WordRegistry};
use std::collections::HashSet;

/// Query engine for executing search queries
pub struct QueryEngine<'a> {
    word_registry: &'a WordRegistry,
    meaning_registry: &'a MeaningRegistry,
    cloze_registry: &'a ClozeRegistry,
    queue_registry: &'a QueueRegistry,
}

impl<'a> QueryEngine<'a> {
    pub fn new(
        word_registry: &'a WordRegistry,
        meaning_registry: &'a MeaningRegistry,
        cloze_registry: &'a ClozeRegistry,
        queue_registry: &'a QueueRegistry,
    ) -> Self {
        Self {
            word_registry,
            meaning_registry,
            cloze_registry,
            queue_registry,
        }
    }

    /// Execute a query and return matching word IDs with scores
    pub fn execute(&self, query: &Query) -> Vec<(WordId, i32)> {
        // Apply filter conditions to get candidates
        let candidates = self.apply_condition(&query.filter);

        // Calculate scores for text matches
        let scored = self.calculate_scores(candidates, &query.filter);

        // Sort results
        self.sort_results(scored, query.sort)
    }

    /// Apply a condition to filter words
    fn apply_condition(&self, condition: &Condition) -> HashSet<WordId> {
        match condition {
            Condition::Text(query) => self.search_text(query),
            Condition::HasTag(tag_id) => self
                .meaning_registry
                .iter_by_tag(*tag_id)
                .map(|(_, m)| m.word_id)
                .collect(),
            Condition::NotHasTag(tag_id) => {
                let all_words = self.all_word_ids();
                let tagged_words: HashSet<_> = self
                    .meaning_registry
                    .iter_by_tag(*tag_id)
                    .map(|(_, m)| m.word_id)
                    .collect();
                all_words.difference(&tagged_words).copied().collect()
            }
            Condition::HasPos(pos) => self
                .meaning_registry
                .iter()
                .filter(|(_, m)| m.pos == *pos)
                .map(|(_, m)| m.word_id)
                .collect(),
            Condition::NotHasPos(pos) => {
                let all_words = self.all_word_ids();
                let pos_words: HashSet<_> = self
                    .meaning_registry
                    .iter()
                    .filter(|(_, m)| m.pos == *pos)
                    .map(|(_, m)| m.word_id)
                    .collect();
                all_words.difference(&pos_words).copied().collect()
            }
            Condition::HasStatus(status) => self.filter_by_status(*status),
            Condition::NotHasStatus(status) => {
                let all_words = self.all_word_ids();
                let status_words = self.filter_by_status(*status);
                all_words.difference(&status_words).copied().collect()
            }
            Condition::All(conditions) => {
                // Intersection (AND)
                if conditions.is_empty() {
                    return self.all_word_ids();
                }

                let mut result = self.all_word_ids();
                for cond in conditions {
                    result = result
                        .intersection(&self.apply_condition(cond))
                        .copied()
                        .collect();
                    if result.is_empty() {
                        break;
                    }
                }
                result
            }
            Condition::Any(conditions) => {
                // Union (OR)
                if conditions.is_empty() {
                    return HashSet::new();
                }

                conditions
                    .iter()
                    .flat_map(|c| self.apply_condition(c))
                    .collect()
            }
            // Temporary conditions should be resolved before execution
            Condition::HasTagName(_) | Condition::NotHasTagName(_) => {
                tracing::warn!("Unresolved tag name in condition");
                HashSet::new()
            }
        }
    }

    /// Get all word IDs
    fn all_word_ids(&self) -> HashSet<WordId> {
        self.word_registry.iter().map(|(id, _)| *id).collect()
    }

    /// Search for words matching text query
    fn search_text(&self, query: &str) -> HashSet<WordId> {
        let query_lower = query.to_lowercase();
        let mut results = HashSet::new();

        for (word_id, word) in self.word_registry.iter() {
            // Check word content
            if word.content.to_lowercase().contains(&query_lower) {
                results.insert(*word_id);
                continue;
            }

            // Check definitions
            for (_, meaning) in self.meaning_registry.iter_by_word(*word_id) {
                if meaning.definition.to_lowercase().contains(&query_lower) {
                    results.insert(*word_id);
                    break;
                }
            }
        }

        results
    }

    /// Filter words by status
    fn filter_by_status(&self, status: StatusFilter) -> HashSet<WordId> {
        let mut result = HashSet::new();

        match status {
            StatusFilter::Pending => {
                for (_, word) in self.word_registry.iter() {
                    let all_meanings_have_queue_entry = !word.meaning_ids.is_empty()
                        && word
                            .meaning_ids
                            .iter()
                            .all(|mid| self.queue_registry.contains(*mid));
                    if all_meanings_have_queue_entry {
                        result.insert(word.id);
                    }
                }
            }
            StatusFilter::Done => {
                for (_, word) in self.word_registry.iter() {
                    let any_meaning_has_completed = word
                        .meaning_ids
                        .iter()
                        .any(|mid| !self.queue_registry.contains(*mid));
                    if any_meaning_has_completed {
                        result.insert(word.id);
                    }
                }
            }
            StatusFilter::Cloze => {
                for (_, word) in self.word_registry.iter() {
                    let has_cloze = word
                        .meaning_ids
                        .iter()
                        .any(|mid| self.cloze_registry.count_by_meaning(*mid) > 0);
                    if has_cloze {
                        result.insert(word.id);
                    }
                }
            }
            StatusFilter::Plain => {
                for (_, word) in self.word_registry.iter() {
                    let has_cloze = word
                        .meaning_ids
                        .iter()
                        .any(|mid| self.cloze_registry.count_by_meaning(*mid) > 0);
                    if !has_cloze {
                        result.insert(word.id);
                    }
                }
            }
        }

        result
    }

    /// Calculate scores for text matches
    fn calculate_scores(
        &self,
        candidates: HashSet<WordId>,
        condition: &Condition,
    ) -> Vec<(WordId, i32)> {
        let text_queries = condition.text_queries();

        if text_queries.is_empty() {
            return candidates.into_iter().map(|id| (id, 0)).collect();
        }

        candidates
            .into_iter()
            .map(|word_id| {
                let score = self.calculate_word_score(word_id, &text_queries);
                (word_id, score)
            })
            .collect()
    }

    /// Calculate score for a single word
    fn calculate_word_score(&self, word_id: WordId, queries: &[&str]) -> i32 {
        let word = match self.word_registry.get(word_id) {
            Some(w) => w,
            None => return 0,
        };

        let word_lower = word.content.to_lowercase();
        let mut max_score = 0;

        for query in queries {
            let query_lower = query.to_lowercase();

            // Exact match
            if word_lower == query_lower {
                max_score = max_score.max(100);
            }
            // Starts with
            else if word_lower.starts_with(&query_lower) {
                max_score = max_score.max(50);
            }
            // Contains
            else if word_lower.contains(&query_lower) {
                max_score = max_score.max(20);
            }

            // Check definition matches
            for (_, meaning) in self.meaning_registry.iter_by_word(word_id) {
                if meaning.definition.to_lowercase().contains(&query_lower) {
                    max_score = max_score.max(10);
                }
            }
        }

        max_score
    }

    /// Sort results by the specified sort type
    fn sort_results(&self, mut results: Vec<(WordId, i32)>, sort: SortType) -> Vec<(WordId, i32)> {
        match sort {
            SortType::BestMatch => {
                results.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            }
            SortType::Newest => {
                results.sort_by(|a, b| b.0.cmp(&a.0));
            }
            SortType::Oldest => {
                results.sort_by(|a, b| a.0.cmp(&b.0));
            }
            SortType::AZ => {
                results.sort_by(|a, b| {
                    let word_a = self
                        .word_registry
                        .get(a.0)
                        .map(|w| w.content.to_lowercase())
                        .unwrap_or_default();
                    let word_b = self
                        .word_registry
                        .get(b.0)
                        .map(|w| w.content.to_lowercase())
                        .unwrap_or_default();
                    word_a.cmp(&word_b)
                });
            }
            SortType::Length => {
                results.sort_by(|a, b| {
                    let len_a = self
                        .word_registry
                        .get(a.0)
                        .map(|w| w.content.len())
                        .unwrap_or(0);
                    let len_b = self
                        .word_registry
                        .get(b.0)
                        .map(|w| w.content.len())
                        .unwrap_or(0);
                    len_a.cmp(&len_b)
                });
            }
        }

        results
    }
}

/// Legacy search function for backward compatibility
///
/// This function is deprecated. Use QueryEngine instead.
pub fn search(
    word_registry: &WordRegistry,
    meaning_registry: &MeaningRegistry,
    cloze_registry: &ClozeRegistry,
    queue_registry: &QueueRegistry,
    _ast: &crate::query::ast::QueryAST,
    sort: SortType,
) -> Vec<(WordId, i32)> {
    let engine = QueryEngine::new(
        word_registry,
        meaning_registry,
        cloze_registry,
        queue_registry,
    );

    let query = Query::empty();
    let mut results = engine.execute(&query);

    // Apply sort
    match sort {
        SortType::BestMatch => {
            results.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        }
        SortType::Newest => {
            results.sort_by(|a, b| b.0.cmp(&a.0));
        }
        SortType::Oldest => {
            results.sort_by(|a, b| a.0.cmp(&b.0));
        }
        SortType::AZ => {
            results.sort_by(|a, b| {
                let word_a = word_registry
                    .get(a.0)
                    .map(|w| w.content.to_lowercase())
                    .unwrap_or_default();
                let word_b = word_registry
                    .get(b.0)
                    .map(|w| w.content.to_lowercase())
                    .unwrap_or_default();
                word_a.cmp(&word_b)
            });
        }
        SortType::Length => {
            results.sort_by(|a, b| {
                let len_a = word_registry.get(a.0).map(|w| w.content.len()).unwrap_or(0);
                let len_b = word_registry.get(b.0).map(|w| w.content.len()).unwrap_or(0);
                len_a.cmp(&len_b)
            });
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Meaning, Tag, Word};

    fn setup_test_data() -> (WordRegistry, MeaningRegistry, ClozeRegistry, QueueRegistry) {
        let mut word_registry = WordRegistry::new();
        let mut meaning_registry = MeaningRegistry::new();
        let cloze_registry = ClozeRegistry::new();
        let queue_registry = QueueRegistry::new();

        // Add test words
        let word1 = Word::builder().content("hello".to_string()).build();
        let word2 = Word::builder().content("world".to_string()).build();
        let word3 = Word::builder().content("foo".to_string()).build();

        word_registry.add(word1.clone());
        word_registry.add(word2.clone());
        word_registry.add(word3.clone());

        // Add meanings
        let meaning1 = Meaning::builder()
            .word_id(word1.id)
            .definition("a greeting".to_string())
            .pos(crate::models::PartOfSpeech::Noun)
            .build();
        let meaning2 = Meaning::builder()
            .word_id(word2.id)
            .definition("the earth".to_string())
            .pos(crate::models::PartOfSpeech::Noun)
            .build();

        meaning_registry.add(meaning1);
        meaning_registry.add(meaning2);

        (
            word_registry,
            meaning_registry,
            cloze_registry,
            queue_registry,
        )
    }

    #[test]
    fn test_engine_empty_query() {
        let (word_registry, meaning_registry, cloze_registry, queue_registry) = setup_test_data();
        let engine = QueryEngine::new(
            &word_registry,
            &meaning_registry,
            &cloze_registry,
            &queue_registry,
        );

        let query = Query::empty();
        let results = engine.execute(&query);

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_engine_text_search() {
        let (word_registry, meaning_registry, cloze_registry, queue_registry) = setup_test_data();
        let engine = QueryEngine::new(
            &word_registry,
            &meaning_registry,
            &cloze_registry,
            &queue_registry,
        );

        let query = Query::new(Condition::Text("hello".to_string()), SortType::BestMatch);
        let results = engine.execute(&query);

        assert_eq!(results.len(), 1);
        assert!(results.iter().any(|(id, _)| {
            word_registry
                .get(*id)
                .map(|w| w.content == "hello")
                .unwrap_or(false)
        }));
    }

    #[test]
    fn test_engine_and_condition() {
        let (word_registry, meaning_registry, cloze_registry, queue_registry) = setup_test_data();
        let engine = QueryEngine::new(
            &word_registry,
            &meaning_registry,
            &cloze_registry,
            &queue_registry,
        );

        let query = Query::new(
            Condition::All(vec![
                Condition::Text("hello".to_string()),
                Condition::Text("world".to_string()),
            ]),
            SortType::BestMatch,
        );
        let results = engine.execute(&query);

        // No word matches both "hello" and "world"
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_engine_or_condition() {
        let (word_registry, meaning_registry, cloze_registry, queue_registry) = setup_test_data();
        let engine = QueryEngine::new(
            &word_registry,
            &meaning_registry,
            &cloze_registry,
            &queue_registry,
        );

        let query = Query::new(
            Condition::Any(vec![
                Condition::Text("hello".to_string()),
                Condition::Text("world".to_string()),
            ]),
            SortType::BestMatch,
        );
        let results = engine.execute(&query);

        assert_eq!(results.len(), 2);
    }
}
