//! Words panel update handler.

use crate::models::types::{ClozeId, MeaningId};
use crate::models::{Meaning, Tag, Word};
use crate::state::Model;
use crate::ui::words::manager::{DetailPanelState, TagDropdownTarget};
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::Task;

#[allow(deprecated)]
pub fn update(
    state: &mut WordsState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        // Search
        WordsMessage::SearchQueryChanged(query) => {
            state.search.set_query(query);
            // Execute the query immediately
            state.search.execute(
                &model.word_registry,
                &model.meaning_registry,
                &model.cloze_registry,
                &model.queue_registry,
                &model.tag_registry,
            );
        }
        WordsMessage::SearchCleared => {
            state.search.clear_query();
            // Re-execute to show all words
            state.search.execute(
                &model.word_registry,
                &model.meaning_registry,
                &model.cloze_registry,
                &model.queue_registry,
                &model.tag_registry,
            );
        }
        WordsMessage::SortTypeChanged(sort) => {
            state.search.set_sort(sort);
            // Re-execute with new sort
            state.search.execute(
                &model.word_registry,
                &model.meaning_registry,
                &model.cloze_registry,
                &model.queue_registry,
                &model.tag_registry,
            );
        }
        WordsMessage::SuggestionAccepted => {
            if let Some(suggestion) = state.search.get_suggestion(&model.word_registry) {
                state.search.set_query(suggestion);
                state.search.execute(
                    &model.word_registry,
                    &model.meaning_registry,
                    &model.cloze_registry,
                    &model.queue_registry,
                    &model.tag_registry,
                );
            }
        }

        // Filter (now integrated into search query)
        WordsMessage::FiltersCleared => {
            state.search.clear_filters();
            // Re-execute to show all words
            state.search.execute(
                &model.word_registry,
                &model.meaning_registry,
                &model.cloze_registry,
                &model.queue_registry,
                &model.tag_registry,
            );
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
        WordsMessage::WordSelected(word_id) => match state.panel.state() {
            DetailPanelState::WordView { word_id: id } if *id == word_id => {
                state.panel.close();
            }
            _ => {
                state.panel.show_word(word_id);
            }
        },
        WordsMessage::MeaningSelected(meaning_id) => match state.panel.state() {
            DetailPanelState::MeaningView { meaning_id: id } if *id == meaning_id => {
                state.panel.close();
            }
            _ => {
                state.panel.show_meaning(meaning_id);
            }
        },
        WordsMessage::ClozeSelected(cloze_id) => match state.panel.state() {
            DetailPanelState::ClozeView { cloze_id: id } if *id == cloze_id => {
                state.panel.close();
            }
            _ => {
                state.panel.show_cloze(cloze_id);
            }
        },
        WordsMessage::DetailClosed => {
            state.panel.close();
        }

        // Detail panel editing - start operations
        WordsMessage::NewWordStarted => {
            state.panel.close();
            state.panel.start_word_create();
        }
        WordsMessage::MeaningAddStarted { word_id } => {
            state.panel.close();
            state.panel.start_meaning_create(word_id);
        }
        WordsMessage::EditWordStarted(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                state
                    .panel
                    .start_word_edit(word_id, word.content.clone(), word.language.clone());
            }
        }
        WordsMessage::EditMeaningStarted(meaning_id) => {
            if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                state.panel.start_meaning_edit(
                    meaning_id,
                    meaning.definition.clone(),
                    meaning.pos,
                    meaning.cefr_level,
                );
            }
        }

        // Detail panel editing - field updates
        WordsMessage::EditWordContentChanged(content) => {
            state.panel.word_buffer.content = content;
        }
        WordsMessage::EditWordLanguageChanged { input, parsed } => {
            state.panel.word_buffer.language_input = input;
            state.panel.word_buffer.language = parsed;
        }
        WordsMessage::EditMeaningDefinitionChanged(definition) => {
            state.panel.meaning_buffer.definition = definition;
        }
        WordsMessage::EditMeaningPosChanged(pos) => {
            state.panel.meaning_buffer.pos = pos;
        }
        WordsMessage::EditMeaningCefrChanged(cefr) => {
            state.panel.meaning_buffer.cefr = cefr;
        }

        // Detail panel editing - save/cancel
        WordsMessage::EditSaved => {
            match state.panel.state() {
                DetailPanelState::WordEditing { word_id } => {
                    if let Some(word) = model.word_registry.get_mut(*word_id) {
                        let trimmed = state.panel.word_buffer.content.trim();
                        if !trimmed.is_empty() {
                            word.content = trimmed.to_string();
                            tracing::debug!("Updated word: {} (id={})", word.content, word_id);
                        }
                    }
                }
                DetailPanelState::MeaningEditing { meaning_id } => {
                    if let Some(meaning) = model.meaning_registry.get_mut(*meaning_id) {
                        let trimmed = state.panel.meaning_buffer.definition.trim();
                        if !trimmed.is_empty() {
                            meaning.definition = trimmed.to_string();
                        }
                        meaning.pos = state.panel.meaning_buffer.pos;
                        meaning.cefr_level = state.panel.meaning_buffer.cefr;
                        tracing::debug!(
                            "Updated meaning: {} (id={})",
                            meaning.definition,
                            meaning_id
                        );
                    }
                }
                _ => {}
            }
            state.panel.close();
        }
        WordsMessage::NewWordSaved => {
            let word_buffer = &state.panel.word_buffer;
            let meaning_buffer = &state.panel.meaning_buffer;
            let word_content = word_buffer.content.trim();

            if word_content.is_empty() {
                state.panel.close();
                return Task::none();
            }

            let exists = model
                .word_registry
                .iter()
                .any(|(_, w)| w.content.to_lowercase() == word_content.to_lowercase());
            if exists {
                state.panel.close();
                return Task::none();
            }

            let word_id = if let Some(ref lang) = word_buffer.language {
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

            if !meaning_buffer.definition.trim().is_empty() {
                let meaning = Meaning::builder()
                    .word_id(word_id)
                    .definition(meaning_buffer.definition.trim().to_string())
                    .pos(meaning_buffer.pos)
                    .cefr_level(meaning_buffer.cefr)
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

            state.panel.close();
            state.panel.show_word(word_id);
        }
        WordsMessage::MeaningAddSaved => {
            let buffer = &state.panel.meaning_buffer;
            let definition = buffer.definition.trim();

            if definition.is_empty() {
                state.panel.close();
                return Task::none();
            }

            if let DetailPanelState::MeaningCreating { word_id } = state.panel.state() {
                let meaning = Meaning::builder()
                    .word_id(*word_id)
                    .definition(definition.to_string())
                    .pos(buffer.pos)
                    .cefr_level(buffer.cefr)
                    .build();

                tracing::debug!(
                    "Creating meaning: {} (id={}, word_id={})",
                    meaning.definition,
                    meaning.id,
                    word_id
                );

                let meaning_id = meaning.id;
                model.meaning_registry.add(meaning);
                model.word_registry.add_meaning(*word_id, meaning_id);

                state.panel.close();
                state.panel.show_meaning(meaning_id);
            }
        }
        WordsMessage::EditCancelled => {
            state.panel.close();
        }

        // Word CRUD
        WordsMessage::WordCreated { content } => {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
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
        WordsMessage::WordDeleted(word_id) => {
            tracing::debug!("Deleting word: {}", word_id);
            if let Some(word) = model.word_registry.get(word_id) {
                for meaning_id in &word.meaning_ids {
                    model.cloze_registry.delete_by_meaning(*meaning_id);
                }
            }
            model.meaning_registry.delete_by_word(word_id);
            model.word_registry.delete(word_id);
            if let Some(word) = model.word_registry.get(word_id) {
                for mid in &word.meaning_ids {
                    state.selection.remove_meaning(mid);
                }
            }
        }
        WordsMessage::WordExpanded(word_id) => {
            state.expansion.expand(word_id);
        }
        WordsMessage::WordCollapsed(word_id) => {
            state.expansion.collapse(word_id);
        }
        WordsMessage::WordsExpandedAll => {
            let ids: Vec<_> = model.word_registry.iter().map(|(id, _)| *id).collect();
            state.expansion.expand_all(ids);
        }
        WordsMessage::WordsCollapsedAll => {
            state.expansion.collapse_all();
        }

        WordsMessage::MeaningDeleted(meaning_id) => {
            let word_id = model.meaning_registry.get(meaning_id).map(|m| m.word_id);

            if let Some(word_id) = word_id {
                tracing::debug!("Deleting meaning: {} from word {}", meaning_id, word_id);
                model.cloze_registry.delete_by_meaning(meaning_id);
                model.word_registry.remove_meaning(word_id, meaning_id);
                model.meaning_registry.delete(meaning_id);
                state.selection.remove_meaning(&meaning_id);
            }
        }

        // Tag operations
        WordsMessage::TagDropdownOpened { for_meaning } => {
            state
                .panel
                .open_tag_dropdown(TagDropdownTarget::SingleMeaning(for_meaning));
        }
        WordsMessage::TagBatchDropdownOpened => {
            state
                .panel
                .open_tag_dropdown(TagDropdownTarget::SelectedMeanings);
        }
        WordsMessage::TagSearchChanged(query) => {
            if let Some(ref mut dropdown) = state.panel.tag_dropdown_mut() {
                dropdown.search = query;
            }
        }
        WordsMessage::TagAddedToMeaning { meaning_id, tag_id } => {
            model.meaning_registry.add_tag(meaning_id, tag_id);
            state.panel.close_tag_dropdown();
        }
        WordsMessage::TagAddedToSelected { tag_id } => {
            for meaning_id in state.selection.selected_meanings().iter() {
                model.meaning_registry.add_tag(*meaning_id, tag_id);
            }
            state.panel.close_tag_dropdown();
        }
        WordsMessage::TagRemovedFromMeaning { meaning_id, tag_id } => {
            model.meaning_registry.remove_tag(meaning_id, tag_id);
        }
        WordsMessage::TagQuickCreated { meaning_id, name } => {
            let trimmed = name.trim();
            if !trimmed.is_empty() {
                let existing = model
                    .tag_registry
                    .iter()
                    .find(|(_, t)| t.name.to_lowercase() == trimmed.to_lowercase());

                let tag_id = if let Some((_, tag)) = existing {
                    tag.id
                } else {
                    let tag = Tag::builder().name(trimmed.to_string()).build();
                    let id = tag.id;
                    model.tag_registry.add(tag);
                    tracing::debug!("Created tag: {} (id={})", trimmed, id);
                    id
                };

                model.meaning_registry.add_tag(meaning_id, tag_id);
                state.panel.close_tag_dropdown();
            }
        }
        WordsMessage::TagDropdownClosed => {
            state.panel.close_tag_dropdown();
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
            let meaning_ids: Vec<MeaningId> = state
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
            let cloze_ids: Vec<ClozeId> =
                state.selection.selected_clozes().iter().copied().collect();

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

                if let Err(e) = std::fs::write(&path, sentences.join("\n")) {
                    tracing::error!(error = %e, "Failed to write plaintext export");
                    return Task::done(WordsMessage::ExportFailed(e.to_string()));
                } else {
                    tracing::info!(count = sentences.len(), path = ?path, "Exported clozes to plaintext");
                }
            }
        }
        // Export failure — converted to PushNotification in compositor layer
        WordsMessage::ExportFailed(_) => {}
        // Ignore deprecated messages
        WordsMessage::SearchResultsReady(_) | WordsMessage::TagFilterChanged(_) => {
            tracing::warn!("Received deprecated message: {:?}", message);
        }
    }

    Task::none()
}
