//! Words panel update handler.

use crate::models::{Meaning, Tag, Word};
use crate::state::Model;
use crate::ui::state::MainWindowState;
use crate::ui::words::message::{ExportKind, WordsMessage};
use crate::ui::words::state::{TagDropdownState, TagDropdownTarget};
use iced::Task;
use uuid::Uuid;

/// Handles all words-related messages.
pub fn update(
    state: &mut MainWindowState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        // Search & Filter
        WordsMessage::SearchChanged(query) => {
            state.words_ui.search_query = query;
        }
        WordsMessage::FilterByClozeStatus(filter) => {
            state.words_ui.filter.cloze_status = filter;
        }
        WordsMessage::FilterByTag(tag_id) => {
            state.words_ui.filter.tag_id = tag_id;
        }
        WordsMessage::ClearFilter => {
            state.words_ui.search_query.clear();
            state.words_ui.filter = Default::default();
        }

        // Selection
        WordsMessage::ToggleWordSelection(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                let word = word.clone();
                state.words_ui.toggle_word_selection(&word);
            }
        }
        WordsMessage::ToggleMeaningSelection(meaning_id) => {
            state.words_ui.toggle_meaning_selection(meaning_id);
        }
        WordsMessage::SelectAll => {
            state.words_ui.select_all(&model.meaning_registry);
        }
        WordsMessage::DeselectAll => {
            state.words_ui.clear_selection();
        }

        // Expand/Collapse
        WordsMessage::ToggleWordExpand(word_id) => {
            if state.words_ui.expanded_word_ids.contains(&word_id) {
                state.words_ui.expanded_word_ids.remove(&word_id);
            } else {
                state.words_ui.expanded_word_ids.insert(word_id);
            }
        }
        WordsMessage::ToggleClozeExpand(cloze_id) => {
            if state.words_ui.expanded_cloze_ids.contains(&cloze_id) {
                state.words_ui.expanded_cloze_ids.remove(&cloze_id);
            } else {
                state.words_ui.expanded_cloze_ids.insert(cloze_id);
            }
        }
        WordsMessage::ExpandAll => {
            for (id, _) in model.word_registry.iter() {
                state.words_ui.expanded_word_ids.insert(*id);
            }
        }
        WordsMessage::CollapseAll => {
            state.words_ui.expanded_word_ids.clear();
        }

        // Word operations
        WordsMessage::CreateWord(content) => {
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
                    state.words_ui.search_query.clear();
                }
            }
        }
        WordsMessage::EditWordStart(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                state.words_ui.editing_word_id = Some(word_id);
                state.words_ui.editing_word_text = word.content.clone();
            }
        }
        WordsMessage::EditWordInput(text) => {
            state.words_ui.editing_word_text = text;
        }
        WordsMessage::EditWordSave(word_id) => {
            let trimmed = state.words_ui.editing_word_text.trim();
            if !trimmed.is_empty() {
                if let Some(word) = model.word_registry.get_mut(word_id) {
                    word.content = trimmed.to_string();
                    tracing::debug!("Updated word: {} (id={})", word.content, word_id);
                }
            }
            state.words_ui.editing_word_id = None;
            state.words_ui.editing_word_text.clear();
        }
        WordsMessage::EditWordCancel => {
            state.words_ui.editing_word_id = None;
            state.words_ui.editing_word_text.clear();
        }
        WordsMessage::DeleteWord(word_id) => {
            tracing::debug!("Deleting word: {}", word_id);
            // Delete all clozes for meanings of this word
            if let Some(word) = model.word_registry.get(word_id) {
                for meaning_id in &word.meaning_ids {
                    model.cloze_registry.delete_by_meaning(*meaning_id);
                }
            }
            // Delete all meanings
            model.meaning_registry.delete_by_word(word_id);
            // Delete word
            model.word_registry.delete(word_id);
            // Clear selection for this word's meanings
            if let Some(word) = model.word_registry.get(word_id) {
                for mid in &word.meaning_ids {
                    state.words_ui.selected_meaning_ids.remove(mid);
                }
            }
        }

        // Meaning operations
        WordsMessage::AddMeaningStart(word_id) => {
            state.words_ui.adding_meaning_to_word = Some(word_id);
            state.words_ui.meaning_input = Default::default();
        }
        WordsMessage::AddMeaningInput(definition) => {
            state.words_ui.meaning_input.definition = definition;
        }
        WordsMessage::AddMeaningPosSelected(pos) => {
            state.words_ui.meaning_input.pos = pos;
        }
        WordsMessage::AddMeaningSave => {
            if let Some(word_id) = state.words_ui.adding_meaning_to_word {
                let trimmed = state.words_ui.meaning_input.definition.trim();
                if !trimmed.is_empty() {
                    let meaning = Meaning::builder()
                        .word_id(word_id)
                        .definition(trimmed.to_string())
                        .pos(state.words_ui.meaning_input.pos)
                        .build();

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
            state.words_ui.adding_meaning_to_word = None;
            state.words_ui.meaning_input = Default::default();
        }
        WordsMessage::AddMeaningCancel => {
            state.words_ui.adding_meaning_to_word = None;
            state.words_ui.meaning_input = Default::default();
        }
        WordsMessage::EditMeaningStart(meaning_id) => {
            if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                state.words_ui.editing_meaning_id = Some(meaning_id);
                state.words_ui.editing_meaning_text = meaning.definition.clone();
            }
        }
        WordsMessage::EditMeaningInput(text) => {
            state.words_ui.editing_meaning_text = text;
        }
        WordsMessage::EditMeaningSave(meaning_id) => {
            let trimmed = state.words_ui.editing_meaning_text.trim();
            if !trimmed.is_empty() {
                if let Some(meaning) = model.meaning_registry.get_mut(meaning_id) {
                    meaning.definition = trimmed.to_string();
                    tracing::debug!(
                        "Updated meaning: {} (id={})",
                        meaning.definition,
                        meaning_id
                    );
                }
            }
            state.words_ui.editing_meaning_id = None;
            state.words_ui.editing_meaning_text.clear();
        }
        WordsMessage::EditMeaningCancel => {
            state.words_ui.editing_meaning_id = None;
            state.words_ui.editing_meaning_text.clear();
        }
        WordsMessage::DeleteMeaning(meaning_id) => {
            let word_id = model.meaning_registry.get(meaning_id).map(|m| m.word_id);

            if let Some(word_id) = word_id {
                tracing::debug!("Deleting meaning: {} from word {}", meaning_id, word_id);
                model.cloze_registry.delete_by_meaning(meaning_id);
                model.word_registry.remove_meaning(word_id, meaning_id);
                model.meaning_registry.delete(meaning_id);
                state.words_ui.selected_meaning_ids.remove(&meaning_id);
            }
        }

        // Tag operations
        WordsMessage::ShowTagDropdown(meaning_id) => {
            state.words_ui.tag_dropdown = Some(TagDropdownState::new(
                TagDropdownTarget::SingleMeaning(meaning_id),
            ));
        }
        WordsMessage::ShowBatchTagDropdown => {
            state.words_ui.tag_dropdown =
                Some(TagDropdownState::new(TagDropdownTarget::SelectedMeanings));
        }
        WordsMessage::TagSearchChanged(search) => {
            if let Some(ref mut dropdown) = state.words_ui.tag_dropdown {
                dropdown.search = search;
            }
        }
        WordsMessage::AddTagToMeaning(meaning_id, tag_id) => {
            model.meaning_registry.add_tag(meaning_id, tag_id);
            state.words_ui.tag_dropdown = None;
        }
        WordsMessage::AddTagToSelected(tag_id) => {
            for meaning_id in state.words_ui.selected_meaning_ids.iter() {
                model.meaning_registry.add_tag(*meaning_id, tag_id);
            }
            state.words_ui.tag_dropdown = None;
        }
        WordsMessage::RemoveTagFromMeaning(meaning_id, tag_id) => {
            model.meaning_registry.remove_tag(meaning_id, tag_id);
        }
        WordsMessage::QuickCreateTag(meaning_id, name) => {
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
                state.words_ui.tag_dropdown = None;
            }
        }
        WordsMessage::CloseTagDropdown => {
            state.words_ui.tag_dropdown = None;
        }

        // Cloze operations
        WordsMessage::RegenerateCloze(_cloze_id) => {
            // TODO: Implement cloze regeneration
            tracing::warn!("RegenerateCloze not yet implemented");
        }
        WordsMessage::DeleteCloze(cloze_id) => {
            model.cloze_registry.delete(cloze_id);
            tracing::debug!("Deleted cloze: {}", cloze_id);
        }
        WordsMessage::ToggleClozeSelection(cloze_id) => {
            state.words_ui.toggle_cloze_selection(cloze_id);
        }

        // Batch operations
        WordsMessage::QueueSelected => {
            let count = state.words_ui.selected_count();
            for meaning_id in state.words_ui.selected_meaning_ids.iter() {
                model.queue_registry.enqueue(*meaning_id);
            }
            tracing::info!("Added {} meanings to queue", count);
            state.words_ui.clear_selection();
        }
        WordsMessage::DeleteSelected => {
            let count = state.words_ui.selected_count();
            let meaning_ids: Vec<Uuid> = state
                .words_ui
                .selected_meaning_ids
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
            state.words_ui.clear_selection();
        }
        WordsMessage::DeleteSelectedClozes => {
            let count = state.words_ui.selected_cloze_count();
            let cloze_ids: Vec<Uuid> = state.words_ui.selected_cloze_ids.iter().copied().collect();

            for cloze_id in &cloze_ids {
                model.cloze_registry.delete(*cloze_id);
            }

            tracing::info!("Deleted {} clozes", count);
            state.words_ui.clear_cloze_selection();
        }

        // Export operations
        WordsMessage::ExportSelected(kind) => {
            match kind {
                ExportKind::Plaintext => {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Text", &["txt"])
                        .set_file_name("clozes.txt")
                        .save_file()
                    {
                        // Collect cloze sentences from selected clozes
                        let sentences: Vec<String> = state
                            .words_ui
                            .selected_cloze_ids
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
                ExportKind::TypstPdf => {
                    // TODO: Implement PDF export with Typst
                    tracing::warn!("Typst PDF export not yet implemented");
                }
            }
        }
    }

    Task::none()
}
