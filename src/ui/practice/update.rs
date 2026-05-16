use crate::models::cloze::ClozeSegment;
use crate::state::Model;

use super::message::{NotificationLevel, PracticeMessage};
use super::state::PracticeState;

pub fn update(
    state: &mut PracticeState,
    message: PracticeMessage,
    model: &mut Model,
) -> iced::Task<PracticeMessage> {
    match message {
        PracticeMessage::ToggleTagPicker => {
            state.show_tag_picker = !state.show_tag_picker;
            state.tag_search.clear();
        }
        PracticeMessage::TagSearchChanged(query) => {
            state.tag_search = query;
        }
        PracticeMessage::TagFilterSelected(tag_id) => {
            state.tag_filter = Some(tag_id);
            state.show_tag_picker = false;
            state.tag_search.clear();
        }
        PracticeMessage::TagFilterCleared => {
            state.tag_filter = None;
            state.show_tag_picker = false;
            state.tag_search.clear();
        }
        PracticeMessage::StartSession => {
            state.build_session(model);
            if !state.session_clozes.is_empty() {
                state.is_active = true;
                state.current_index = 0;
                state.reset_current_cloze();
            } else {
                return iced::Task::done(PracticeMessage::Notify {
                    level: NotificationLevel::Warning,
                    message: "No clozes found for the selected filter.".to_string(),
                });
            }
        }

        PracticeMessage::EndSession => {
            state.is_active = false;
            state.session_clozes.clear();
            state.total_blanks = 0;
            state.current_index = 0;
            state.reset_current_cloze();
            state.correct_count = 0;
            state.total_attempted = 0;
        }

        PracticeMessage::NextCloze => {
            if state.current_index + 1 < state.session_clozes.len() {
                state.current_index += 1;
                state.reset_current_cloze();
            }
        }
        PracticeMessage::PreviousCloze => {
            if state.current_index > 0 {
                state.current_index -= 1;
                state.reset_current_cloze();
            }
        }

        PracticeMessage::AnswerChanged { blank_index, value } => {
            state.answers.insert(blank_index, value);
        }

        PracticeMessage::SubmitAnswers => {
            if state.submitted {
                return iced::Task::none();
            }
            state.submitted = true;
            state.total_attempted += 1;
            state.results.clear();

            if let Some(cloze) = state.current_cloze(model) {
                let blanks: Vec<&str> = cloze
                    .segments
                    .iter()
                    .filter_map(|s| match s {
                        ClozeSegment::Blank(answer) => Some(answer.as_str()),
                        ClozeSegment::Text(_) => None,
                    })
                    .collect();

                for (blank_idx, correct_answer) in blanks.iter().enumerate() {
                    let user_answer = state
                        .answers
                        .get(&blank_idx)
                        .map(|s| s.trim())
                        .unwrap_or("");
                    let is_correct = user_answer.to_lowercase() == correct_answer.to_lowercase();
                    state.results.insert(blank_idx, is_correct);
                    if is_correct {
                        state.correct_count += 1;
                    }
                }
            }
        }
        PracticeMessage::SkipCloze => {
            if state.current_index + 1 < state.session_clozes.len() {
                state.current_index += 1;
                state.reset_current_cloze();
            }
        }

        PracticeMessage::Notify { .. } => {}
    }
    iced::Task::none()
}
