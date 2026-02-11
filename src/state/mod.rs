pub mod data;
pub mod generator;
pub mod queue;
pub mod selection;
pub mod ui;

pub use self::data::DataState;
pub use self::generator::{Generator, GeneratorState};
pub use self::queue::{QueueGenerationResult, QueueState};
pub use self::selection::SelectionState;
pub use self::ui::UiState;

use crate::Message;
use crate::models::{Meaning, Word};
use crate::state::ui::TagDropdownState;
use iced::Task;

#[derive(Debug, Clone)]
pub struct AppState {
    pub data: DataState,
    pub queue: QueueState,
    pub generator: GeneratorState,
    pub selection: SelectionState,
    pub ui: UiState,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            data: DataState::new(),
            queue: QueueState::new(),
            generator: GeneratorState::new(),
            selection: SelectionState::new(),
            ui: UiState::new(),
        }
    }

    pub fn with_sample_data(mut self) -> Self {
        self.data = self.data.with_sample_data();
        self
    }

    // Unified Update Handler

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Word operations
            Message::CreateWord(content) => {
                let trimmed = content.trim();
                if !trimmed.is_empty() {
                    let word = Word::builder().content(trimmed.to_string()).build();
                    self.data.word_registry.insert(word);
                }
            }
            Message::DeleteWord(word_id) => {
                // Delete all clozes for meanings of this word
                for (meaning_id, _) in self.data.meaning_registry.iter_by_word(word_id) {
                    self.data.cloze_registry.delete_by_meaning(*meaning_id);
                }
                // Delete all meanings
                self.data.meaning_registry.delete_by_word(word_id);
                // Delete word
                self.data.word_registry.delete(word_id);
                self.selection.selected_word_ids.remove(&word_id);
            }
            Message::DeleteSelected => {
                for &id in &self.selection.selected_word_ids {
                    // Delete clozes for meanings
                    for (meaning_id, _) in self.data.meaning_registry.iter_by_word(id) {
                        self.data.cloze_registry.delete_by_meaning(*meaning_id);
                    }
                    // Delete meanings
                    self.data.meaning_registry.delete_by_word(id);
                    // Delete word
                    self.data.word_registry.delete(id);
                }
                self.selection.selected_word_ids.clear();
            }
            Message::ToggleWordExpand(word_id) => {
                if self.ui.words.expanded_word_ids.contains(&word_id) {
                    self.ui.words.expanded_word_ids.remove(&word_id);
                } else {
                    self.ui.words.expanded_word_ids.insert(word_id);
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

                    self.data.meaning_registry.insert(meaning.clone());

                    // Update Word.meaning_ids
                    self.data.word_registry.add_meaning(word_id, meaning.id);
                }
            }
            Message::SaveMeaning(word_id) => {
                if let Some(input) = self.ui.words.meaning_inputs.get(&word_id) {
                    // Create new Meaning from input
                    let meaning = Meaning::builder()
                        .word_id(word_id)
                        .definition(input.definition.clone())
                        .pos(input.pos)
                        .build();

                    self.data.meaning_registry.insert(meaning.clone());

                    // Update Word.meaning_ids
                    self.data.word_registry.add_meaning(word_id, meaning.id);

                    // Hide input
                    self.ui.words.meaning_inputs.remove(&word_id);
                }
            }
            Message::CancelMeaningInput(word_id) => {
                self.ui.words.meaning_inputs.remove(&word_id);
            }
            Message::ToggleMeaningInput(word_id) => {
                let input = self.ui.words.meaning_inputs.entry(word_id).or_default();
                input.visible = !input.visible;
                if !input.visible {
                    self.ui.words.meaning_inputs.remove(&word_id);
                }
            }
            Message::MeaningDefInputChanged(word_id, value) => {
                self.ui
                    .words
                    .meaning_inputs
                    .entry(word_id)
                    .or_default()
                    .definition = value;
            }
            Message::MeaningPosSelected(word_id, pos) => {
                self.ui.words.meaning_inputs.entry(word_id).or_default().pos = pos;
            }
            Message::DeleteMeaning(meaning_id) => {
                // Get word_id for cleanup
                let word_id =
                    if let Some(meaning) = self.data.meaning_registry.get_by_id(meaning_id) {
                        meaning.word_id
                    } else {
                        return Task::none();
                    };

                // Delete clozes
                self.data.cloze_registry.delete_by_meaning(meaning_id);

                // Remove from Word.meaning_ids
                self.data.word_registry.remove_meaning(word_id, meaning_id);

                // Delete meaning
                self.data.meaning_registry.delete(meaning_id);
            }

            // Tag operations
            Message::CreateTag(name) => {
                let trimmed = name.trim();
                if !trimmed.is_empty() {
                    let existing = self
                        .data
                        .tag_registry
                        .iter()
                        .find(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase());
                    if existing.is_none() {
                        let tag = crate::models::Tag::builder()
                            .name(trimmed.to_string())
                            .build();
                        self.data.tag_registry.insert(tag);
                    }
                }
            }
            Message::DeleteTag(tag_id) => {
                self.data.tag_registry.delete(tag_id);
            }
            Message::AddTagToMeaning(meaning_id, tag_id) => {
                self.data.meaning_registry.add_tag(meaning_id, tag_id);
                self.ui.words.active_tag_dropdown = None;
            }
            Message::RemoveTagFromMeaning(meaning_id, tag_id) => {
                self.data.meaning_registry.remove_tag(meaning_id, tag_id);
            }
            Message::WordsMeaningToggleTagDropdown(meaning_id) => {
                self.ui.words.active_tag_dropdown =
                    if self.ui.words.active_tag_dropdown == Some(meaning_id) {
                        None
                    } else {
                        Some(meaning_id)
                    };
                self.ui.words.tag_search_input.clear();
            }
            Message::WordsMeaningTagSearchChanged(value) => {
                self.ui.words.tag_search_input = value;
            }
            Message::AddTagToMeaningSearch(meaning_id, tag_name) => {
                self.ui.words.tag_search_input = tag_name.clone();
                // Check if we should auto-create the tag
                let trimmed = tag_name.trim();
                if !trimmed.is_empty() {
                    let existing = self
                        .data
                        .tag_registry
                        .iter()
                        .find(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase());
                    if let Some((_, tag)) = existing {
                        self.data.meaning_registry.add_tag(meaning_id, tag.id);
                        self.ui.words.active_tag_dropdown = None;
                        self.ui.words.tag_search_input.clear();
                    }
                }
            }

            // Batch tag operations for selected meanings
            Message::BatchAddTagToSelectedMeanings(tag_id) => {
                for &meaning_id in &self.selection.selected_meaning_ids {
                    self.data.meaning_registry.add_tag(meaning_id, tag_id);
                }
                self.ui.words.meanings_tag_dropdown_state = TagDropdownState::None;
                self.ui.words.meanings_tag_search_input.clear();
            }
            Message::BatchRemoveTagFromSelectedMeanings(tag_id) => {
                for &meaning_id in &self.selection.selected_meaning_ids {
                    self.data.meaning_registry.remove_tag(meaning_id, tag_id);
                }
                self.ui.words.meanings_tag_dropdown_state = TagDropdownState::None;
                self.ui.words.meanings_tag_remove_search_input.clear();
            }
            Message::ToggleMeaningsAddTagDropdown => {
                self.ui.words.meanings_tag_dropdown_state =
                    if self.ui.words.meanings_tag_dropdown_state == TagDropdownState::Add {
                        TagDropdownState::None
                    } else {
                        TagDropdownState::Add
                    };
                if self.ui.words.meanings_tag_dropdown_state == TagDropdownState::Add {
                    self.ui.words.meanings_tag_search_input.clear();
                }
            }
            Message::ToggleMeaningsRemoveTagDropdown => {
                self.ui.words.meanings_tag_dropdown_state =
                    if self.ui.words.meanings_tag_dropdown_state == TagDropdownState::Remove {
                        TagDropdownState::None
                    } else {
                        TagDropdownState::Remove
                    };
                if self.ui.words.meanings_tag_dropdown_state == TagDropdownState::Remove {
                    self.ui.words.meanings_tag_remove_search_input.clear();
                }
            }
            Message::MeaningsTagSearchChanged(value) => {
                self.ui.words.meanings_tag_search_input = value;
            }
            Message::MeaningsTagRemoveSearchChanged(value) => {
                self.ui.words.meanings_tag_remove_search_input = value;
            }

            // Cloze operations
            Message::DeleteCloze(cloze_id) => {
                self.data.cloze_registry.delete(cloze_id);
            }
            Message::CreateCloze(meaning_id, sentence) => {
                let trimmed = sentence.trim();
                if !trimmed.is_empty() {
                    let segments = crate::models::Cloze::parse_from_sentence(trimmed);
                    let cloze = crate::models::Cloze::builder()
                        .meaning_id(meaning_id)
                        .segments(segments)
                        .build();
                    self.data.cloze_registry.insert(cloze);
                    self.ui.words.cloze_inputs.remove(&meaning_id);
                }
            }
            Message::ClozeInputChanged(meaning_id, value) => {
                self.ui.words.cloze_inputs.insert(meaning_id, value);
            }

            // Selection - Words
            Message::ToggleWord(word_id) => {
                if self.selection.selected_word_ids.contains(&word_id) {
                    self.selection.selected_word_ids.remove(&word_id);
                    // Deselect all meanings of this word
                    for (mid, _) in self.data.meaning_registry.iter_by_word(word_id) {
                        self.selection.selected_meaning_ids.remove(mid);
                    }
                } else {
                    self.selection.selected_word_ids.insert(word_id);
                    // Select all meanings of this word
                    for (mid, _) in self.data.meaning_registry.iter_by_word(word_id) {
                        self.selection.selected_meaning_ids.insert(*mid);
                    }
                }
            }
            Message::ToggleMeaning(meaning_id) => {
                // Toggle meaning selection
                if self.selection.selected_meaning_ids.contains(&meaning_id) {
                    self.selection.selected_meaning_ids.remove(&meaning_id);
                } else {
                    self.selection.selected_meaning_ids.insert(meaning_id);
                }

                // Get word_id for this meaning
                if let Some(meaning) = self.data.meaning_registry.get_by_id(meaning_id) {
                    let word_id = meaning.word_id;

                    // Check if ALL meanings of word are selected
                    let all_meanings_selected: bool = self
                        .data
                        .meaning_registry
                        .iter_by_word(word_id)
                        .all(|(mid, _)| self.selection.selected_meaning_ids.contains(mid));

                    // Sync word selection
                    if all_meanings_selected {
                        self.selection.selected_word_ids.insert(word_id);
                    } else {
                        self.selection.selected_word_ids.remove(&word_id);
                    }
                }
            }
            Message::SelectAllWords => {
                for (id, _) in self.data.word_registry.iter() {
                    self.selection.selected_word_ids.insert(*id);
                }
            }
            Message::DeselectAllWords => {
                self.selection.selected_word_ids.clear();
            }

            // Selection - Tags
            Message::ToggleTag(tag_id) => {
                if self.selection.selected_tag_ids.contains(&tag_id) {
                    self.selection.selected_tag_ids.remove(&tag_id);
                } else {
                    self.selection.selected_tag_ids.insert(tag_id);
                }
            }
            Message::SelectAllTags => {
                for (_, tag) in self.data.tag_registry.iter() {
                    self.selection.selected_tag_ids.insert(tag.id);
                }
            }
            Message::DeselectAllTags => {
                self.selection.selected_tag_ids.clear();
            }

            Message::WordsInputChanged(value) => {
                self.ui.words.word_input = value;
            }
            Message::WordsTagFilterChanged(value) => {
                self.ui.words.tag_filter = value;
            }
            Message::WordsClearTagFilter => {
                self.ui.words.tag_filter.clear();
            }
            Message::WordsToggleTagDropdown => {
                self.ui.words.tag_dropdown_state =
                    if self.ui.words.tag_dropdown_state == TagDropdownState::Add {
                        TagDropdownState::None
                    } else {
                        TagDropdownState::Add
                    };
                if self.ui.words.tag_dropdown_state == TagDropdownState::Add {
                    self.ui.words.tag_search_input.clear();
                    self.ui.words.tag_remove_search_input.clear();
                }
            }
            Message::WordsTagSearchChanged(value) => {
                self.ui.words.tag_search_input = value;
            }
            Message::WordsToggleRemoveTagDropdown => {
                self.ui.words.tag_dropdown_state =
                    if self.ui.words.tag_dropdown_state == TagDropdownState::Remove {
                        TagDropdownState::None
                    } else {
                        TagDropdownState::Remove
                    };
                if self.ui.words.tag_dropdown_state == TagDropdownState::Remove {
                    self.ui.words.tag_remove_search_input.clear();
                    self.ui.words.tag_search_input.clear();
                }
            }
            Message::WordsTagRemoveSearchChanged(value) => {
                self.ui.words.tag_remove_search_input = value;
            }

            // UI - Tags
            Message::TagsInputChanged(value) => {
                self.ui.tags.input = value;
            }
            Message::TagsToggleCollapse(id) => {
                if self.ui.tags.collapsed_ids.contains(&id) {
                    self.ui.tags.collapsed_ids.remove(&id);
                } else {
                    self.ui.tags.collapsed_ids.insert(id);
                }
            }
            Message::TagsSelectTag(id) => {
                self.ui.tags.selected_ids.insert(id);
            }
            Message::TagsDeselectTag(id) => {
                self.ui.tags.selected_ids.remove(&id);
            }

            // Queue
            Message::QueueSelectToggle(item_id) => {
                if let Some(item) = self.queue.queue_registry.get_item(item_id) {
                    if item.selected {
                        self.queue.queue_registry.deselect(item_id);
                    } else {
                        self.queue.queue_registry.select(item_id);
                    }
                }
            }
            Message::QueueSelectAll => {
                self.queue.queue_registry.select_all();
            }
            Message::QueueDeselectAll => {
                self.queue.queue_registry.deselect_all();
            }
            Message::QueueSelected => {
                // Queue all meanings of selected words
                for &word_id in &self.selection.selected_word_ids {
                    for (meaning_id, _) in self.data.meaning_registry.iter_by_word(word_id) {
                        self.queue.queue_registry.enqueue(*meaning_id);
                    }
                }
            }
            Message::QueueProcess => {
                let generator = self.generator.generator();
                return self.queue.process(
                    &generator,
                    &self.data.word_registry,
                    &self.data.meaning_registry,
                );
            }
            Message::QueueClearCompleted => {
                self.queue.queue_registry.clear_completed();
            }
            Message::QueueRemove(item_id) => {
                self.queue.queue_registry.remove(item_id);
            }
            Message::QueueGenerationResult(result) => {
                self.queue.queue_registry.set_completed(result.item_id);
                self.data.cloze_registry.insert(result.cloze);
            }
        }
        Task::none()
    }
}
