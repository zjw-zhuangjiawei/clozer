use crate::models::{PartOfSpeech, TagId};
use strum::Display;

#[derive(Debug, Clone)]
pub enum Token {
    IncludeTag(TagId),
    ExcludeTag(TagId),
    IncludePos(Vec<PartOfSpeech>),
    ExcludePos(Vec<PartOfSpeech>),
    IncludeStatus(StatusFilter),
    ExcludeStatus(StatusFilter),
    Text(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusFilter {
    Pending,
    Done,
    Cloze,
    Plain,
}

impl StatusFilter {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Some(Self::Pending),
            "done" => Some(Self::Done),
            "cloze" => Some(Self::Cloze),
            "plain" => Some(Self::Plain),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryAST {
    pub tokens: Vec<Token>,
}

impl QueryAST {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn text_tokens(&self) -> Vec<&str> {
        self.tokens
            .iter()
            .filter_map(|t| {
                if let Token::Text(s) = t {
                    Some(s.as_str())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

impl Default for QueryAST {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SortType {
    #[default]
    BestMatch,
    Newest,
    Oldest,
    AZ,
    Length,
}

impl SortType {
    pub fn variants() -> Vec<SortType> {
        vec![
            SortType::BestMatch,
            SortType::Newest,
            SortType::Oldest,
            SortType::AZ,
            SortType::Length,
        ]
    }
}
