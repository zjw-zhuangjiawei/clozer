//! Words panel command handlers.
//!
//! Command handlers process messages and update state. Each handler function
//! corresponds to a domain-specific message type.

use crate::models::{Meaning, Tag, Word};
use crate::state::Model;
use crate::ui::words::message::{
    BatchMessage, ClozeMessage, DetailMessage, ExportKind, ExportMessage, FilterMessage,
    ImportMessage, MeaningMessage, SearchMessage, SelectionMessage, TagMessage, WordMessage,
    WordsMessage,
};
use crate::ui::words::state::{DetailSelection, EditContext, TagDropdownState};
use iced::Task;
use uuid::Uuid;

// ============================================================================
// Search Handler
// ============================================================================

/// Handle search-related messages.
pub fn search(state: &mut crate::ui::words::WordsState, message: SearchMessage) {
    match message {
        SearchMessage::QueryChanged(query) => {
            state.query = query;
        }
        SearchMessage::Clear => {
            state.query.clear();
        }
    }
}

// ============================================================================
// Filter Handler
// ============================================================================

/// Handle filter-related messages.
pub fn filter(state: &mut crate::ui::words::WordsState, message: FilterMessage) {
    match message {
        FilterMessage::ByClozeStatus(status) => {
            state.filter.cloze_status = status;
        }
        FilterMessage::ByTag(tag_id) => {
            state.filter.tag_id = tag_id;
        }
        FilterMessage::Clear => {
            state.query.clear();
            state.filter = Default::default();
        }
    }
}

// ============================================================================
// Selection Handler
// ============================================================================

/// Handle selection-related messages.
pub fn selection(
    state: &mut crate::ui::words::WordsState,
    message: SelectionMessage,
    model: &Model,
) {
    match message {
        SelectionMessage::ToggleWord(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                let word = word.clone();
                state.selection.toggle_word(&word);
            }
        }
        SelectionMessage::ToggleMeaning(meaning_id) => {
            state.selection.toggle_meaning(meaning_id);
        }
        SelectionMessage::ToggleCloze(cloze_id) => {
            state.selection.toggle_cloze(cloze_id);
        }
        SelectionMessage::SelectAll => {
            state.selection.select_all(&model.meaning_registry);
        }
        SelectionMessage::DeselectAll => {
            state.selection.clear();
        }
    }
}

// ============================================================================
// Detail Handler
// ============================================================================

/// Handle detail panel messages.
pub fn detail(
    state: &mut crate::ui::words::WordsState,
    message: DetailMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        DetailMessage::SelectWord(word_id) => {
            state.detail_selection.toggle_word(word_id);
        }
        DetailMessage::SelectMeaning(meaning_id) => {
            state.detail_selection.toggle_meaning(meaning_id);
        }
        DetailMessage::SelectCloze(cloze_id) => {
            state.detail_selection.toggle_cloze(cloze_id);
        }
        DetailMessage::Clear => {
            state.detail_selection.clear();
        }
        DetailMessage::StartEditWord(word_id) => {
            if let Some(word) = model.word_registry.get(word_id) {
                state.edit_context = EditContext::Word(word_id);
                state.edit_buffer.word_content = word.content.clone();
            }
        }
        DetailMessage::StartEditMeaning(meaning_id) => {
            if let Some(meaning) = model.meaning_registry.get(meaning_id) {
                state.edit_context = EditContext::Meaning(meaning_id);
                state.edit_buffer.meaning_definition = meaning.definition.clone();
                state.edit_buffer.meaning_pos = meaning.pos;
                state.edit_buffer.meaning_cefr = meaning.cefr_level;
            }
        }
        DetailMessage::EditWordContent(content) => {
            state.edit_buffer.word_content = content;
        }
        DetailMessage::EditMeaningDefinition(definition) => {
            state.edit_buffer.meaning_definition = definition;
        }
        DetailMessage::EditMeaningPos(pos) => {
            state.edit_buffer.meaning_pos = pos;
        }
        DetailMessage::EditMeaningCefr(cefr) => {
            state.edit_buffer.meaning_cefr = cefr;
        }
        DetailMessage::Save => {
            match state.edit_context {
                EditContext::Word(id) => {
                    if let Some(word) = model.word_registry.get_mut(id) {
                        let trimmed = state.edit_buffer.word_content.trim();
                        if !trimmed.is_empty() {
                            word.content = trimmed.to_string();
                            tracing::debug!("Updated word: {} (id={})", word.content, id);
                        }
                    }
                }
                EditContext::Meaning(id) => {
                    if let Some(meaning) = model.meaning_registry.get_mut(id) {
                        let trimmed = state.edit_buffer.meaning_definition.trim();
                        if !trimmed.is_empty() {
                            meaning.definition = trimmed.to_string();
                        }
                        meaning.pos = state.edit_buffer.meaning_pos;
                        meaning.cefr_level = state.edit_buffer.meaning_cefr;
                        tracing::debug!("Updated meaning: {} (id={})", meaning.definition, id);
                    }
                }
                EditContext::None => {}
            }
            state.edit_context.clear();
        }
        DetailMessage::Cancel => {
            state.edit_context.clear();
        }
    }
    Task::none()
}

