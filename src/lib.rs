pub mod models;
pub mod registry;
pub mod state;
pub mod ui;

use iced::{Element, Task};
use uuid::Uuid;

use self::state::{AppState, QueueGenerationResult};
use crate::models::PartOfSpeech;

#[derive(Debug, Clone)]
pub enum Message {
    // Word operations
    CreateWord(String),
    DeleteWord(Uuid),
    DeleteSelected,
    ToggleWordExpand(Uuid),

    // Meaning operations
    CreateMeaning(Uuid, String, PartOfSpeech), // word_id, definition, pos
    SaveMeaning(Uuid),                         // word_id (create from input)
    ToggleMeaning(Uuid),                       // meaning_id
    CancelMeaningInput(Uuid),                  // word_id
    ToggleMeaningInput(Uuid),                  // word_id
    DeleteMeaning(Uuid),
    MeaningDefInputChanged(Uuid, String), // word_id, definition
    MeaningPosSelected(Uuid, PartOfSpeech), // word_id, pos

    // Tag operations
    CreateTag(String),
    DeleteTag(Uuid),
    AddTagToMeaning(Uuid, Uuid),         // meaning_id, tag_id
    RemoveTagFromMeaning(Uuid, Uuid),    // meaning_id, tag_id
    WordsMeaningToggleTagDropdown(Uuid), // meaning_id
    WordsMeaningTagSearchChanged(String),
    AddTagToMeaningSearch(Uuid, String), // meaning_id, tag_name

    // Batch tag operations for selected meanings
    BatchAddTagToSelectedMeanings(Uuid),      // tag_id
    BatchRemoveTagFromSelectedMeanings(Uuid), // tag_id
    ToggleMeaningsAddTagDropdown,             // Toggle Add Tag dropdown
    ToggleMeaningsRemoveTagDropdown,          // Toggle Remove Tag dropdown
    MeaningsTagSearchChanged(String),         // Search for add
    MeaningsTagRemoveSearchChanged(String),   // Search for remove

    // Cloze operations
    DeleteCloze(Uuid),
    CreateCloze(Uuid, String),       // meaning_id, sentence
    ClozeInputChanged(Uuid, String), // meaning_id, sentence

    // Selection - Words
    ToggleWord(Uuid),
    SelectAllWords,
    DeselectAllWords,

    // Selection - Tags
    ToggleTag(Uuid),
    SelectAllTags,
    DeselectAllTags,

    // UI - Words
    WordsInputChanged(String),
    WordsTagFilterChanged(String),
    WordsClearTagFilter,

    // UI - Tags
    TagsInputChanged(String),
    TagsToggleCollapse(Uuid),
    TagsSelectTag(Uuid),
    TagsDeselectTag(Uuid),

    // Queue
    QueueSelectToggle(Uuid),
    QueueSelectAll,
    QueueDeselectAll,
    QueueSelected,
    QueueProcess,
    QueueClearCompleted,
    QueueRemove(Uuid),
    QueueGenerationResult(QueueGenerationResult),
}

#[derive(Debug)]
pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::new(),
        }
    }

    pub fn with_sample_data(mut self) -> Self {
        self.state = self.state.with_sample_data();
        self
    }

    pub fn title(&self) -> String {
        String::from("Clozer")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        self.state.update(message)
    }

    pub fn view(&self) -> Element<'_, Message> {
        let left_panel = ui::words_view(
            &self.state.data.word_registry,
            &self.state.data.meaning_registry,
            &self.state.data.cloze_registry,
            &self.state.data.tag_registry,
            &self.state.ui.words.word_input,
            &self.state.ui.words.tag_filter,
            &self.state.selection.selected_word_ids,
            &self.state.selection.selected_meaning_ids,
            &self.state.ui.words.expanded_word_ids,
            &self.state.ui.words.meaning_inputs,
            &self.state.ui.words.active_tag_dropdown,
            &self.state.ui.words.meanings_tag_dropdown_state,
            &self.state.ui.words.meanings_tag_search_input,
            &self.state.ui.words.meanings_tag_remove_search_input,
        );

        let right_panel = ui::queue_view(
            &self.state.queue.queue_registry,
            &self.state.data.meaning_registry,
            &self.state.data.word_registry,
        );

        iced::widget::row![
            iced::widget::column![left_panel]
                .spacing(20)
                .padding(20)
                .width(iced::Length::FillPortion(2)),
            iced::widget::column![right_panel]
                .width(iced::Length::FillPortion(1))
                .padding(10),
        ]
        .into()
    }
}
