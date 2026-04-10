use crate::models::{PartOfSpeech, TagId};
use crate::query::ast::{Condition, Query, SortType, StatusFilter, Token};
use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;

/// Parses a query string into a Query with resolved tag names
pub fn parse_query(input: &str, resolver: &mut TagResolver) -> Query {
    if input.trim().is_empty() {
        return Query::empty();
    }

    let tokens = tokenize(input);
    let condition = build_ast(tokens);

    // Resolve tag names to TagIds
    let filter = condition
        .resolve_tags(resolver)
        .unwrap_or_else(|| Condition::All(vec![]));

    Query::new(filter, SortType::BestMatch)
}

/// Parses a query string into a Query without resolving tags (for testing)
pub fn parse_query_raw(input: &str) -> (Vec<Token>, Condition) {
    let tokens = tokenize(input);
    let condition = build_ast(tokens.clone());
    (tokens, condition)
}

/// First phase: Tokenize input string
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut current_token = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            '|' => {
                // Flush current token before OR
                if !current_token.is_empty() {
                    if let Some(token) = parse_single_token(&current_token) {
                        tokens.push(token);
                    }
                    current_token.clear();
                }
                tokens.push(Token::Or);
            }
            '(' => {
                if !current_token.is_empty() {
                    if let Some(token) = parse_single_token(&current_token) {
                        tokens.push(token);
                    }
                    current_token.clear();
                }
                tokens.push(Token::LeftParen);
            }
            ')' => {
                if !current_token.is_empty() {
                    if let Some(token) = parse_single_token(&current_token) {
                        tokens.push(token);
                    }
                    current_token.clear();
                }
                tokens.push(Token::RightParen);
            }
            ' ' | '\t' | '\n' => {
                // Whitespace separates tokens
                if !current_token.is_empty() {
                    if let Some(token) = parse_single_token(&current_token) {
                        tokens.push(token);
                    }
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    // Don't forget the last token
    if !current_token.is_empty() {
        if let Some(token) = parse_single_token(&current_token) {
            tokens.push(token);
        }
    }

    tokens
}

/// Parses a single token string into a Token
fn parse_single_token(s: &str) -> Option<Token> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    if s.starts_with('-') {
        // Exclusion tokens
        if let Some(tag_name) = s.strip_prefix("-#") {
            if !tag_name.is_empty() {
                return Some(Token::ExcludeTagName(tag_name.to_string()));
            }
        } else if let Some(pos_str) = s.strip_prefix("-:") {
            let pos_list = parse_pos_list(pos_str);
            if !pos_list.is_empty() {
                return Some(Token::ExcludePos(pos_list));
            }
        } else if let Some(status_str) = s.strip_prefix("-is:") {
            if let Some(status) = StatusFilter::parse(status_str) {
                return Some(Token::ExcludeStatus(status));
            }
        }
        // Text starting with - but not a special token
        return Some(Token::Text(s.to_string()));
    } else if s.starts_with('#') {
        let tag_name = s.strip_prefix('#').unwrap_or(s);
        if !tag_name.is_empty() {
            return Some(Token::IncludeTagName(tag_name.to_string()));
        }
    } else if let Some(pos_str) = s.strip_prefix(':') {
        let pos_list = parse_pos_list(pos_str);
        if !pos_list.is_empty() {
            return Some(Token::IncludePos(pos_list));
        }
    } else if let Some(status_str) = s.strip_prefix("is:") {
        if let Some(status) = StatusFilter::parse(status_str) {
            return Some(Token::IncludeStatus(status));
        }
    }

    // Default: text search
    Some(Token::Text(s.to_string()))
}

/// Second phase: Build AST from tokens using recursive descent parser
///
/// Grammar:
///   expression := and_expr ("|" and_expr)*
///   and_expr   := primary+
///   primary    := tag | pos | status | text | "(" expression ")"
pub fn build_ast(tokens: Vec<Token>) -> Condition {
    if tokens.is_empty() {
        return Condition::All(vec![]);
    }

    let mut iter = tokens.iter().peekable();
    parse_expression(&mut iter)
}