// ============================================================================
// Word Handler
// ============================================================================

/// Handle word CRUD messages.
pub fn word(
    state: &mut crate::ui::words::WordsState,
    message: WordMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
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
                    state.query.clear();
                }
            }
        }
        WordMessage::Delete { id } => {
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
                    state.selection.meanings.remove(mid);
                }
            }
        }
        WordMessage::Expand { id } => {
            state.expansion.words.insert(id);
        }
        WordMessage::Collapse { id } => {
            state.expansion.words.remove(&id);
        }
        WordMessage::ExpandAll => {
            for (id, _) in model.word_registry.iter() {
                state.expansion.words.insert(*id);
            }
        }
        WordMessage::CollapseAll => {
            state.expansion.words.clear();
        }
    }
    Task::none()
}

// ============================================================================
// Meaning Handler
// ============================================================================

/// Handle meaning CRUD messages.
pub fn meaning(
    state: &mut crate::ui::words::WordsState,
    message: MeaningMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        MeaningMessage::AddStart { word_id } => {
            state.new_meaning.start(word_id);
        }
        MeaningMessage::AddInput { definition } => {
            state.new_meaning.definition = definition;
        }
        MeaningMessage::AddPos { pos } => {
            state.new_meaning.pos = pos;
        }
        MeaningMessage::AddCefr { level } => {
            state.new_meaning.cefr_level = level;
        }
        MeaningMessage::AddSave => {
            if let Some(word_id) = state.new_meaning.word_id {
                let trimmed = state.new_meaning.definition.trim();
                if !trimmed.is_empty() {
                    let mut meaning = Meaning::builder()
                        .word_id(word_id)
                        .definition(trimmed.to_string())
                        .pos(state.new_meaning.pos)
                        .build();
                    meaning.cefr_level = state.new_meaning.cefr_level;

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
            state.new_meaning.cancel();
        }
        MeaningMessage::AddCancel => {
            state.new_meaning.cancel();
        }
        MeaningMessage::Delete { id } => {
            let word_id = model.meaning_registry.get(id).map(|m| m.word_id);

            if let Some(word_id) = word_id {
                tracing::debug!("Deleting meaning: {} from word {}", id, word_id);
                model.cloze_registry.delete_by_meaning(id);
                model.word_registry.remove_meaning(word_id, id);
                model.meaning_registry.delete(id);
                state.selection.meanings.remove(&id);
            }
        }
    }
    Task::none()
}

// ============================================================================
// Tag Handler
// ============================================================================

/// Handle tag operation messages.
pub fn tag(
    state: &mut crate::ui::words::WordsState,
    message: TagMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        TagMessage::ShowDropdown { meaning_id } => {
            state.tag_dropdown = Some(TagDropdownState::for_meaning(meaning_id));
        }
        TagMessage::ShowBatchDropdown => {
            state.tag_dropdown = Some(TagDropdownState::for_batch());
        }
        TagMessage::Search { query } => {
            if let Some(ref mut dropdown) = state.tag_dropdown {
                dropdown.search = query;
            }
        }
        TagMessage::AddToMeaning { meaning_id, tag_id } => {
            model.meaning_registry.add_tag(meaning_id, tag_id);
            state.tag_dropdown = None;
        }
        TagMessage::AddToSelected { tag_id } => {
            for meaning_id in state.selection.meanings.iter() {
                model.meaning_registry.add_tag(*meaning_id, tag_id);
            }
            state.tag_dropdown = None;
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
                state.tag_dropdown = None;
            }
        }
        TagMessage::Close => {
            state.tag_dropdown = None;
        }
    }
    Task::none()
}

// ============================================================================
// Cloze Handler
// ============================================================================

