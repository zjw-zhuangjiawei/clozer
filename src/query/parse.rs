use crate::models::{PartOfSpeech, TagId};
use crate::query::ast::{QueryAST, StatusFilter, Token};
use std::collections::HashMap;

pub fn parse_query(input: &str) -> QueryAST {
    let input = input.trim();
    if input.is_empty() {
        return QueryAST::default();
    }

    let mut tokens = Vec::new();
    let mut text_parts = Vec::new();

    for token_raw in input.split_whitespace() {
        if let Some(token) = parse_token(token_raw) {
            if !text_parts.is_empty() {
                let text = text_parts.join(" ");
                tokens.push(Token::Text(text));
                text_parts.clear();
            }
            tokens.push(token);
        } else {
            text_parts.push(token_raw);
        }
    }

    if !text_parts.is_empty() {
        let text = text_parts.join(" ");
        tokens.push(Token::Text(text));
    }

    QueryAST { tokens }
}

fn parse_token(s: &str) -> Option<Token> {
    if s.starts_with('-') {
        if let Some(tag_name) = s.strip_prefix("-#") {
            if !tag_name.is_empty() {
                return Some(Token::ExcludeTag(TagId::default()));
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
    } else if s.starts_with('#') {
        let tag_name = s.strip_prefix('#').unwrap_or(s);
        if !tag_name.is_empty() {
            return Some(Token::IncludeTag(TagId::default()));
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
    None
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
    cache: HashMap<String, TagId>,
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
        if let Some(&cached) = self.cache.get(&name_lower) {
            return Some(cached);
        }
        let found = self
            .tag_registry
            .iter()
            .find(|(_, t)| t.name.to_lowercase() == name_lower)
            .map(|(id, _)| *id);
        if let Some(id) = found {
            self.cache.insert(name_lower, id);
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let ast = parse_query("");
        assert!(ast.is_empty());
    }

    #[test]
    fn test_parse_text_only() {
        let ast = parse_query("hello world");
        assert_eq!(ast.tokens.len(), 1);
        match &ast.tokens[0] {
            Token::Text(s) => assert_eq!(s, "hello world"),
            _ => panic!("expected Text token"),
        }
    }

    #[test]
    fn test_parse_pos() {
        let ast = parse_query(":n,v");
        assert_eq!(ast.tokens.len(), 1);
        match &ast.tokens[0] {
            Token::IncludePos(pos_list) => {
                assert_eq!(pos_list.len(), 2);
            }
            _ => panic!("expected IncludePos token"),
        }
    }

    #[test]
    fn test_parse_exclude_pos() {
        let ast = parse_query("-:adj");
        assert_eq!(ast.tokens.len(), 1);
        match &ast.tokens[0] {
            Token::ExcludePos(pos_list) => {
                assert_eq!(pos_list.len(), 1);
            }
            _ => panic!("expected ExcludePos token"),
        }
    }

    #[test]
    fn test_parse_status() {
        let ast = parse_query("is:pending");
        assert_eq!(ast.tokens.len(), 1);
        match &ast.tokens[0] {
            Token::IncludeStatus(s) => assert_eq!(*s, StatusFilter::Pending),
            _ => panic!("expected IncludeStatus token"),
        }
    }

    #[test]
    fn test_parse_mixed() {
        let ast = parse_query("hello :n -#mytag is:done");
        assert_eq!(ast.tokens.len(), 4);
    }

    #[test]
    fn test_parse_pos_verbose() {
        let ast = parse_query(":noun,verb");
        match &ast.tokens[0] {
            Token::IncludePos(list) => assert_eq!(list.len(), 2),
            _ => panic!("expected IncludePos"),
        }
    }
}
