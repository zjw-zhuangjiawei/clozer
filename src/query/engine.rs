use crate::models::types::WordId;
use crate::query::ast::{QueryAST, SortType, StatusFilter, Token};
use crate::registry::{ClozeRegistry, MeaningRegistry, QueueRegistry, WordRegistry};
use std::collections::HashSet;

pub fn search(
    word_registry: &WordRegistry,
    meaning_registry: &MeaningRegistry,
    cloze_registry: &ClozeRegistry,
    queue_registry: &QueueRegistry,
    ast: &QueryAST,
    sort: SortType,
) -> Vec<(WordId, i32)> {
    if ast.is_empty() {
        let mut results: Vec<_> = word_registry.iter().map(|(id, _)| (*id, 0)).collect();
        sort_results(&mut results, sort);
        return results;
    }

    let text_query = ast.text_tokens();
    let text_query_lower: Vec<String> = text_query.iter().map(|s| s.to_lowercase()).collect();

    let mut candidates: HashSet<WordId> = word_registry.iter().map(|(id, _)| *id).collect();

    for token in &ast.tokens {
        match token {
            Token::IncludeTag(tag_id) => {
                let matching: HashSet<WordId> = meaning_registry
                    .iter_by_tag(*tag_id)
                    .map(|(_, m)| m.word_id)
                    .collect();
                candidates = candidates.intersection(&matching).copied().collect();
            }
            Token::ExcludeTag(tag_id) => {
                let excluded: HashSet<WordId> = meaning_registry
                    .iter_by_tag(*tag_id)
                    .map(|(_, m)| m.word_id)
                    .collect();
                candidates = candidates.difference(&excluded).copied().collect();
            }
            Token::IncludePos(pos_list) => {
                let matching = find_words_by_pos(meaning_registry, pos_list);
                candidates = candidates.intersection(&matching).copied().collect();
            }
            Token::ExcludePos(pos_list) => {
                let excluded = find_words_by_pos(meaning_registry, pos_list);
                candidates = candidates.difference(&excluded).copied().collect();
            }
            Token::IncludeStatus(status) => {
                let matching = find_words_by_status(
                    word_registry,
                    meaning_registry,
                    cloze_registry,
                    queue_registry,
                    *status,
                );
                candidates = candidates.intersection(&matching).copied().collect();
            }
            Token::ExcludeStatus(status) => {
                let excluded = find_words_by_status(
                    word_registry,
                    meaning_registry,
                    cloze_registry,
                    queue_registry,
                    *status,
                );
                candidates = candidates.difference(&excluded).copied().collect();
            }
            Token::Text(_) => {}
        }
    }

    let mut scored: Vec<(WordId, i32)> = candidates
        .iter()
        .map(|word_id| {
            let score = if text_query_lower.is_empty() {
                0
            } else {
                calculate_score(word_registry, meaning_registry, *word_id, &text_query_lower)
            };
            (*word_id, score)
        })
        .collect();

    sort_results(&mut scored, sort);
    scored
}

fn find_words_by_pos(
    meaning_registry: &MeaningRegistry,
    pos_list: &[crate::models::PartOfSpeech],
) -> HashSet<WordId> {
    let mut result = HashSet::new();
    for (_, meaning) in meaning_registry.iter() {
        if pos_list.contains(&meaning.pos) {
            result.insert(meaning.word_id);
        }
    }
    result
}

fn find_words_by_status(
    word_registry: &WordRegistry,
    _meaning_registry: &MeaningRegistry,
    cloze_registry: &ClozeRegistry,
    queue_registry: &QueueRegistry,
    status: StatusFilter,
) -> HashSet<WordId> {
    let mut result = HashSet::new();
    match status {
        StatusFilter::Pending => {
            for (_, word) in word_registry.iter() {
                let all_meanings_have_queue_entry = !word.meaning_ids.is_empty()
                    && word
                        .meaning_ids
                        .iter()
                        .all(|mid| queue_registry.contains(*mid));
                if all_meanings_have_queue_entry {
                    result.insert(word.id);
                }
            }
        }
        StatusFilter::Done => {
            for (_, word) in word_registry.iter() {
                let any_meaning_has_completed = word
                    .meaning_ids
                    .iter()
                    .any(|mid| !queue_registry.contains(*mid));
                if any_meaning_has_completed {
                    result.insert(word.id);
                }
            }
        }
        StatusFilter::Cloze => {
            for (_, word) in word_registry.iter() {
                let has_cloze = word
                    .meaning_ids
                    .iter()
                    .any(|mid| cloze_registry.count_by_meaning(*mid) > 0);
                if has_cloze {
                    result.insert(word.id);
                }
            }
        }
        StatusFilter::Plain => {
            for (_, word) in word_registry.iter() {
                let has_cloze = word
                    .meaning_ids
                    .iter()
                    .any(|mid| cloze_registry.count_by_meaning(*mid) > 0);
                if !has_cloze {
                    result.insert(word.id);
                }
            }
        }
    }
    result
}

fn calculate_score(
    word_registry: &WordRegistry,
    meaning_registry: &MeaningRegistry,
    word_id: WordId,
    text_query: &[String],
) -> i32 {
    let word = match word_registry.get(word_id) {
        Some(w) => w,
        None => return 0,
    };
    let word_lower = word.content.to_lowercase();

    let mut max_score = 0;

    for query in text_query {
        let query_lower = query.to_lowercase();

        if word_lower == query_lower {
            max_score = max_score.max(100);
        } else if word_lower.starts_with(&query_lower) {
            max_score = max_score.max(50);
        } else if word_lower.contains(&query_lower) {
            max_score = max_score.max(20);
        }

        for (_, meaning) in meaning_registry.iter_by_word(word_id) {
            if meaning.definition.to_lowercase().contains(&query_lower) {
                max_score = max_score.max(10);
            }
        }
    }

    max_score
}

fn sort_results(results: &mut [(WordId, i32)], sort: SortType) {
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
                let word_a = b.0;
                let word_b = a.0;
                word_a.cmp(&word_b)
            });
        }
        SortType::Length => {
            results.sort_by_key(|(id, _)| id.to_string().len());
        }
    }
}
