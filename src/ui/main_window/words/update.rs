//! Words panel update handler.

use super::message::WordsMessage;
use super::state::TagDropdownState;
use crate::models::{Meaning, Word};
use crate::state::Model;
use crate::ui::main_window::state::MainWindowState;
use iced::Task;

/// Handles all words-related messages.
pub fn update(
    state: &mut MainWindowState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        // Word CRUD
        WordsMessage::InputChanged(value) => {
            state.words_ui.word_input = value;
        }
        WordsMessage::CreateWord(content) => {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                let word = Word::builder().content(trimmed.to_string()).build();
                tracing::debug!("Creating word: {} (id={})", word.content, word.id);
                model.word_registry.add(word);
            }
        }
        WordsMessage::DeleteWord(word_id) => {
            tracing::debug!("Deleting word: {}", word_id);
            // Delete all clozes for meanings of this word
            for (meaning_id, _) in model.meaning_registry.iter_by_word(word_id) {
                model.cloze_registry.delete_by_meaning(*meaning_id);
            }
            // Delete all meanings
            model.meaning_registry.delete_by_word(word_id);
            // Delete word
            model.word_registry.delete(word_id);
            state.selected_word_ids.remove(&word_id);
        }
        WordsMessage::DeleteSelected => {
            for &meaning_id in &state.selected_meaning_ids {
                model.cloze_registry.delete_by_meaning(meaning_id);
                model.meaning_registry.delete(meaning_id);
            }
            state.selected_meaning_ids.clear();
        }
        WordsMessage::ToggleWordExpand(word_id) => {
            if state.expanded_word_ids.contains(&word_id) {
                state.expanded_word_ids.remove(&word_id);
            } else {
                state.expanded_word_ids.insert(word_id);
            }
        }

        // Selection
        WordsMessage::ToggleWord(word_id) => {
            if state.selected_word_ids.contains(&word_id) {
                state.selected_word_ids.remove(&word_id);
                for (mid, _) in model.meaning_registry.iter_by_word(word_id) {
                    state.selected_meaning_ids.remove(mid);
                }
            } else {
                state.selected_word_ids.insert(word_id);
                for (mid, _) in model.meaning_registry.iter_by_word(word_id) {
                    state.selected_meaning_ids.insert(*mid);
                }
            }
        }
        WordsMessage::ToggleMeaning(meaning_id) => {
            if state.selected_meaning_ids.contains(&meaning_id) {
                state.selected_meaning_ids.remove(&meaning_id);
            } else {
                state.selected_meaning_ids.insert(meaning_id);
            }

            if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                let word_id = meaning.word_id;
                let all_meanings_selected: bool = model
                    .meaning_registry
                    .iter_by_word(word_id)
                    .all(|(mid, _)| state.selected_meaning_ids.contains(mid));

                if all_meanings_selected {
                    state.selected_word_ids.insert(word_id);
                } else {
                    state.selected_word_ids.remove(&word_id);
                }
            }
        }
        WordsMessage::SelectAllWords => {
            for (id, _) in model.word_registry.iter() {
                state.selected_word_ids.insert(*id);
                for (mid, _) in model.meaning_registry.iter_by_word(*id) {
                    state.selected_meaning_ids.insert(*mid);
                }
            }
        }
        WordsMessage::DeselectAllWords => {
            state.selected_word_ids.clear();
            state.selected_meaning_ids.clear();
        }

        // Filtering
        WordsMessage::TagFilterChanged(value) => {
            state.words_ui.tag_filter = value;
        }
        WordsMessage::ClearTagFilter => {
            state.words_ui.tag_filter.clear();
        }

        // Meaning input
        WordsMessage::ToggleMeaningInput(word_id) => {
            let input = state.words_ui.meaning_inputs.entry(word_id).or_default();
            input.visible = !input.visible;
            if !input.visible {
                state.words_ui.meaning_inputs.remove(&word_id);
            }
        }
        WordsMessage::MeaningDefInputChanged(word_id, value) => {
            state
                .words_ui
                .meaning_inputs
                .entry(word_id)
                .or_default()
                .definition = value;
        }
        WordsMessage::MeaningPosSelected(word_id, pos) => {
            state
                .words_ui
                .meaning_inputs
                .entry(word_id)
                .or_default()
                .pos = pos;
        }
        WordsMessage::SaveMeaning(word_id) => {
            if let Some(input) = state.words_ui.meaning_inputs.get(&word_id) {
                let meaning = Meaning::builder()
                    .word_id(word_id)
                    .definition(input.definition.clone())
                    .pos(input.pos)
                    .build();

                model.meaning_registry.add(meaning.clone());
                model.word_registry.add_meaning(word_id, meaning.id);
                state.words_ui.meaning_inputs.remove(&word_id);
            }
        }
        WordsMessage::CancelMeaningInput(word_id) => {
            state.words_ui.meaning_inputs.remove(&word_id);
        }
        WordsMessage::DeleteMeaning(meaning_id) => {
            let word_id = if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                meaning.word_id
            } else {
                return Task::none();
            };

            model.cloze_registry.delete_by_meaning(meaning_id);
            model.word_registry.remove_meaning(word_id, meaning_id);
            model.meaning_registry.delete(meaning_id);
        }

        // Per-meaning tag operations
        WordsMessage::MeaningToggleTagDropdown(meaning_id) => {
            state.words_ui.active_tag_dropdown =
                if state.words_ui.active_tag_dropdown == Some(meaning_id) {
                    None
                } else {
                    Some(meaning_id)
                };
        }
        WordsMessage::MeaningTagSearchChanged(_value) => {
            // No-op
        }
        WordsMessage::AddTagToMeaningSearch(meaning_id, tag_name) => {
            let trimmed = tag_name.trim();
            if !trimmed.is_empty() {
                let existing = model
                    .tag_registry
                    .iter()
                    .find(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase());
                if let Some((_, tag)) = existing {
                    model.meaning_registry.add_tag(meaning_id, tag.id);
                    state.words_ui.active_tag_dropdown = None;
                }
            }
        }
        WordsMessage::AddTagToMeaning(meaning_id, tag_id) => {
            model.meaning_registry.add_tag(meaning_id, tag_id);
            state.words_ui.active_tag_dropdown = None;
        }
        WordsMessage::RemoveTagFromMeaning(meaning_id, tag_id) => {
            model.meaning_registry.remove_tag(meaning_id, tag_id);
        }

        // Batch tag operations
        WordsMessage::ToggleBatchAddTagDropdown => {
            state.words_ui.meanings_tag_dropdown_state =
                if state.words_ui.meanings_tag_dropdown_state == TagDropdownState::Add {
                    TagDropdownState::None
                } else {
                    TagDropdownState::Add
                };
            if state.words_ui.meanings_tag_dropdown_state == TagDropdownState::Add {
                state.words_ui.meanings_tag_search_input.clear();
            }
        }
        WordsMessage::ToggleBatchRemoveTagDropdown => {
            state.words_ui.meanings_tag_dropdown_state =
                if state.words_ui.meanings_tag_dropdown_state == TagDropdownState::Remove {
                    TagDropdownState::None
                } else {
                    TagDropdownState::Remove
                };
            if state.words_ui.meanings_tag_dropdown_state == TagDropdownState::Remove {
                state.words_ui.meanings_tag_remove_search_input.clear();
            }
        }
        WordsMessage::BatchTagSearchChanged(value) => {
            state.words_ui.meanings_tag_search_input = value;
        }
        WordsMessage::BatchTagRemoveSearchChanged(value) => {
            state.words_ui.meanings_tag_remove_search_input = value;
        }
        WordsMessage::BatchAddTagToSelectedMeanings(tag_id) => {
            for &meaning_id in &state.selected_meaning_ids {
                model.meaning_registry.add_tag(meaning_id, tag_id);
            }
            state.words_ui.meanings_tag_dropdown_state = TagDropdownState::None;
            state.words_ui.meanings_tag_search_input.clear();
        }
        WordsMessage::BatchRemoveTagFromSelectedMeanings(tag_id) => {
            for &meaning_id in &state.selected_meaning_ids {
                model.meaning_registry.remove_tag(meaning_id, tag_id);
            }
            state.words_ui.meanings_tag_dropdown_state = TagDropdownState::None;
            state.words_ui.meanings_tag_remove_search_input.clear();
        }

        // Tag CRUD
        WordsMessage::CreateTag(name) => {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                let existing = model
                    .tag_registry
                    .iter()
                    .find(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase());
                if existing.is_none() {
                    let tag = crate::models::Tag::builder()
                        .name(trimmed.to_string())
                        .build();
                    model.tag_registry.add(tag);
                }
            }
        }
        WordsMessage::DeleteTag(tag_id) => {
            model.tag_registry.delete(tag_id);
        }

        // Queue trigger
        WordsMessage::QueueSelected => {
            for &meaning_id in &state.selected_meaning_ids {
                model.queue_registry.enqueue(meaning_id);
            }
            state.selected_meaning_ids.clear();
        }
    }
    Task::none()
}