/// Handle cloze operation messages.
pub fn cloze(
    state: &mut crate::ui::words::WordsState,
    message: ClozeMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        ClozeMessage::Delete { id } => {
            model.cloze_registry.delete(id);
            tracing::debug!("Deleted cloze: {}", id);
        }
        ClozeMessage::ToggleSelection { id } => {
            state.selection.toggle_cloze(id);
        }
    }
    Task::none()
}

// ============================================================================
// Batch Handler
// ============================================================================

/// Handle batch operation messages.
pub fn batch(
    state: &mut crate::ui::words::WordsState,
    message: BatchMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        BatchMessage::QueueSelected => {
            let count = state.selection.meaning_count();
            for meaning_id in state.selection.meanings.iter() {
                model.queue_registry.enqueue(*meaning_id);
            }
            tracing::info!("Added {} meanings to queue", count);
            state.selection.clear();
        }
        BatchMessage::DeleteSelected => {
            let count = state.selection.meaning_count();
            let meaning_ids: Vec<Uuid> = state.selection.meanings.iter().copied().collect();

            for meaning_id in &meaning_ids {
                let word_id = model.meaning_registry.get(*meaning_id).map(|m| m.word_id);

                if let Some(word_id) = word_id {
                    model.cloze_registry.delete_by_meaning(*meaning_id);
                    model.word_registry.remove_meaning(word_id, *meaning_id);
                    model.meaning_registry.delete(*meaning_id);
                }
            }

            tracing::info!("Deleted {} meanings", count);
            state.selection.clear();
        }
        BatchMessage::DeleteSelectedClozes => {
            let count = state.selection.cloze_count();
            let cloze_ids: Vec<Uuid> = state.selection.clozes.iter().copied().collect();

            for cloze_id in &cloze_ids {
                model.cloze_registry.delete(*cloze_id);
            }

            tracing::info!("Deleted {} clozes", count);
            state.selection.clear_clozes();
        }
    }
    Task::none()
}

// ============================================================================
// Export Handler
// ============================================================================

/// Handle export operation messages.
pub fn export(
    state: &mut crate::ui::words::WordsState,
    message: ExportMessage,
    model: &Model,
) -> Task<WordsMessage> {
    match message {
        ExportMessage::ToPlaintext => {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Text", &["txt"])
                .set_file_name("clozes.txt")
                .save_file()
            {
                // Collect cloze sentences from selected clozes
                let sentences: Vec<String> = state
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
    }
    Task::none()
}

// ============================================================================
// Import Handler
// ============================================================================

/// Handle import operation messages.
///
/// Returns `Task<WordsMessage>` for async file operations.
pub fn import(
    state: &mut crate::ui::words::WordsState,
    message: ImportMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    match message {
        ImportMessage::MdxStart => {
            // TODO: Open file dialog and trigger import
            // For now, placeholder - implement when parser is ready
            tracing::info!("MDX import requested");
        }
        ImportMessage::MdxFileSelected(path) => {
            // TODO: Start async import
            tracing::info!(path = %path, "MDX file selected for import");
            state.import.start(path);
        }
        ImportMessage::MdxProgress { current, total } => {
            state.import.update_progress(current, total);
        }
        ImportMessage::MdxCompleted { words, meanings } => {
            state.import.complete(words, meanings, 0);
            tracing::info!(words = words, meanings = meanings, "MDX import completed");
        }
        ImportMessage::MdxFailed { error } => {
            state.import.fail(error.clone());
            tracing::error!(error = %error, "MDX import failed");
        }
        ImportMessage::MdxCancel => {
            // TODO: Cancel ongoing import
            state.import.dismiss();
        }
        ImportMessage::Dismiss => {
            state.import.dismiss();
        }
    }
    Task::none()
}

// ============================================================================
// Main Update Function (delegates to domain handlers)
// ============================================================================

/// Handle all words-related messages.
///
/// Returns `Task<WordsMessage>` for async operations.
pub fn update(
    state: &mut crate::ui::words::WordsState,
    message: WordsMessage,
    model: &mut Model,
) -> Task<WordsMessage> {
    use WordsMessage::*;
    match message {
        Search(msg) => search(state, msg),
        Filter(msg) => filter(state, msg),
        Selection(msg) => selection(state, msg, model),
        Detail(msg) => return detail(state, msg, model),
        Word(msg) => word(state, msg, model),
        Meaning(msg) => meaning(state, msg, model),
        Tag(msg) => tag(state, msg, model),
        Cloze(msg) => cloze(state, msg, model),
        Batch(msg) => batch(state, msg, model),
        Export(msg) => export(state, msg, model),
        Import(msg) => return import(state, msg, model),
    }
    Task::none()
}
