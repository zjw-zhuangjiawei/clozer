use crate::models::{PartOfSpeech, TagId};
use crate::query::parse::TagResolver;
use strum::Display;

/// Token for parsing phase (before tag resolution)
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Tag name to be resolved (include)
    IncludeTagName(String),
    /// Tag name to be resolved (exclude)
    ExcludeTagName(String),
    /// Resolved tag ID (internal use)
    IncludeTag(TagId),
    /// Resolved tag ID (internal use)
    ExcludeTag(TagId),
    /// Part of speech list (include)
    IncludePos(Vec<PartOfSpeech>),
    /// Part of speech list (exclude)
    ExcludePos(Vec<PartOfSpeech>),
    /// Status filter (include)
    IncludeStatus(StatusFilter),
    /// Status filter (exclude)
    ExcludeStatus(StatusFilter),
    /// Text search
    Text(String),
    /// OR operator
    Or,
    /// Left parenthesis
    LeftParen,
    /// Right parenthesis
    RightParen,
}

/// Unified condition expression for filtering
#[derive(Debug, Clone)]
pub enum Condition {
    /// Text search (matches word content or definition)
    Text(String),
    /// Has specific tag
    HasTag(TagId),
    /// Does not have specific tag
    NotHasTag(TagId),
    /// Temporary: tag name to be resolved
    HasTagName(String),
    /// Temporary: tag name to be resolved (negated)
    NotHasTagName(String),
    /// Has specific part of speech
    HasPos(PartOfSpeech),
    /// Does not have specific part of speech
    NotHasPos(PartOfSpeech),
    /// Has specific status
    HasStatus(StatusFilter),
    /// Does not have specific status
    NotHasStatus(StatusFilter),
    /// All conditions must match (AND)
    All(Vec<Condition>),
    /// Any condition must match (OR)
    Any(Vec<Condition>),
}

impl Condition {
    /// Resolves all TagName conditions to TagId conditions
    pub fn resolve_tags(self, resolver: &mut TagResolver) -> Option<Condition> {
        match self {
            Condition::HasTagName(name) => resolver.resolve(&name).map(Condition::HasTag),
            Condition::NotHasTagName(name) => resolver.resolve(&name).map(Condition::NotHasTag),
            Condition::All(conditions) => {
                let resolved: Vec<_> = conditions
                    .into_iter()
                    .filter_map(|c| c.resolve_tags(resolver))
                    .collect();
                if resolved.is_empty() {
                    None
                } else {
                    Some(Condition::All(resolved))
                }
            }
            Condition::Any(conditions) => {
                let resolved: Vec<_> = conditions
                    .into_iter()
                    .filter_map(|c| c.resolve_tags(resolver))
                    .collect();
                if resolved.is_empty() {
                    None
                } else {
                    Some(Condition::Any(resolved))
                }
            }
            other => Some(other),
        }
    }

    /// Returns true if this condition has any text search components
    pub fn has_text_search(&self) -> bool {
        match self {
            Condition::Text(_) => true,
            Condition::All(conds) => conds.iter().any(|c| c.has_text_search()),
            Condition::Any(conds) => conds.iter().any(|c| c.has_text_search()),
            _ => false,
        }
    }

    /// Extracts all text queries from this condition
    pub fn text_queries(&self) -> Vec<&str> {
        let mut queries = Vec::new();
        self.collect_text_queries(&mut queries);
        queries
    }

    fn collect_text_queries<'a>(&'a self, queries: &mut Vec<&'a str>) {
        match self {
            Condition::Text(s) => queries.push(s.as_str()),
            Condition::All(conds) | Condition::Any(conds) => {
                for cond in conds {
                    cond.collect_text_queries(queries);
                }
            }
            _ => {}
        }
    }
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

/// Query structure
#[derive(Debug, Clone)]
pub struct Query {
    pub filter: Condition,
    pub sort: SortType,
}

impl Query {
    pub fn new(filter: Condition, sort: SortType) -> Self {
        Self { filter, sort }
    }

    /// Creates an empty query that matches everything
    pub fn empty() -> Self {
        Self {
            filter: Condition::All(vec![]),
            sort: SortType::default(),
        }
    }

    /// Resolves all tag names in the filter
    pub fn resolve_tags(mut self, resolver: &mut TagResolver) -> Option<Self> {
        self.filter = self.filter.resolve_tags(resolver)?;
        Some(self)
    }
}

/// Legacy AST structure (for backward compatibility during migration)
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
