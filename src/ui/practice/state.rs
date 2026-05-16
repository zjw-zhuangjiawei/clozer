use std::collections::HashMap;

use crate::models::cloze::{Cloze, ClozeSegment};
use crate::models::types::{ClozeId, TagId};
use crate::state::Model;

#[derive(Debug, Default)]
pub struct PracticeState {
    pub tag_filter: Option<TagId>,
    pub show_tag_picker: bool,
    pub tag_search: String,

    pub session_clozes: Vec<ClozeId>,
    pub total_blanks: usize,
    pub current_index: usize,
    pub is_active: bool,

    pub answers: HashMap<usize, String>,
    pub submitted: bool,
    pub results: HashMap<usize, bool>,

    pub correct_count: usize,
    pub total_attempted: usize,
}

impl PracticeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_session(&mut self, model: &Model) {
        self.session_clozes.clear();
        self.total_blanks = 0;

        if let Some(tag_id) = self.tag_filter {
            for (meaning_id, _) in model.meaning_registry.iter_by_tag(tag_id) {
                for (cloze_id, cloze) in model.cloze_registry.iter_by_meaning_id(*meaning_id) {
                    self.total_blanks += cloze
                        .segments
                        .iter()
                        .filter(|s| matches!(s, ClozeSegment::Blank(_)))
                        .count();
                    self.session_clozes.push(*cloze_id);
                }
            }
        } else {
            for (cloze_id, cloze) in model.cloze_registry.iter() {
                self.total_blanks += cloze
                    .segments
                    .iter()
                    .filter(|s| matches!(s, ClozeSegment::Blank(_)))
                    .count();
                self.session_clozes.push(*cloze_id);
            }
        }

        self.current_index = 0;
        self.answers.clear();
        self.results.clear();
        self.submitted = false;
        self.correct_count = 0;
        self.total_attempted = 0;
    }

    pub fn current_cloze<'a>(&self, model: &'a Model) -> Option<&'a Cloze> {
        self.session_clozes
            .get(self.current_index)
            .and_then(|id| model.cloze_registry.get(*id))
    }

    pub fn blank_segments<'a>(&self, model: &'a Model) -> Vec<(usize, &'a str)> {
        let Some(cloze) = self.current_cloze(model) else {
            return Vec::new();
        };
        let mut idx = 0;
        let mut blanks = Vec::new();
        for seg in &cloze.segments {
            if let ClozeSegment::Blank(answer) = seg {
                blanks.push((idx, answer.as_str()));
                idx += 1;
            }
        }
        blanks
    }

    pub fn has_blanks(&self, model: &Model) -> bool {
        self.current_cloze(model)
            .map(|c| {
                c.segments
                    .iter()
                    .any(|s| matches!(s, ClozeSegment::Blank(_)))
            })
            .unwrap_or(false)
    }

    pub fn is_session_complete(&self) -> bool {
        self.is_active && self.current_index >= self.session_clozes.len()
    }

    pub fn render_sentence_with_numbers(&self, model: &Model) -> String {
        let Some(cloze) = self.current_cloze(model) else {
            return String::new();
        };
        let mut result = String::new();
        let mut blank_idx = 0;
        for seg in &cloze.segments {
            match seg {
                ClozeSegment::Text(t) => result.push_str(t),
                ClozeSegment::Blank(a) => {
                    if self.submitted {
                        result.push_str(&format!("[{}]", a));
                    } else {
                        result.push_str(&format!("({}) ___", blank_idx + 1));
                    }
                    blank_idx += 1;
                }
            }
        }
        result
    }

    pub fn reset_current_cloze(&mut self) {
        self.answers.clear();
        self.results.clear();
        self.submitted = false;
    }

    pub fn score_percent(&self) -> f64 {
        if self.total_blanks == 0 {
            return 0.0;
        }
        (self.correct_count as f64 / self.total_blanks as f64) * 100.0
    }
}