/// Parse expression (handles OR operator)
/// expression := and_expr ("|" and_expr)*
fn parse_expression(tokens: &mut Peekable<Iter<Token>>) -> Condition {
    let left = parse_and(tokens);

    // Check if there are more OR conditions
    let mut or_conditions = vec![left];

    while let Some(token) = tokens.peek() {
        if **token == Token::Or {
            tokens.next(); // consume |
            let right = parse_and(tokens);
            or_conditions.push(right);
        } else {
            break;
        }
    }

    if or_conditions.len() == 1 {
        or_conditions.into_iter().next().unwrap()
    } else {
        Condition::Any(or_conditions)
    }
}

/// Parse AND expression (handles implicit AND via space separation)
/// and_expr := primary+
fn parse_and(tokens: &mut Peekable<Iter<Token>>) -> Condition {
    let mut conditions = vec![];

    while let Some(token) = tokens.peek() {
        match token {
            // Stop at OR or right paren
            Token::Or | Token::RightParen => break,
            _ => {
                let cond = parse_primary(tokens);
                conditions.push(cond);
            }
        }
    }

    if conditions.is_empty() {
        Condition::All(vec![])
    } else if conditions.len() == 1 {
        conditions.into_iter().next().unwrap()
    } else {
        Condition::All(conditions)
    }
}

/// Parse primary expression
/// primary := tag | pos | status | text | "(" expression ")"
fn parse_primary(tokens: &mut Peekable<Iter<Token>>) -> Condition {
    match tokens.next() {
        Some(Token::LeftParen) => {
            let cond = parse_expression(tokens);
            // Expect right paren
            if let Some(Token::RightParen) = tokens.peek() {
                tokens.next(); // consume )
            }
            cond
        }
        Some(Token::IncludeTagName(name)) => Condition::HasTagName(name.clone()),
        Some(Token::ExcludeTagName(name)) => Condition::NotHasTagName(name.clone()),
        Some(Token::IncludeTag(id)) => Condition::HasTag(*id),
        Some(Token::ExcludeTag(id)) => Condition::NotHasTag(*id),
        Some(Token::IncludePos(pos_list)) => {
            if pos_list.len() == 1 {
                Condition::HasPos(pos_list[0])
            } else {
                // Multiple POS: treat as OR
                Condition::Any(pos_list.iter().map(|p| Condition::HasPos(*p)).collect())
            }
        }
        Some(Token::ExcludePos(pos_list)) => {
            if pos_list.len() == 1 {
                Condition::NotHasPos(pos_list[0])
            } else {
                // Multiple POS exclusion: treat as AND of exclusions
                Condition::All(pos_list.iter().map(|p| Condition::NotHasPos(*p)).collect())
            }
        }
        Some(Token::IncludeStatus(status)) => Condition::HasStatus(*status),
        Some(Token::ExcludeStatus(status)) => Condition::NotHasStatus(*status),
        Some(Token::Text(text)) => Condition::Text(text.clone()),
        _ => Condition::All(vec![]), // Empty condition matches all
    }
}

fn parse_pos_list(s: &str) -> Vec<PartOfSpeech> {
    s.split(',').filter_map(|p| parse_pos(p.trim())).collect()
}

pub fn parse_pos(s: &str) -> Option<PartOfSpeech> {
    match s.to_lowercase().as_str() {
        "n" | "noun" => Some(PartOfSpeech::Noun),
        "v" | "verb" => Some(PartOfSpeech::Verb),
        "adj" | "adjective" => Some(PartOfSpeech::Adjective),
        "adv" | "adverb" => Some(PartOfSpeech::Adverb),
        "pron" | "pronoun" => Some(PartOfSpeech::Pronoun),
        "prep" | "preposition" => Some(PartOfSpeech::Preposition),
        "conj" | "conjunction" => Some(PartOfSpeech::Conjunction),
        "interj" | "interjection" => Some(PartOfSpeech::Interjection),
        "det" | "determiner" => Some(PartOfSpeech::Determiner),
        "art" | "article" => Some(PartOfSpeech::Article),
        "modal" => Some(PartOfSpeech::Modal),
        "num" | "numeral" => Some(PartOfSpeech::Numeral),
        "abbr" | "abbreviation" => Some(PartOfSpeech::Abbreviation),
        _ => None,
    }
}

