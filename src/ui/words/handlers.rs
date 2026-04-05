//! Words panel command handlers.
//!
//! Command handlers process messages and update state. Each handler processes
//! a flattened message variant directly.

use crate::models::types::{ClozeId, MeaningId, TagId, WordId};
use crate::models::{CefrLevel, Meaning, PartOfSpeech, Tag, Word};
use crate::state::Model;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::{DetailSelection, EditContext, TagDropdownState};
use iced::Task;

/// Handle all words-related messages.
///
/// Returns `Task<WordsMessage>` for async operations.
pub fn update(
    state: &mut crate::ui::words::WordsState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        // Search
        WordsMessage::SearchQueryChanged(query) => {
            state.search.set_query(query);
        }
        WordsMessage::SearchCleared => {
            state.search.clear_query();
        }

        // Filter
        WordsMessage::TagFilterChanged(tag_id) => {
            state.search.set_tag_filter(tag_id);
        }
        WordsMessage::FiltersCleared => {
            state.search.clear_filters();
        }

        // Selection
        WordsMessage::WordToggled(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                let word = word.clone();
                state.selection.toggle_word(&word);
            }
        }
        WordsMessage::MeaningToggled(meaning_id) => {
            state.selection.toggle_meaning(meaning_id);
        }
        WordsMessage::ClozeToggled(cloze_id) => {
            state.selection.toggle_cloze(cloze_id);
        }
        WordsMessage::SelectAllTriggered => {
            state.selection.select_all_meanings(&model.meaning_registry);
        }
        WordsMessage::DeselectAllTriggered => {
            state.selection.clear_all();
        }

        // Detail panel selection
        WordsMessage::WordSelected(word_id) => {
            state.detail.select_word(word_id);
        }
        WordsMessage::MeaningSelected(meaning_id) => {
            state.detail.select_meaning(meaning_id);
        }
        WordsMessage::ClozeSelected(cloze_id) => {
            state.detail.select_cloze(cloze_id);
        }
        WordsMessage::DetailClosed => {
            state.detail.clear_selection();
        }

        // Detail panel editing - start operations
        WordsMessage::NewWordStarted => {
            state.detail.clear_selection();
            state.edit.start_new_word();
        }
        WordsMessage::AddMeaningStarted(word_id) => {
            state.detail.clear_selection();
            state.edit.start_add_meaning(word_id);
        }
        WordsMessage::EditWordStarted(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                state
                    .edit
                    .start_edit_word(word_id, word.content.clone(), word.language.clone());
            }
        }
        WordsMessage::EditMeaningStarted(meaning_id) => {
            if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                state.edit.start_edit_meaning(
                    meaning_id,
                    meaning.definition.clone(),
                    meaning.pos,
                    meaning.cefr_level,
                );
            }
        }

        // Detail panel editing - field updates
        WordsMessage::EditWordContentChanged(content) => {
            state.edit.update_word_content(content);
        }
        WordsMessage::EditWordLanguageChanged(lang) => {
            state.edit.update_word_language(lang);
        }
        WordsMessage::EditNewWordContentChanged(content) => {
            state.edit.update_word_content(content);
        }
        WordsMessage::EditNewWordLanguageChanged(lang) => {
            state.edit.update_word_language(lang);
        }
        WordsMessage::EditMeaningDefinitionChanged(definition) => {
            state.edit.update_meaning_definition(definition);
        }
        WordsMessage::EditMeaningPosChanged(pos) => {
            state.edit.update_meaning_pos(pos);
        }
        WordsMessage::EditMeaningCefrChanged(cefr) => {
            state.edit.update_meaning_cefr(cefr);
        }

        // Detail panel editing - save/cancel
        WordsMessage::EditSaved => {
            match state.edit.context() {
                EditContext::Word(id) => {
                    if let Some(word) = model.word_registry.get_mut(id) {
                        let trimmed = state.edit.buffer().word_content.trim();
                        if !trimmed.is_empty() {
                            word.content = trimmed.to_string();
                            tracing::debug!("Updated word: {} (id={})", word.content, id);
                        }
                    }
                }
                EditContext::Meaning(id) => {
                    if let Some(meaning) = model.meaning_registry.get_mut(id) {
                        let trimmed = state.edit.buffer().meaning_definition.trim();
                        if !trimmed.is_empty() {
                            meaning.definition = trimmed.to_string();
                        }
                        meaning.pos = state.edit.buffer().meaning_pos;
                        meaning.cefr_level = state.edit.buffer().meaning_cefr;
                        tracing::debug!("Updated meaning: {} (id={})", meaning.definition, id);
                    }
                }
                EditContext::None => {}
            }
            state.edit.clear_context();
        }
        WordsMessage::EditCancelled => {
            state.edit.clear_context();
        }

        // Word CRUD
        WordsMessage::WordCreated { content } => {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                // Check for duplicate
                let exists = model
                    .word_registry
                    .iter()
                    .any(|(_, w)| w.content.to_lowercase() == trimmed.to_lowercase());
                if !exists {
                    let word = Word::builder().content(trimmed.to_string()).build();
                    tracing::debug!("Creating word: {} (id={})", word.content, word.id);
                    model.word_registry.add(word);
                    state.search.clear_query();
                }
            }
        }
        WordsMessage::WordDeleted(id) => {
            tracing::debug!("Deleting word: {}", id);
            // Delete all clozes for meanings of this word
            if let Some(word) = model.word_registry.get(id) {
                for meaning_id in &word.meaning_ids {
                    model.cloze_registry.delete_by_meaning(*meaning_id);
                }
            }
            // Delete all meanings
            model.meaning_registry.delete_by_word(id);
            // Delete word
            model.word_registry.delete(id);
            // Clear selection for this word's meanings
            if let Some(word) = model.word_registry.get(id) {
                for mid in &word.meaning_ids {
                    state.selection.remove_meaning(mid);
                }
            }
        }
        WordsMessage::WordExpanded(id) => {
            state.expansion.expand(id);
        }
        WordsMessage::WordCollapsed(id) => {
            state.expansion.collapse(id);
        }
        WordsMessage::WordsExpandedAll => {
            let ids: Vec<_> = model.word_registry.iter().map(|(id, _)| *id).collect();
            state.expansion.expand_all(ids);
        }
        WordsMessage::WordsCollapsedAll => {
            state.expansion.collapse_all();
        }

        // Meaning CRUD
        WordsMessage::MeaningAddStarted { word_id } => {
            state.edit.start_new_meaning(word_id);
        }
        WordsMessage::MeaningAddInput { definition } => {
            state.edit.update_new_meaning_definition(definition);
        }
        WordsMessage::MeaningAddPos { pos } => {
            state.edit.update_new_meaning_pos(pos);
        }
        WordsMessage::MeaningAddCefr { level } => {
            state.edit.update_new_meaning_cefr(level);
        }
        WordsMessage::MeaningAddSaved => {
            if let Some(word_id) = state.edit.new_meaning_form().word_id {
                let trimmed = state.edit.new_meaning_form().definition.trim();
                if !trimmed.is_empty() {
                    let mut meaning = Meaning::builder()
                        .word_id(word_id)
                        .definition(trimmed.to_string())
                        .pos(state.edit.new_meaning_form().pos)
                        .build();
                    meaning.cefr_level = state.edit.new_meaning_form().cefr_level;

                    tracing::debug!(
                        "Creating meaning: {} (id={}, word_id={})",
                        meaning.definition,
                        meaning.id,
                        word_id
                    );
                    model.meaning_registry.add(meaning.clone());
                    model.word_registry.add_meaning(word_id, meaning.id);
                }
            }
            state.edit.clear_new_meaning();
        }
        WordsMessage::MeaningAddCancelled => {
            state.edit.clear_new_meaning();
        }
        WordsMessage::MeaningDeleted(id) => {
            let word_id = model.meaning_registry.get(id).map(|m| m.word_id);

            if let Some(word_id) = word_id {
                tracing::debug!("Deleting meaning: {} from word {}", id, word_id);
                model.cloze_registry.delete_by_meaning(id);
                model.word_registry.remove_meaning(word_id, id);
                model.meaning_registry.delete(id);
                state.selection.remove_meaning(&id);
            }
        }

        // Tag operations
        WordsMessage::TagDropdownOpened { for_meaning } => {
            state
                .detail
                .open_tag_dropdown(TagDropdownTarget::SingleMeaning(for_meaning));
        }
        WordsMessage::TagBatchDropdownOpened => {
            state
                .detail
                .open_tag_dropdown(TagDropdownTarget::SelectedMeanings);
        }
        WordsMessage::TagSearchChanged(query) => {
            if let Some(ref mut dropdown) = state.detail.tag_dropdown_mut() {
                dropdown.search = query;
            }
        }
        WordsMessage::TagAddedToMeaning { meaning_id, tag_id } => {
            model.meaning_registry.add_tag(meaning_id, tag_id);
            state.detail.close_tag_dropdown();
        }
        WordsMessage::TagAddedToSelected { tag_id } => {
            for meaning_id in state.selection.selected_meanings().iter() {
                model.meaning_registry.add_tag(*meaning_id, tag_id);
            }
            state.detail.close_tag_dropdown();
        }
        WordsMessage::TagRemovedFromMeaning { meaning_id, tag_id } => {
            model.meaning_registry.remove_tag(meaning_id, tag_id);
        }
        WordsMessage::TagQuickCreated { meaning_id, name } => {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                // Check for existing tag
                let existing = model
                    .tag_registry
                    .iter()
                    .find(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase());

                let tag_id = if let Some((_, tag)) = existing {
                    tag.id
                } else {
                    // Create new tag
                    let tag = Tag::builder().name(trimmed.to_string()).build();
                    let id = tag.id;
                    model.tag_registry.add(tag);
                    tracing::debug!("Created tag: {} (id={})", trimmed, id);
                    id
                };

                model.meaning_registry.add_tag(meaning_id, tag_id);
                state.detail.close_tag_dropdown();
            }
        }
        WordsMessage::TagDropdownClosed => {
            state.detail.close_tag_dropdown();
        }

        // Cloze operations
        WordsMessage::ClozeDeleted(cloze_id) => {
            model.cloze_registry.delete(cloze_id);
            tracing::debug!("Deleted cloze: {}", cloze_id);
        }

        // Batch operations
        WordsMessage::MeaningsQueuedForGeneration => {
            let count = state.selection.meaning_count();
            for meaning_id in state.selection.selected_meanings().iter() {
                model.queue_registry.enqueue(*meaning_id);
            }
            tracing::info!("Added {} meanings to queue", count);
            state.selection.clear_all();
        }
        WordsMessage::MeaningsDeleted => {
            let count = state.selection.meaning_count();
            let meaning_ids: Vec<Uuid> = state
                .selection
                .selected_meanings()
                .iter()
                .copied()
                .collect();

            for meaning_id in &meaning_ids {
                let word_id = model.meaning_registry.get(*meaning_id).map(|m| m.word_id);

                if let Some(word_id) = word_id {
                    model.cloze_registry.delete_by_meaning(*meaning_id);
                    model.word_registry.remove_meaning(word_id, *meaning_id);
                    model.meaning_registry.delete(*meaning_id);
                }
            }

            tracing::info!("Deleted {} meanings", count);
            state.selection.clear_all();
        }
        WordsMessage::ClozesDeleted => {
            let count = state.selection.cloze_count();
            let cloze_ids: Vec<Uuid> = state.selection.selected_clozes().iter().copied().collect();

            for cloze_id in &cloze_ids {
                model.cloze_registry.delete(*cloze_id);
            }

            tracing::info!("Deleted {} clozes", count);
            state.selection.clear_clozes();
        }

        // Export operations
        WordsMessage::ExportPlaintext => {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Text", &["txt"])
                .set_file_name("clozes.txt")
                .save_file()
            {
                // Collect cloze sentences from selected clozes
                let sentences: Vec<String> = state
                    .selection
                    .selected_clozes()
                    .iter()
                    .filter_map(|cloze_id| {
                        model
                            .cloze_registry
                            .get(*cloze_id)
                            .map(|c| c.render_answers())
                    })
                    .collect();

                // Write to file (one sentence per line)
                if let Err(e) = std::fs::write(&path, sentences.join("\n")) {
                    tracing::error!(error = %e, "Failed to write plaintext export");
                } else {
                    tracing::info!(count = sentences.len(), path = ?path, "Exported clozes to plaintext");
                }
            }
        }

        // These messages are handled in update.rs with full model access
        WordsMessage::NewWordSaved | WordsMessage::NewMeaningSaved => {
            // Handled by main update
        }
    }
    Task::none()
}
