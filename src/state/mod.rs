//! Application state management.
//!
//! Contains Model (data + business logic) and AppState (orchestrator).

pub mod generator;
pub mod model;
pub mod queue;

pub use self::generator::{Generator, GeneratorState};
pub use self::model::Model;
pub use self::queue::{QueueGenerationResult, QueueState};

use crate::message::Message;
use crate::models::{Meaning, Word};
use crate::window::{TagDropdownState, WindowState};
use iced::Task;

/// AppState holding Model (data + business logic only).
#[derive(Debug)]
pub struct AppState {
    pub model: Model,
}

impl AppState {
    /// Creates a new AppState with the given database.
    pub fn new(db: crate::persistence::Db) -> Self {
        Self {
            model: Model::new(db),
        }
    }

    /// Unified Update Handler
    /// Takes window state as parameter for UI operations
    pub fn update(&mut self, message: Message, window: &mut WindowState) -> Task<Message> {
        match message {
            // Word operations
            Message::CreateWord(content) => {
                let trimmed = content.trim();
                if !trimmed.is_empty() {
                    let word = Word::builder().content(trimmed.to_string()).build();
                    tracing::debug!("Creating word: {} (id={})", word.content, word.id);
                    self.model.word_registry.add(word);
                }
            }
            Message::DeleteWord(word_id) => {
                tracing::debug!("Deleting word: {}", word_id);
                // Delete all clozes for meanings of this word
                for (meaning_id, _) in self.model.meaning_registry.iter_by_word(word_id) {
                    self.model.cloze_registry.delete_by_meaning(*meaning_id);
                }
                // Delete all meanings
                self.model.meaning_registry.delete_by_word(word_id);
                // Delete word
                self.model.word_registry.delete(word_id);
                window.selected_word_ids.remove(&word_id);
            }
            Message::DeleteSelected => {
                for &meaning_id in &window.selected_meaning_ids {
                    // Delete clozes for meanings
                    self.model.cloze_registry.delete_by_meaning(meaning_id);
                    // Delete meanings
                    self.model.meaning_registry.delete(meaning_id);
                }
                window.selected_meaning_ids.clear();
            }
            Message::ToggleWordExpand(word_id) => {
                if window.expanded_word_ids.contains(&word_id) {
                    window.expanded_word_ids.remove(&word_id);
                } else {
                    window.expanded_word_ids.insert(word_id);
                }
            }

            // Meaning operations
            Message::CreateMeaning(word_id, definition, pos) => {
                let trimmed_def = definition.trim();
                if !trimmed_def.is_empty() {
                    let meaning = Meaning::builder()
                        .word_id(word_id)
                        .definition(trimmed_def.to_string())
                        .pos(pos)
                        .build();

                    tracing::debug!(
                        "Creating meaning for word {}: {} ({})",
                        word_id,
                        meaning.definition,
                        meaning.pos
                    );
                    self.model.meaning_registry.add(meaning.clone());

                    // Update Word.meaning_ids
                    self.model.word_registry.add_meaning(word_id, meaning.id);
                }
            }
            Message::SaveMeaning(word_id) => {
                if let Some(input) = window.words_ui.meaning_inputs.get(&word_id) {
                    // Create new Meaning from input
                    let meaning = Meaning::builder()
                        .word_id(word_id)
                        .definition(input.definition.clone())
                        .pos(input.pos)
                        .build();

                    self.model.meaning_registry.add(meaning.clone());

                    // Update Word.meaning_ids
                    self.model.word_registry.add_meaning(word_id, meaning.id);

                    // Hide input
                    window.words_ui.meaning_inputs.remove(&word_id);
                }
            }
            Message::CancelMeaningInput(word_id) => {
                window.words_ui.meaning_inputs.remove(&word_id);
            }
            Message::ToggleMeaningInput(word_id) => {
                let input = window.words_ui.meaning_inputs.entry(word_id).or_default();
                input.visible = !input.visible;
                if !input.visible {
                    window.words_ui.meaning_inputs.remove(&word_id);
                }
            }
            Message::MeaningDefInputChanged(word_id, value) => {
                window
                    .words_ui
                    .meaning_inputs
                    .entry(word_id)
                    .or_default()
                    .definition = value;
            }
            Message::MeaningPosSelected(word_id, pos) => {
                window
                    .words_ui
                    .meaning_inputs
                    .entry(word_id)
                    .or_default()
                    .pos = pos;
            }
            Message::DeleteMeaning(meaning_id) => {
                // Get word_id for cleanup
                let word_id = if let Some(meaning) = self.model.meaning_registry.get(meaning_id) {
                    meaning.word_id
                } else {
                    return Task::none();
                };

                // Delete clozes
                self.model.cloze_registry.delete_by_meaning(meaning_id);

                // Remove from Word.meaning_ids
                self.model.word_registry.remove_meaning(word_id, meaning_id);

                // Delete meaning
                self.model.meaning_registry.delete(meaning_id);
            }

            // Tag operations
            Message::CreateTag(name) => {
                let trimmed = name.trim();
                if !trimmed.is_empty() {
                    let existing = self
                        .model
                        .tag_registry
                        .iter()
                        .find(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase());
                    if existing.is_none() {
                        let tag = crate::models::Tag::builder()
                            .name(trimmed.to_string())
                            .build();
                        self.model.tag_registry.add(tag);
                    }
                }
            }
            Message::DeleteTag(tag_id) => {
                self.model.tag_registry.delete(tag_id);
            }
            Message::AddTagToMeaning(meaning_id, tag_id) => {
                self.model.meaning_registry.add_tag(meaning_id, tag_id);
                window.words_ui.active_tag_dropdown = None;
            }
            Message::RemoveTagFromMeaning(meaning_id, tag_id) => {
                self.model.meaning_registry.remove_tag(meaning_id, tag_id);
            }
            Message::WordsMeaningToggleTagDropdown(meaning_id) => {
                window.words_ui.active_tag_dropdown =
                    if window.words_ui.active_tag_dropdown == Some(meaning_id) {
                        None
                    } else {
                        Some(meaning_id)
                    };
            }
            Message::WordsMeaningTagSearchChanged(_value) => {
                // No-op: tag search is handled per-meaning via meanings_tag_search_input
            }
            Message::AddTagToMeaningSearch(meaning_id, tag_name) => {
                // Check if we should auto-create the tag
                let trimmed = tag_name.trim();
                if !trimmed.is_empty() {
                    let existing = self
                        .model
                        .tag_registry
                        .iter()
                        .find(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase());
                    if let Some((_, tag)) = existing {
                        self.model.meaning_registry.add_tag(meaning_id, tag.id);
                        window.words_ui.active_tag_dropdown = None;
                    }
                }
            }

            // Batch tag operations for selected meanings
            Message::BatchAddTagToSelectedMeanings(tag_id) => {
                for &meaning_id in &window.selected_meaning_ids {
                    self.model.meaning_registry.add_tag(meaning_id, tag_id);
                }
                window.words_ui.meanings_tag_dropdown_state = TagDropdownState::None;
                window.words_ui.meanings_tag_search_input.clear();
            }
            Message::BatchRemoveTagFromSelectedMeanings(tag_id) => {
                for &meaning_id in &window.selected_meaning_ids {
                    self.model.meaning_registry.remove_tag(meaning_id, tag_id);
                }
                window.words_ui.meanings_tag_dropdown_state = TagDropdownState::None;
                window.words_ui.meanings_tag_remove_search_input.clear();
            }
            Message::ToggleMeaningsAddTagDropdown => {
                window.words_ui.meanings_tag_dropdown_state =
                    if window.words_ui.meanings_tag_dropdown_state == TagDropdownState::Add {
                        TagDropdownState::None
                    } else {
                        TagDropdownState::Add
                    };
                if window.words_ui.meanings_tag_dropdown_state == TagDropdownState::Add {
                    window.words_ui.meanings_tag_search_input.clear();
                }
            }
            Message::ToggleMeaningsRemoveTagDropdown => {
                window.words_ui.meanings_tag_dropdown_state =
                    if window.words_ui.meanings_tag_dropdown_state == TagDropdownState::Remove {
                        TagDropdownState::None
                    } else {
                        TagDropdownState::Remove
                    };
                if window.words_ui.meanings_tag_dropdown_state == TagDropdownState::Remove {
                    window.words_ui.meanings_tag_remove_search_input.clear();
                }
            }
            Message::MeaningsTagSearchChanged(value) => {
                window.words_ui.meanings_tag_search_input = value;
            }
            Message::MeaningsTagRemoveSearchChanged(value) => {
                window.words_ui.meanings_tag_remove_search_input = value;
            }

            // Selection - Words
            Message::ToggleWord(word_id) => {
                if window.selected_word_ids.contains(&word_id) {
                    window.selected_word_ids.remove(&word_id);
                    // Deselect all meanings of this word
                    for (mid, _) in self.model.meaning_registry.iter_by_word(word_id) {
                        window.selected_meaning_ids.remove(mid);
                    }
                } else {
                    window.selected_word_ids.insert(word_id);
                    // Select all meanings of this word
                    for (mid, _) in self.model.meaning_registry.iter_by_word(word_id) {
                        window.selected_meaning_ids.insert(*mid);
                    }
                }
            }
            Message::ToggleMeaning(meaning_id) => {
                // Toggle meaning selection
                if window.selected_meaning_ids.contains(&meaning_id) {
                    window.selected_meaning_ids.remove(&meaning_id);
                } else {
                    window.selected_meaning_ids.insert(meaning_id);
                }

                // Get word_id for this meaning
                if let Some(meaning) = self.model.meaning_registry.get(meaning_id) {
                    let word_id = meaning.word_id;

                    // Check if ALL meanings of word are selected
                    let all_meanings_selected: bool = self
                        .model
                        .meaning_registry
                        .iter_by_word(word_id)
                        .all(|(mid, _)| window.selected_meaning_ids.contains(mid));

                    // Sync word selection
                    if all_meanings_selected {
                        window.selected_word_ids.insert(word_id);
                    } else {
                        window.selected_word_ids.remove(&word_id);
                    }
                }
            }
            Message::SelectAllWords => {
                for (id, _) in self.model.word_registry.iter() {
                    window.selected_word_ids.insert(*id);
                    // Also select all meanings of this word
                    for (mid, _) in self.model.meaning_registry.iter_by_word(*id) {
                        window.selected_meaning_ids.insert(*mid);
                    }
                }
            }
            Message::DeselectAllWords => {
                window.selected_word_ids.clear();
                window.selected_meaning_ids.clear();
            }

            // Selection - Tags
            Message::ToggleTag(tag_id) => {
                if window.selected_tag_ids.contains(&tag_id) {
                    window.selected_tag_ids.remove(&tag_id);
                } else {
                    window.selected_tag_ids.insert(tag_id);
                }
            }
            Message::SelectAllTags => {
                for (_, tag) in self.model.tag_registry.iter() {
                    window.selected_tag_ids.insert(tag.id);
                }
            }
            Message::DeselectAllTags => {
                window.selected_tag_ids.clear();
            }

            Message::WordsInputChanged(value) => {
                window.words_ui.word_input = value;
            }
            Message::WordsTagFilterChanged(value) => {
                window.words_ui.tag_filter = value;
            }
            Message::WordsClearTagFilter => {
                window.words_ui.tag_filter.clear();
            }

            // UI - Tags
            Message::TagsInputChanged(value) => {
                window.tags_ui.input = value;
            }
            Message::TagsToggleCollapse(id) => {
                if window.tags_ui.collapsed_ids.contains(&id) {
                    window.tags_ui.collapsed_ids.remove(&id);
                } else {
                    window.tags_ui.collapsed_ids.insert(id);
                }
            }
            Message::TagsSelectTag(id) => {
                window.selected_tag_ids.insert(id);
            }
            Message::TagsDeselectTag(id) => {
                window.selected_tag_ids.remove(&id);
            }

            // Window management - handled at app level
            Message::WindowOpened(_, _) => {}
            Message::WindowCloseRequested(_) => {}
            Message::WindowClosed(_) => {}

            // Queue
            Message::QueueSelectToggle(item_id) => {
                if let Some(item) = self.model.queue_registry.get_item(item_id) {
                    if item.selected {
                        self.model.queue_registry.deselect(item_id);
                    } else {
                        self.model.queue_registry.select(item_id);
                    }
                }
            }
            Message::QueueSelectAll => {
                self.model.queue_registry.select_all();
            }
            Message::QueueDeselectAll => {
                self.model.queue_registry.deselect_all();
            }
            Message::QueueSelected => {
                // Queue all selected meanings
                for &meaning_id in &window.selected_meaning_ids {
                    self.model.queue_registry.enqueue(meaning_id);
                }
                window.selected_meaning_ids.clear();
            }
            Message::QueueProcess => {
                let generator = self.model.generator.generator();
                return QueueState::process(
                    &mut self.model.queue_registry,
                    &generator,
                    &self.model.word_registry,
                    &self.model.meaning_registry,
                );
            }
            Message::QueueClearCompleted => {
                self.model.queue_registry.clear_completed();
            }
            Message::QueueRemove(item_id) => {
                self.model.queue_registry.remove(item_id);
            }
            Message::QueueGenerationResult(result) => {
                self.model.queue_registry.set_completed(result.item_id);
                self.model.cloze_registry.add(result.cloze);
            }
        }
        Task::none()
    }
}