pub struct TagResolver<'a> {
    tag_registry: &'a crate::registry::TagRegistry,
    cache: HashMap<String, Option<TagId>>,
}

impl<'a> TagResolver<'a> {
    pub fn new(tag_registry: &'a crate::registry::TagRegistry) -> Self {
        Self {
            tag_registry,
            cache: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, name: &str) -> Option<TagId> {
        let name_lower = name.to_lowercase();

        // Check cache first
        if let Some(cached) = self.cache.get(&name_lower) {
            return *cached;
        }

        // Look up in registry
        let found = self
            .tag_registry
            .iter()
            .find(|(_, t)| t.name.to_lowercase() == name_lower)
            .map(|(id, _)| *id);

        // Cache the result
        self.cache.insert(name_lower, found);
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("", 0; "empty")]
    #[test_case("hello world", 2; "text only")]
    #[test_case("#important", 1; "hash tag")]
    #[test_case("-#ignored", 1; "exclude tag")]
    #[test_case(":noun,verb", 1; "POS filter")]
    #[test_case("hello | world", 3; "OR operator")]
    #[test_case("(hello world)", 4; "grouping")]
    fn test_tokenize(input: &str, expected_count: usize) {
        let tokens = tokenize(input);
        assert_eq!(tokens.len(), expected_count);
    }

    #[test_case("hello world", "hello"; "first text token")]
    #[test_case("#important", "important"; "tag name")]
    #[test_case("-#ignored", "ignored"; "exclude tag name")]
    #[test_case("hello | world", "hello"; "text before OR")]
    #[test_case("(hello world)", "hello"; "text after left paren")]
    fn test_tokenize_first_text(input: &str, expected: &str) {
        let tokens = tokenize(input);
        assert!(!tokens.is_empty());
        match &tokens[0] {
            Token::Text(s) => assert_eq!(s, expected),
            Token::IncludeTagName(name) => assert_eq!(name, expected),
            Token::ExcludeTagName(name) => assert_eq!(name, expected),
            _ => {}
        }
    }

    #[test_case("hello", "hello"; "simple text")]
    fn test_parse_simple(input: &str, expected_text: &str) {
        let (_, cond) = parse_query_raw(input);
        assert!(matches!(cond, Condition::Text(s) if s == expected_text));
    }

    #[test_case("hello world", 2; "implicit AND")]
    #[test_case("hello :noun | world", 2; "OR with AND")]
    #[test_case("(#tag1 | #tag2) :noun", 2; "grouped OR")]
    #[test_case("hello -world | foo (#tag1 | #tag2)", 2; "complex query")]
    fn test_parse_condition_structure(input: &str, expected_branch_count: usize) {
        let (_, cond) = parse_query_raw(input);
        match cond {
            Condition::Any(conds) => assert_eq!(conds.len(), expected_branch_count),
            Condition::All(conds) => assert_eq!(conds.len(), expected_branch_count),
            Condition::Text(_) => assert_eq!(1, expected_branch_count),
            _ => {}
        }
    }

    #[test_case("is:pending", StatusFilter::Pending; "pending status")]
    #[test_case("-is:done", StatusFilter::Done; "exclude done status")]
    fn test_parse_status(input: &str, expected_status: StatusFilter) {
        let (_, cond) = parse_query_raw(input);
        match cond {
            Condition::HasStatus(status) => assert_eq!(status, expected_status),
            Condition::NotHasStatus(status) => assert_eq!(status, expected_status),
            _ => panic!("expected status condition"),
        }
    }
}
