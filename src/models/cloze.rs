use fancy_regex::Regex;
use once_cell::sync::Lazy;
use std::fmt;
use typed_builder::TypedBuilder;
use uuid::Uuid;

static BLANK_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[([^\]]+)\]").unwrap());

/// A segment of a cloze sentence - either plain text or a blank with answer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClozeSegment {
    Text(String),
    Blank(String),
}

impl fmt::Display for ClozeSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClozeSegment::Text(s) => write!(f, "{}", s),
            ClozeSegment::Blank(a) => write!(f, "[{}]", a),
        }
    }
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Cloze {
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub meaning_id: Uuid,
    pub segments: Vec<ClozeSegment>,
}

impl Cloze {
    /// Parse a sentence with `[answer]` markers into segments
    pub fn parse_from_sentence(sentence: &str) -> Vec<ClozeSegment> {
        let mut segments = Vec::new();
        let mut last_end = 0;

        for result in BLANK_RE.find_iter(sentence) {
            let cap =
                result.expect("fancy-regex: failed to find [answer] pattern in cloze sentence");

            // Text before the blank
            if cap.start() > last_end {
                let text = &sentence[last_end..cap.start()];
                segments.push(ClozeSegment::Text(text.to_string()));
            }
            // Extract answer from capture group (without brackets)
            let answer = &sentence[cap.start() + 1..cap.end() - 1];
            segments.push(ClozeSegment::Blank(answer.to_string()));
            last_end = cap.end();
        }

        // Remaining text
        if last_end < sentence.len() {
            segments.push(ClozeSegment::Text(sentence[last_end..].to_string()));
        }

        segments
    }

    /// Render sentence with blanks visible as `___`
    pub fn render_blanks(&self) -> String {
        self.segments
            .iter()
            .map(|s| match s {
                ClozeSegment::Text(t) => t.clone(),
                ClozeSegment::Blank(_) => "___".to_string(),
            })
            .collect()
    }

    /// Render sentence with answers filled in
    pub fn render_answers(&self) -> String {
        self.segments
            .iter()
            .map(|s| match s {
                ClozeSegment::Text(t) => t.clone(),
                ClozeSegment::Blank(a) => a.clone(),
            })
            .collect()
    }
}
