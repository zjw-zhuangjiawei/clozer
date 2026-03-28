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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloze_segment_display_text() {
        let segment = ClozeSegment::Text("hello".to_string());
        assert_eq!(segment.to_string(), "hello");
    }

    #[test]
    fn test_cloze_segment_display_blank() {
        let segment = ClozeSegment::Blank("answer".to_string());
        assert_eq!(segment.to_string(), "[answer]");
    }

    #[test]
    fn test_cloze_segment_eq() {
        assert_eq!(
            ClozeSegment::Text("hello".to_string()),
            ClozeSegment::Text("hello".to_string())
        );
        assert_eq!(
            ClozeSegment::Blank("answer".to_string()),
            ClozeSegment::Blank("answer".to_string())
        );
    }

    #[test]
    fn test_cloze_segment_ne() {
        assert_ne!(
            ClozeSegment::Text("hello".to_string()),
            ClozeSegment::Text("world".to_string())
        );
        assert_ne!(
            ClozeSegment::Text("hello".to_string()),
            ClozeSegment::Blank("hello".to_string())
        );
    }

    #[test]
    fn test_parse_single_blank() {
        let segments = Cloze::parse_from_sentence("The [cat] sat");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0], ClozeSegment::Text("The ".to_string()));
        assert_eq!(segments[1], ClozeSegment::Blank("cat".to_string()));
        assert_eq!(segments[2], ClozeSegment::Text(" sat".to_string()));
    }

    #[test]
    fn test_parse_multiple_blanks() {
        let segments = Cloze::parse_from_sentence("[a] [b] [c]");
        assert_eq!(segments.len(), 5);
        assert_eq!(segments[0], ClozeSegment::Blank("a".to_string()));
        assert_eq!(segments[1], ClozeSegment::Text(" ".to_string()));
        assert_eq!(segments[2], ClozeSegment::Blank("b".to_string()));
        assert_eq!(segments[3], ClozeSegment::Text(" ".to_string()));
        assert_eq!(segments[4], ClozeSegment::Blank("c".to_string()));
    }

    #[test]
    fn test_parse_blank_at_start() {
        let segments = Cloze::parse_from_sentence("[hello] world");
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], ClozeSegment::Blank("hello".to_string()));
        assert_eq!(segments[1], ClozeSegment::Text(" world".to_string()));
    }

    #[test]
    fn test_parse_blank_at_end() {
        let segments = Cloze::parse_from_sentence("hello [world]");
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], ClozeSegment::Text("hello ".to_string()));
        assert_eq!(segments[1], ClozeSegment::Blank("world".to_string()));
    }

    #[test]
    fn test_parse_no_blank() {
        let segments = Cloze::parse_from_sentence("plain text without blanks");
        assert_eq!(segments.len(), 1);
        assert_eq!(
            segments[0],
            ClozeSegment::Text("plain text without blanks".to_string())
        );
    }

    #[test]
    fn test_parse_empty_string() {
        let segments = Cloze::parse_from_sentence("");
        assert!(segments.is_empty());
    }

    #[test]
    fn test_parse_consecutive_blanks() {
        let segments = Cloze::parse_from_sentence("start [a][b] end");
        assert_eq!(segments.len(), 4);
        assert_eq!(segments[0], ClozeSegment::Text("start ".to_string()));
        assert_eq!(segments[1], ClozeSegment::Blank("a".to_string()));
        assert_eq!(segments[2], ClozeSegment::Blank("b".to_string()));
        assert_eq!(segments[3], ClozeSegment::Text(" end".to_string()));
    }

    #[test]
    fn test_parse_only_blank() {
        let segments = Cloze::parse_from_sentence("[only]");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0], ClozeSegment::Blank("only".to_string()));
    }

    #[test]
    fn test_parse_blank_with_special_chars() {
        let segments = Cloze::parse_from_sentence("Hello [world!]");
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], ClozeSegment::Text("Hello ".to_string()));
        assert_eq!(segments[1], ClozeSegment::Blank("world!".to_string()));
    }

    #[test]
    fn test_parse_blank_with_numbers() {
        let segments = Cloze::parse_from_sentence("2 + 2 = [4]");
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0], ClozeSegment::Text("2 + 2 = ".to_string()));
        assert_eq!(segments[1], ClozeSegment::Blank("4".to_string()));
    }

    #[test]
    fn test_parse_multiple_blanks_together() {
        let segments = Cloze::parse_from_sentence("The [quick] [brown] [fox]");
        assert_eq!(segments.len(), 6);
        assert_eq!(segments[0], ClozeSegment::Text("The ".to_string()));
        assert_eq!(segments[1], ClozeSegment::Blank("quick".to_string()));
        assert_eq!(segments[2], ClozeSegment::Text(" ".to_string()));
        assert_eq!(segments[3], ClozeSegment::Blank("brown".to_string()));
        assert_eq!(segments[4], ClozeSegment::Text(" ".to_string()));
        assert_eq!(segments[5], ClozeSegment::Blank("fox".to_string()));
    }

    #[test]
    fn test_render_blanks_single_blank() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![
                ClozeSegment::Text("Hello ".to_string()),
                ClozeSegment::Blank("world".to_string()),
            ])
            .build();
        assert_eq!(cloze.render_blanks(), "Hello ___");
    }

    #[test]
    fn test_render_blanks_multiple_blanks() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![
                ClozeSegment::Text("The ".to_string()),
                ClozeSegment::Blank("cat".to_string()),
                ClozeSegment::Text(" sat on the ".to_string()),
                ClozeSegment::Blank("mat".to_string()),
            ])
            .build();
        assert_eq!(cloze.render_blanks(), "The ___ sat on the ___");
    }

    #[test]
    fn test_render_blanks_no_blanks() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![ClozeSegment::Text("plain text".to_string())])
            .build();
        assert_eq!(cloze.render_blanks(), "plain text");
    }

    #[test]
    fn test_render_blanks_empty_segments() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![])
            .build();
        assert_eq!(cloze.render_blanks(), "");
    }

    #[test]
    fn test_render_answers_single_blank() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![
                ClozeSegment::Text("Hello ".to_string()),
                ClozeSegment::Blank("world".to_string()),
            ])
            .build();
        assert_eq!(cloze.render_answers(), "Hello world");
    }

    #[test]
    fn test_render_answers_multiple_blanks() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![
                ClozeSegment::Text("The ".to_string()),
                ClozeSegment::Blank("cat".to_string()),
                ClozeSegment::Text(" sat on the ".to_string()),
                ClozeSegment::Blank("mat".to_string()),
            ])
            .build();
        assert_eq!(cloze.render_answers(), "The cat sat on the mat");
    }

    #[test]
    fn test_render_answers_no_blanks() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![ClozeSegment::Text("plain text".to_string())])
            .build();
        assert_eq!(cloze.render_answers(), "plain text");
    }

    #[test]
    fn test_render_answers_empty_segments() {
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(vec![])
            .build();
        assert_eq!(cloze.render_answers(), "");
    }

    #[test]
    fn test_parse_render_roundtrip() {
        let original = "The [cat] sat on the [mat]";
        let segments = Cloze::parse_from_sentence(original);
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(segments)
            .build();
        assert_eq!(cloze.render_blanks(), "The ___ sat on the ___");
        assert_eq!(cloze.render_answers(), "The cat sat on the mat");
    }

    #[test]
    fn test_parse_render_roundtrip_empty() {
        let original = "";
        let segments = Cloze::parse_from_sentence(original);
        let cloze = Cloze::builder()
            .meaning_id(MeaningId::new())
            .segments(segments)
            .build();
        assert_eq!(cloze.render_blanks(), "");
        assert_eq!(cloze.render_answers(), "");
    }
}
