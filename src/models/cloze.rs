use fancy_regex::Regex;
use once_cell::sync::Lazy;
use std::fmt;
use typed_builder::TypedBuilder;

use super::{ClozeId, MeaningId};

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
    #[builder(default = ClozeId::new())]
    pub id: ClozeId,
    pub meaning_id: MeaningId,
    pub segments: Vec<ClozeSegment>,
}

impl Cloze {
    /// Parse a sentence with `[answer]` markers into segments
    pub fn parse_from_sentence(sentence: &str) -> Vec<ClozeSegment> {
        let mut segments = Vec::new();
        let mut last_end = 0;

        for result in BLANK_RE.find_iter(sentence) {
            let Ok(cap) = result else {
                tracing::error!("fancy-regex: failed to find [answer] pattern in cloze sentence");
                continue;
            };

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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(ClozeSegment::Text("hello".to_string()), "hello"; "text segment")]
    #[test_case(ClozeSegment::Blank("answer".to_string()), "[answer]"; "blank segment")]
    fn test_cloze_segment_display(segment: ClozeSegment, expected: &str) {
        assert_eq!(segment.to_string(), expected);
    }

    #[test_case(ClozeSegment::Text("hello".to_string()), ClozeSegment::Text("hello".to_string()), true; "text equals text")]
    #[test_case(ClozeSegment::Blank("answer".to_string()), ClozeSegment::Blank("answer".to_string()), true; "blank equals blank")]
    #[test_case(ClozeSegment::Text("hello".to_string()), ClozeSegment::Text("world".to_string()), false; "text not equals different text")]
    #[test_case(ClozeSegment::Text("hello".to_string()), ClozeSegment::Blank("hello".to_string()), false; "text not equals blank")]
    fn test_cloze_segment_equality(a: ClozeSegment, b: ClozeSegment, should_equal: bool) {
        assert_eq!(a == b, should_equal);
    }

    fn cloze_with_segments(segments: Vec<ClozeSegment>) -> Cloze {
        Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(segments)
            .build()
    }

    #[test_case("The [cat] sat", 3; "single blank in middle")]
    #[test_case("[a] [b] [c]", 5; "multiple blanks with spaces")]
    #[test_case("[hello] world", 2; "blank at start")]
    #[test_case("hello [world]", 2; "blank at end")]
    #[test_case("plain text without blanks", 1; "no blanks")]
    #[test_case("", 0; "empty string")]
    #[test_case("start [a][b] end", 4; "consecutive blanks")]
    #[test_case("[only]", 1; "only blank")]
    #[test_case("Hello [world!]", 2; "blank with special chars")]
    #[test_case("2 + 2 = [4]", 2; "blank with numbers")]
    #[test_case("The [quick] [brown] [fox]", 6; "multiple separate blanks")]
    fn test_parse_from_sentence(input: &str, expected_len: usize) {
        let segments = Cloze::parse_from_sentence(input);
        assert_eq!(segments.len(), expected_len);
    }

    #[test_case("The [cat] sat", 1, "cat"; "blank in middle")]
    #[test_case("[a] [b] [c]", 0, "a"; "first blank")]
    #[test_case("hello [world]", 1, "world"; "blank at end")]
    fn test_parse_from_sentence_first_blank(input: &str, idx: usize, expected_answer: &str) {
        let segments = Cloze::parse_from_sentence(input);
        assert!(idx < segments.len());
        match &segments[idx] {
            ClozeSegment::Blank(answer) => assert_eq!(answer, expected_answer),
            _ => panic!("Expected Blank at index {}", idx),
        }
    }

    #[test_case(vec![ClozeSegment::Text("Hello ".to_string()), ClozeSegment::Blank("world".to_string())], "Hello ___"; "single blank")]
    #[test_case(vec![ClozeSegment::Text("The ".to_string()), ClozeSegment::Blank("cat".to_string()), ClozeSegment::Text(" sat on the ".to_string()), ClozeSegment::Blank("mat".to_string())], "The ___ sat on the ___"; "multiple blanks")]
    #[test_case(vec![ClozeSegment::Text("plain text".to_string())], "plain text"; "no blanks")]
    #[test_case(vec![], ""; "empty segments")]
    fn test_render_blanks(segments: Vec<ClozeSegment>, expected: &str) {
        assert_eq!(cloze_with_segments(segments).render_blanks(), expected);
    }

    #[test_case(vec![ClozeSegment::Text("Hello ".to_string()), ClozeSegment::Blank("world".to_string())], "Hello world"; "single blank")]
    #[test_case(vec![ClozeSegment::Text("The ".to_string()), ClozeSegment::Blank("cat".to_string()), ClozeSegment::Text(" sat on the ".to_string()), ClozeSegment::Blank("mat".to_string())], "The cat sat on the mat"; "multiple blanks")]
    #[test_case(vec![ClozeSegment::Text("plain text".to_string())], "plain text"; "no blanks")]
    #[test_case(vec![], ""; "empty segments")]
    fn test_render_answers(segments: Vec<ClozeSegment>, expected: &str) {
        assert_eq!(cloze_with_segments(segments).render_answers(), expected);
    }

    #[test_case("The [cat] sat on the [mat]", "The ___ sat on the ___", "The cat sat on the mat"; "multiple blanks roundtrip")]
    #[test_case("", "", ""; "empty roundtrip")]
    fn test_parse_render_roundtrip(input: &str, expected_blanks: &str, expected_answers: &str) {
        let segments = Cloze::parse_from_sentence(input);
        let cloze = cloze_with_segments(segments);
        assert_eq!(cloze.render_blanks(), expected_blanks);
        assert_eq!(cloze.render_answers(), expected_answers);
    }
}
