//! Words panel update handler.

use crate::models::types::{ClozeId, MeaningId};
use crate::models::{Meaning, Tag, Word};
use crate::state::Model;
use crate::ui::state::MainWindowState;
use crate::ui::words::message::{
    BatchMessage, ClozeMessage, DetailMessage, ExportMessage, FilterMessage, MeaningMessage,
    SearchMessage, SelectionMessage, TagMessage, WordMessage, WordsMessage,
};
use crate::ui::words::state::{DetailSelection, TagDropdownState};
use iced::Task;

/// Handles all words-related messages.
pub fn update(
    state: &mut MainWindowState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        // Search
        WordsMessage::Search(msg) => match msg {
            SearchMessage::QueryChanged(query) => {
                state.words.query = query;
            }
            SearchMessage::Clear => {
                state.words.query.clear();
            }
        },

        // Filter
        WordsMessage::Filter(msg) => match msg {
            FilterMessage::ByClozeStatus(filter) => {
                state.words.filter.cloze_status = filter;
            }
            FilterMessage::ByTag(tag_id) => {
                state.words.filter.tag_id = tag_id;
            }
            FilterMessage::Clear => {
                state.words.query.clear();
                state.words.filter = Default::default();
            }
        },

        // Selection
        WordsMessage::Selection(msg) => match msg {
            SelectionMessage::ToggleWord(word_id) => {
                if let Some(word) = model.word_registry.get(word_id) {
                    let word = word.clone();
                    state.words.selection.toggle_word(&word);
                }
            }
            SelectionMessage::ToggleMeaning(meaning_id) => {
                state.words.selection.toggle_meaning(meaning_id);
            }
            SelectionMessage::ToggleCloze(cloze_id) => {
                state.words.selection.toggle_cloze(cloze_id);
            }
            SelectionMessage::SelectAll => {
                state.words.selection.select_all(&model.meaning_registry);
            }
            SelectionMessage::DeselectAll => {
                state.words.selection.clear();
            }
        },

        // Detail panel
        WordsMessage::Detail(msg) => match msg {
            DetailMessage::SelectWord(word_id) => {
                let new_selection = DetailSelection::Word(word_id);
                if state.words.detail_selection == new_selection {
                    state.words.detail_selection = DetailSelection::None;
                } else {
                    state.words.detail_selection = new_selection;
                }
            }
            DetailMessage::SelectMeaning(meaning_id) => {
                let new_selection = DetailSelection::Meaning(meaning_id);
                if state.words.detail_selection == new_selection {
                    state.words.detail_selection = DetailSelection::None;
                } else {
                    state.words.detail_selection = new_selection;
                }
            }
            DetailMessage::SelectCloze(cloze_id) => {
                let new_selection = DetailSelection::Cloze(cloze_id);
                if state.words.detail_selection == new_selection {
                    state.words.detail_selection = DetailSelection::None;
                } else {
                    state.words.detail_selection = new_selection;
                }
            }
            DetailMessage::Clear => {
                state.words.detail_selection = DetailSelection::None;
            }
            DetailMessage::StartEditWord(word_id) => {
                if let Some(word) = model.word_registry.get(word_id) {
                    state.words.edit_context = crate::ui::words::state::EditContext::Word(word_id);
                    state.words.edit_buffer.word_content = word.content.clone();
                }
            }
            DetailMessage::StartEditMeaning(meaning_id) => {
                if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                    state.words.edit_context =
                        crate::ui::words::state::EditContext::Meaning(meaning_id);
                    state.words.edit_buffer.meaning_definition = meaning.definition.clone();
                    state.words.edit_buffer.meaning_pos = meaning.pos;
                    state.words.edit_buffer.meaning_cefr = meaning.cefr_level;
                }
            }
            DetailMessage::EditWordContent(content) => {
                state.words.edit_buffer.word_content = content;
            }
            DetailMessage::EditMeaningDefinition(definition) => {
                state.words.edit_buffer.meaning_definition = definition;
            }
            DetailMessage::EditMeaningPos(pos) => {
                state.words.edit_buffer.meaning_pos = pos;
            }
            DetailMessage::EditMeaningCefr(cefr) => {
                state.words.edit_buffer.meaning_cefr = cefr;
            }
            DetailMessage::Save => {
                match state.words.edit_context {
                    crate::ui::words::state::EditContext::Word(id) => {
                        if let Some(word) = model.word_registry.get_mut(id) {
                            let trimmed = state.words.edit_buffer.word_content.trim();
                            if !trimmed.is_empty() {
                                word.content = trimmed.to_string();
                                tracing::debug!("Updated word: {} (id={})", word.content, id);
                            }
                        }
                    }
                    crate::ui::words::state::EditContext::Meaning(id) => {
                        if let Some(meaning) = model.meaning_registry.get_mut(id) {
                            let trimmed = state.words.edit_buffer.meaning_definition.trim();
                            if !trimmed.is_empty() {
                                meaning.definition = trimmed.to_string();
                            }
                            meaning.pos = state.words.edit_buffer.meaning_pos;
                            meaning.cefr_level = state.words.edit_buffer.meaning_cefr;
                            tracing::debug!("Updated meaning: {} (id={})", meaning.definition, id);
                        }
                    }
                    crate::ui::words::state::EditContext::None => {}
                }
                state.words.edit_context = crate::ui::words::state::EditContext::None;
            }
            DetailMessage::Cancel => {
                state.words.edit_context = crate::ui::words::state::EditContext::None;
            }
        },

        // Word operations
        WordsMessage::Word(msg) => match msg {
            WordMessage::Create { content } => {
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
                        state.words.query.clear();
                    }
                }
            }
            WordMessage::Delete { id: word_id } => {
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
                        state.words.selection.meanings.remove(mid);
                    }
                }
            }
            WordMessage::Expand { id: word_id } => {
                state.words.expansion.words.insert(word_id);
            }
            WordMessage::Collapse { id: word_id } => {
                state.words.expansion.words.remove(&word_id);
            }
            WordMessage::ExpandAll => {
                for (id, _) in model.word_registry.iter() {
                    state.words.expansion.words.insert(*id);
                }
            }
            WordMessage::CollapseAll => {
                state.words.expansion.collapse_all();
            }
        },

        // Meaning operations
        WordsMessage::Meaning(msg) => match msg {
            MeaningMessage::AddStart { word_id } => {
                state.words.new_meaning.start(word_id);
            }
            MeaningMessage::AddInput { definition } => {
                state.words.new_meaning.definition = definition;
            }
            MeaningMessage::AddPos { pos } => {
                state.words.new_meaning.pos = pos;
            }
            MeaningMessage::AddCefr { level } => {
                state.words.new_meaning.cefr_level = level;
            }
            MeaningMessage::AddSave => {
                if let Some(word_id) = state.words.new_meaning.word_id {
                    let trimmed = state.words.new_meaning.definition.trim();
                    if !trimmed.is_empty() {
                        let mut meaning = Meaning::builder()
                            .word_id(word_id)
                            .definition(trimmed.to_string())
                            .pos(state.words.new_meaning.pos)
                            .build();
                        meaning.cefr_level = state.words.new_meaning.cefr_level;

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
                state.words.new_meaning.cancel();
            }
            MeaningMessage::AddCancel => {
                state.words.new_meaning.cancel();
            }
            MeaningMessage::Delete { id: meaning_id } => {
                let word_id = model.meaning_registry.get(meaning_id).map(|m| m.word_id);

                if let Some(word_id) = word_id {
                    tracing::debug!("Deleting meaning: {} from word {}", meaning_id, word_id);
                    model.cloze_registry.delete_by_meaning(meaning_id);
                    model.word_registry.remove_meaning(word_id, meaning_id);
                    model.meaning_registry.delete(meaning_id);
                    state.words.selection.meanings.remove(&meaning_id);
                }
            }
        },

        // Tag operations
        WordsMessage::Tag(msg) => match msg {
            TagMessage::ShowDropdown { meaning_id } => {
                state.words.tag_dropdown = Some(TagDropdownState::for_meaning(meaning_id));
            }
            TagMessage::ShowBatchDropdown => {
                state.words.tag_dropdown = Some(TagDropdownState::for_batch());
            }
            TagMessage::Search { query } => {
                if let Some(ref mut dropdown) = state.words.tag_dropdown {
                    dropdown.search = query;
                }
            }
            TagMessage::AddToMeaning { meaning_id, tag_id } => {
                model.meaning_registry.add_tag(meaning_id, tag_id);
                state.words.tag_dropdown = None;
            }
            TagMessage::AddToSelected { tag_id } => {
                for meaning_id in state.words.selection.meanings.iter() {
                    model.meaning_registry.add_tag(*meaning_id, tag_id);
                }
                state.words.tag_dropdown = None;
            }
            TagMessage::RemoveFromMeaning { meaning_id, tag_id } => {
                model.meaning_registry.remove_tag(meaning_id, tag_id);
            }
            TagMessage::QuickCreate { meaning_id, name } => {
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
                    state.words.tag_dropdown = None;
                }
            }
            TagMessage::Close => {
                state.words.tag_dropdown = None;
            }
        },

        // Cloze operations
        WordsMessage::Cloze(msg) => match msg {
            ClozeMessage::Delete { id: cloze_id } => {
                model.cloze_registry.delete(cloze_id);
                tracing::debug!("Deleted cloze: {}", cloze_id);
            }
            ClozeMessage::ToggleSelection { id: cloze_id } => {
                state.words.selection.toggle_cloze(cloze_id);
            }
        },

        // Batch operations
        WordsMessage::Batch(msg) => match msg {
            BatchMessage::QueueSelected => {
                let count = state.words.selection.meaning_count();
                for meaning_id in state.words.selection.meanings.iter() {
                    model.queue_registry.enqueue(*meaning_id);
                }
                tracing::info!("Added {} meanings to queue", count);
                state.words.selection.clear();
            }
            BatchMessage::DeleteSelected => {
                let count = state.words.selection.meaning_count();
                let meaning_ids: Vec<MeaningId> =
                    state.words.selection.meanings.iter().copied().collect();

                for meaning_id in &meaning_ids {
                    let word_id = model.meaning_registry.get(*meaning_id).map(|m| m.word_id);

                    if let Some(word_id) = word_id {
                        model.cloze_registry.delete_by_meaning(*meaning_id);
                        model.word_registry.remove_meaning(word_id, *meaning_id);
                        model.meaning_registry.delete(*meaning_id);
                    }
                }

                tracing::info!("Deleted {} meanings", count);
                state.words.selection.clear();
            }
            BatchMessage::DeleteSelectedClozes => {
                let count = state.words.selection.cloze_count();
                let cloze_ids: Vec<ClozeId> =
                    state.words.selection.clozes.iter().copied().collect();

                for cloze_id in &cloze_ids {
                    model.cloze_registry.delete(*cloze_id);
                }

                tracing::info!("Deleted {} clozes", count);
                state.words.selection.clear_clozes();
            }
        },

        // Export operations
        WordsMessage::Export(msg) => match msg {
            ExportMessage::ToPlaintext => {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Text", &["txt"])
                    .set_file_name("clozes.txt")
                    .save_file()
                {
                    // Collect cloze sentences from selected clozes
                    let sentences: Vec<String> = state
                        .words
                        .selection
                        .clozes
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
            ExportMessage::ToTypstPdf => {
                // TODO: Implement PDF export with Typst
                tracing::warn!("Typst PDF export not yet implemented");
            }
        },
    }

    Task::none()
}
