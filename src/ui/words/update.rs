//! Words panel update handler.

use crate::models::types::{ClozeId, MeaningId};
use crate::models::{Meaning, Tag, Word};
use crate::state::Model;
use crate::ui::state::MainWindowState;
use crate::ui::words::manager::{EditContext, TagDropdownTarget};
use crate::ui::words::message::WordsMessage;
use iced::Task;

/// Handles all words-related messages.
pub fn update(
    state: &mut MainWindowState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        // Search
        WordsMessage::SearchQueryChanged(query) => {
            state.words.search.set_query(query);
        }
        WordsMessage::SearchCleared => {
            state.words.search.clear_query();
        }

        // Filter
        WordsMessage::ClozeFilterChanged(filter) => {
            state.words.search.set_cloze_filter(filter);
        }
        WordsMessage::TagFilterChanged(tag_id) => {
            state.words.search.set_tag_filter(tag_id);
        }
        WordsMessage::FiltersCleared => {
            state.words.search.clear_filters();
        }

        // Selection
        WordsMessage::WordToggled(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                let word = word.clone();
                state.words.selection.toggle_word(&word);
            }
        }
        WordsMessage::MeaningToggled(meaning_id) => {
            state.words.selection.toggle_meaning(meaning_id);
        }
        WordsMessage::ClozeToggled(cloze_id) => {
            state.words.selection.toggle_cloze(cloze_id);
        }
        WordsMessage::SelectAllTriggered => {
            state
                .words
                .selection
                .select_all_meanings(&model.meaning_registry);
        }
        WordsMessage::DeselectAllTriggered => {
            state.words.selection.clear_all();
        }

        // Detail panel selection
        WordsMessage::WordSelected(word_id) => {
            if state.words.detail.get_selection()
                == crate::ui::words::manager::DetailSelection::Word(word_id)
            {
                state.words.detail.clear_selection();
            } else {
                state.words.detail.select_word(word_id);
            }
        }
        WordsMessage::MeaningSelected(meaning_id) => {
            if state.words.detail.get_selection()
                == crate::ui::words::manager::DetailSelection::Meaning(meaning_id)
            {
                state.words.detail.clear_selection();
            } else {
                state.words.detail.select_meaning(meaning_id);
            }
        }
        WordsMessage::ClozeSelected(cloze_id) => {
            if state.words.detail.get_selection()
                == crate::ui::words::manager::DetailSelection::Cloze(cloze_id)
            {
                state.words.detail.clear_selection();
            } else {
                state.words.detail.select_cloze(cloze_id);
            }
        }
        WordsMessage::DetailClosed => {
            state.words.detail.clear_selection();
        }

        // Detail panel editing - start operations
        WordsMessage::NewWordStarted => {
            state.words.detail.clear_selection();
            state.words.edit.start_new_word();
        }
        WordsMessage::AddMeaningStarted(word_id) => {
            state.words.detail.clear_selection();
            state.words.edit.start_add_meaning(word_id);
        }
        WordsMessage::EditWordStarted(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                state.words.edit.start_edit_word(
                    word_id,
                    word.content.clone(),
                    word.language.clone(),
                );
            }
        }
        WordsMessage::EditMeaningStarted(meaning_id) => {
            if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                state.words.edit.start_edit_meaning(
                    meaning_id,
                    meaning.definition.clone(),
                    meaning.pos,
                    meaning.cefr_level,
                );
            }
        }

        // Detail panel editing - field updates
        WordsMessage::EditWordContentChanged(content) => {
            state.words.edit.update_word_content(content);
        }
        WordsMessage::EditWordLanguageChanged(lang) => {
            state.words.edit.update_word_language(lang);
        }
        WordsMessage::EditNewWordContentChanged(content) => {
            state.words.edit.update_word_content(content);
        }
        WordsMessage::EditNewWordLanguageChanged(lang) => {
            state.words.edit.update_word_language(lang);
        }
        WordsMessage::EditMeaningDefinitionChanged(definition) => {
            state.words.edit.update_meaning_definition(definition);
        }
        WordsMessage::EditMeaningPosChanged(pos) => {
            state.words.edit.update_meaning_pos(pos);
        }
        WordsMessage::EditMeaningCefrChanged(cefr) => {
            state.words.edit.update_meaning_cefr(cefr);
        }

        // Detail panel editing - save/cancel
        WordsMessage::EditSaved => {
            match state.words.edit.context() {
                EditContext::Word(id) => {
                    if let Some(word) = model.word_registry.get_mut(id) {
                        let trimmed = state.words.edit.buffer().word_content.trim();
                        if !trimmed.is_empty() {
                            word.content = trimmed.to_string();
                            tracing::debug!("Updated word: {} (id={})", word.content, id);
                        }
                    }
                }
                EditContext::Meaning(id) => {
                    if let Some(meaning) = model.meaning_registry.get_mut(id) {
                        let trimmed = state.words.edit.buffer().meaning_definition.trim();
                        if !trimmed.is_empty() {
                            meaning.definition = trimmed.to_string();
                        }
                        meaning.pos = state.words.edit.buffer().meaning_pos;
                        meaning.cefr_level = state.words.edit.buffer().meaning_cefr;
                        tracing::debug!("Updated meaning: {} (id={})", meaning.definition, id);
                    }
                }
                // NewWord is handled by NewWordSaved, not EditSaved
                EditContext::NewWord => {}
                // NewMeaning is handled by NewMeaningSaved, not EditSaved
                EditContext::NewMeaning(_) => {}
                EditContext::None => {}
            }
            state.words.edit.clear_context();
        }
        WordsMessage::NewWordSaved => {
            let buffer = state.words.edit.buffer();
            let word_content = buffer.word_content.trim();

            if word_content.is_empty() {
                // Cancel if empty
                state.words.edit.clear_context();
                state.words.detail.clear_selection();
                return Task::none();
            }

            // Check for duplicate
            let exists = model
                .word_registry
                .iter()
                .any(|(_, w)| w.content.to_lowercase() == word_content.to_lowercase());
            if exists {
                // Already exists, just exit edit mode
                state.words.edit.clear_context();
                state.words.detail.clear_selection();
                return Task::none();
            }

            // Create Word
            let word_id = if let Some(ref lang) = buffer.word_language {
                let word = Word::builder()
                    .content(word_content.to_string())
                    .language(lang.clone())
                    .build();
                tracing::debug!("Creating word: {} (id={})", word.content, word.id);
                let id = word.id;
                model.word_registry.add(word);
                id
            } else {
                let word = Word::builder().content(word_content.to_string()).build();
                tracing::debug!("Creating word: {} (id={})", word.content, word.id);
                let id = word.id;
                model.word_registry.add(word);
                id
            };

            // If definition is provided, create Meaning
            if !buffer.meaning_definition.trim().is_empty() {
                let meaning = Meaning::builder()
                    .word_id(word_id)
                    .definition(buffer.meaning_definition.trim().to_string())
                    .pos(buffer.meaning_pos)
                    .cefr_level(buffer.meaning_cefr)
                    .build();

                tracing::debug!(
                    "Creating meaning: {} (id={})",
                    meaning.definition,
                    meaning.id
                );

                let meaning_id = meaning.id;
                model.meaning_registry.add(meaning);
                model.word_registry.add_meaning(word_id, meaning_id);
            }

            state.words.edit.clear_context();
            state.words.detail.select_word(word_id);
        }
        WordsMessage::NewMeaningSaved => {
            let buffer = state.words.edit.buffer();
            let definition = buffer.meaning_definition.trim();

            if definition.is_empty() {
                // Cancel if empty
                state.words.edit.clear_context();
                return Task::none();
            }

            if let EditContext::NewMeaning(word_id) = state.words.edit.context() {
                // Create meaning
                let meaning = Meaning::builder()
                    .word_id(word_id)
                    .definition(definition.to_string())
                    .pos(buffer.meaning_pos)
                    .cefr_level(buffer.meaning_cefr)
                    .build();

                tracing::debug!(
                    "Creating meaning: {} (id={}, word_id={})",
                    meaning.definition,
                    meaning.id,
                    word_id
                );

                let meaning_id = meaning.id;
                model.meaning_registry.add(meaning);
                model.word_registry.add_meaning(word_id, meaning_id);

                state.words.edit.clear_context();
                state.words.detail.select_meaning(meaning_id);
            }
        }
        WordsMessage::EditCancelled => {
            state.words.edit.clear_context();
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
                    state.words.search.clear_query();
                }
            }
        }
        WordsMessage::WordDeleted(word_id) => {
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
                    state.words.selection.remove_meaning(mid);
                }
            }
        }
        WordsMessage::WordExpanded(word_id) => {
            state.words.expansion.expand(word_id);
        }
        WordsMessage::WordCollapsed(word_id) => {
            state.words.expansion.collapse(word_id);
        }
        WordsMessage::WordsExpandedAll => {
            let ids: Vec<_> = model.word_registry.iter().map(|(id, _)| *id).collect();
            state.words.expansion.expand_all(ids);
        }
        WordsMessage::WordsCollapsedAll => {
            state.words.expansion.collapse_all();
        }

        // Meaning CRUD (inline form)
        WordsMessage::MeaningAddStarted { word_id } => {
            state.words.edit.start_new_meaning(word_id);
        }
        WordsMessage::MeaningAddInput { definition } => {
            state.words.edit.update_new_meaning_definition(definition);
        }
        WordsMessage::MeaningAddPos { pos } => {
            state.words.edit.update_new_meaning_pos(pos);
        }
        WordsMessage::MeaningAddCefr { level } => {
            state.words.edit.update_new_meaning_cefr(level);
        }
        WordsMessage::MeaningAddSaved => {
            if let Some(word_id) = state.words.edit.new_meaning_form().word_id {
                let trimmed = state.words.edit.new_meaning_form().definition.trim();
                if !trimmed.is_empty() {
                    let mut meaning = Meaning::builder()
                        .word_id(word_id)
                        .definition(trimmed.to_string())
                        .pos(state.words.edit.new_meaning_form().pos)
                        .build();
                    meaning.cefr_level = state.words.edit.new_meaning_form().cefr_level;

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
            state.words.edit.clear_new_meaning();
        }
        WordsMessage::MeaningAddCancelled => {
            state.words.edit.clear_new_meaning();
        }
        WordsMessage::MeaningDeleted(meaning_id) => {
            let word_id = model.meaning_registry.get(meaning_id).map(|m| m.word_id);

            if let Some(word_id) = word_id {
                tracing::debug!("Deleting meaning: {} from word {}", meaning_id, word_id);
                model.cloze_registry.delete_by_meaning(meaning_id);
                model.word_registry.remove_meaning(word_id, meaning_id);
                model.meaning_registry.delete(meaning_id);
                state.words.selection.remove_meaning(&meaning_id);
            }
        }

        // Tag operations
        WordsMessage::TagDropdownOpened { for_meaning } => {
            state
                .words
                .detail
                .open_tag_dropdown(TagDropdownTarget::SingleMeaning(for_meaning));
        }
        WordsMessage::TagBatchDropdownOpened => {
            state
                .words
                .detail
                .open_tag_dropdown(TagDropdownTarget::SelectedMeanings);
        }
        WordsMessage::TagSearchChanged(query) => {
            if let Some(ref mut dropdown) = state.words.detail.tag_dropdown_mut() {
                dropdown.search = query;
            }
        }
        WordsMessage::TagAddedToMeaning { meaning_id, tag_id } => {
            model.meaning_registry.add_tag(meaning_id, tag_id);
            state.words.detail.close_tag_dropdown();
        }
        WordsMessage::TagAddedToSelected { tag_id } => {
            for meaning_id in state.words.selection.selected_meanings().iter() {
                model.meaning_registry.add_tag(*meaning_id, tag_id);
            }
            state.words.detail.close_tag_dropdown();
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
                state.words.detail.close_tag_dropdown();
            }
        }
        WordsMessage::TagDropdownClosed => {
            state.words.detail.close_tag_dropdown();
        }

        // Cloze operations
        WordsMessage::ClozeDeleted(cloze_id) => {
            model.cloze_registry.delete(cloze_id);
            tracing::debug!("Deleted cloze: {}", cloze_id);
        }

        // Batch operations
        WordsMessage::MeaningsQueuedForGeneration => {
            let count = state.words.selection.meaning_count();
            for meaning_id in state.words.selection.selected_meanings().iter() {
                model.queue_registry.enqueue(*meaning_id);
            }
            tracing::info!("Added {} meanings to queue", count);
            state.words.selection.clear_all();
        }
        WordsMessage::MeaningsDeleted => {
            let count = state.words.selection.meaning_count();
            let meaning_ids: Vec<MeaningId> = state
                .words
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
            state.words.selection.clear_all();
        }
        WordsMessage::ClozesDeleted => {
            let count = state.words.selection.cloze_count();
            let cloze_ids: Vec<ClozeId> = state
                .words
                .selection
                .selected_clozes()
                .iter()
                .copied()
                .collect();

            for cloze_id in &cloze_ids {
                model.cloze_registry.delete(*cloze_id);
            }

            tracing::info!("Deleted {} clozes", count);
            state.words.selection.clear_clozes();
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
                    .words
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
    }

    Task::none()
}
