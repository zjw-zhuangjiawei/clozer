use std::fmt;

use crate::assets;
use crate::models::types::{MeaningId, WordId};
use crate::models::{CefrLevel, PartOfSpeech};
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::components::{CheckboxState, svg_checkbox};
use crate::ui::state::MainWindowState;
use crate::ui::theme::{Breakpoint, ButtonSize};
use crate::ui::words::message::{
    BatchMessage, DetailMessage, ExportMessage, FilterMessage, MeaningMessage, SearchMessage,
    SelectionMessage, TagMessage, WordMessage, WordsMessage,
};
use crate::ui::words::state::{ClozeFilter, TagDropdownState, TagDropdownTarget, WordsState};
use iced::Element;
use iced::widget::{
    Button, Column, Container, PickList, Row, Text, TextInput, button, container, svg,
};
use strum::VariantArray;
use uuid::Uuid;

/// Wrapper for CEFR level picker that handles None option.
#[derive(Debug, Clone, PartialEq)]
enum CefrLevelOption {
    None,
    Some(CefrLevel),
}

impl CefrLevelOption {
    fn to_option(&self) -> Option<CefrLevel> {
        match self {
            CefrLevelOption::None => None,
            CefrLevelOption::Some(level) => Some(*level),
        }
    }

    fn from_option(opt: Option<CefrLevel>) -> Self {
        match opt {
            None => CefrLevelOption::None,
            Some(level) => CefrLevelOption::Some(level),
        }
    }
}

impl fmt::Display for CefrLevelOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CefrLevelOption::None => write!(f, "None"),
            CefrLevelOption::Some(level) => write!(f, "{}", level),
        }
    }
}

// Renders the words panel.
pub fn view<'a>(
    state: &'a MainWindowState,
    model: &'a Model,
    breakpoint: Breakpoint,
) -> Element<'a, WordsMessage> {
    let words_state = &state.words;
    let (left_ratio, right_ratio) = breakpoint.column_ratio();

    // Search and filter bar
    let search_bar = build_search_bar(words_state, model, breakpoint);

    // Word tree (left panel)
    let word_tree = build_word_tree(state, model);

    if breakpoint.is_single_column() {
        // Mobile: single column layout (word tree only, no detail panel)
        Column::new()
            .push(search_bar)
            .push(iced::widget::rule::horizontal(1))
            .push(iced::widget::scrollable(word_tree).height(iced::Length::Fill))
            .push(build_action_bar(words_state, model))
            .spacing(10)
            .padding(10)
            .height(iced::Length::Fill)
            .into()
    } else {
        // Tablet/Desktop: two-column layout
        let left_panel = Column::new()
            .push(search_bar)
            .push(iced::widget::rule::horizontal(1))
            .push(iced::widget::scrollable(word_tree).height(iced::Length::Fill))
            .push(build_action_bar(words_state, model))
            .spacing(10)
            .padding(10)
            .width(iced::Length::FillPortion((left_ratio * 10.0) as u16));

        // Detail panel (right panel)
        let colors = AppTheme::default().colors();
        let right_panel = Container::new(crate::ui::words::detail_view::view(
            Some(words_state.detail_selection),
            words_state.edit_context,
            &words_state.edit_buffer,
            model,
        ))
        .width(iced::Length::FillPortion((right_ratio * 10.0) as u16))
        .height(iced::Length::Fill)
        .style(move |_| container::Style {
            background: Some(colors.surface_elevated.into()),
            ..Default::default()
        });

        // Two-column layout
        Row::new()
            .push(left_panel)
            .push(right_panel)
            .spacing(5)
            .height(iced::Length::Fill)
            .into()
    }
}

/// Build the search and filter bar.
fn build_search_bar<'a>(
    words_state: &'a WordsState,
    _model: &'a Model,
    breakpoint: Breakpoint,
) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

    // Search input
    let search_input = TextInput::new("Search words or definitions...", &words_state.query)
        .on_input(|s| WordsMessage::Search(SearchMessage::QueryChanged(s)))
        .width(iced::Length::Fill)
        .padding(8);

    // Cloze filter dropdown - responsive width based on breakpoint
    let cloze_filter_width = match breakpoint {
        Breakpoint::Mobile => iced::Length::Fixed(80.0),
        Breakpoint::Tablet => iced::Length::Fixed(100.0),
        Breakpoint::Desktop => iced::Length::Fixed(120.0),
    };
    let cloze_filter = PickList::new(
        vec![
            ClozeFilter::All,
            ClozeFilter::HasClozes,
            ClozeFilter::Pending,
            ClozeFilter::Failed,
        ],
        Some(words_state.filter.cloze_status),
        |f| WordsMessage::Filter(FilterMessage::ByClozeStatus(f)),
    )
    .width(cloze_filter_width)
    .placeholder("Filter");

    // Clear filter button
    let clear_btn = if !words_state.query.is_empty()
        || words_state.filter.cloze_status != ClozeFilter::All
        || words_state.filter.tag_id.is_some()
    {
        Button::new(Text::new("Clear"))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::Filter(FilterMessage::Clear))
    } else {
        Button::new(Text::new("Clear"))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
    };

    Row::new()
        .push(search_input)
        .push(cloze_filter)
        .push(clear_btn)
        .spacing(10)
        .align_y(iced::Alignment::Center)
        .into()
}

/// Build the word tree.
fn build_word_tree<'a>(state: &'a MainWindowState, model: &'a Model) -> Element<'a, WordsMessage> {
    let words_state = &state.words;

    // Filter words based on search and filter state
    let filtered_word_ids: Vec<WordId> = model
        .word_registry
        .iter()
        .filter(|(_, word)| {
            // Search filter
            let matches_search = if words_state.query.is_empty() {
                true
            } else {
                let query = words_state.query.to_lowercase();
                // Check word content
                if word.content.to_lowercase().contains(&query) {
                    return true;
                }
                // Check meanings
                for mid in &word.meaning_ids {
                    if let Some(meaning) = model.meaning_registry.get(*mid) {
                        if meaning.definition.to_lowercase().contains(&query) {
                            return true;
                        }
                    }
                }
                false
            };

            // Cloze status filter
            let matches_cloze_filter = match words_state.filter.cloze_status {
                ClozeFilter::All => true,
                ClozeFilter::HasClozes => word
                    .meaning_ids
                    .iter()
                    .any(|mid| model.cloze_registry.iter_by_meaning_id(*mid).count() > 0),
                ClozeFilter::Pending => word
                    .meaning_ids
                    .iter()
                    .all(|mid| model.cloze_registry.iter_by_meaning_id(*mid).count() == 0),
                ClozeFilter::Failed => false, // TODO: Track failed status
            };

            // Tag filter
            let matches_tag_filter = match words_state.filter.tag_id {
                None => true,
                Some(tag_id) => word.meaning_ids.iter().any(|mid| {
                    model
                        .meaning_registry
                        .get(*mid)
                        .map(|m| m.tag_ids.contains(&tag_id))
                        .unwrap_or(false)
                }),
            };

            matches_search && matches_cloze_filter && matches_tag_filter
        })
        .map(|(id, _)| *id)
        .collect();

    // Build word nodes
    let word_nodes: Vec<Element<'a, WordsMessage>> = filtered_word_ids
        .iter()
        .filter_map(|word_id| model.word_registry.get(*word_id))
        .map(|word| build_word_node(state, model, word))
        .collect();

    if word_nodes.is_empty() {
        Column::new()
            .push(
                Container::new(Text::new("No words found. Add a word to get started."))
                    .center_x(iced::Length::Fill)
                    .padding(20),
            )
            .into()
    } else {
        Column::with_children(word_nodes).spacing(5).into()
    }
}

/// Build a single word node with its meanings.
fn build_word_node<'a>(
    state: &'a MainWindowState,
    model: &'a Model,
    word: &'a crate::models::Word,
) -> Element<'a, WordsMessage> {
    let words_state = &state.words;
    let is_expanded = words_state.expansion.words.contains(&word.id);
    let is_selected = words_state.selection.is_word_selected(word);
    let is_partial = words_state.selection.is_word_partial(word);

    // Get theme colors
    let colors = AppTheme::default().colors();

    // Expand/collapse icon (as button)
    let expand_icon_name = if is_expanded {
        "keyboard_arrow_down_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    } else {
        "keyboard_arrow_right_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    };
    let expand_icon_handle = assets::get_svg(expand_icon_name)
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let expand_icon: Element<'a, WordsMessage> = Button::new(
        svg(expand_icon_handle)
            .width(iced::Length::Fixed(16.0))
            .height(iced::Length::Fixed(16.0)),
    )
    .style(button::secondary)
    .padding(ButtonSize::Small.to_iced_padding())
    .on_press(if is_expanded {
        WordsMessage::Word(WordMessage::Collapse { id: word.id })
    } else {
        WordsMessage::Word(WordMessage::Expand { id: word.id })
    })
    .into();

    // Checkbox state
    let checkbox: Element<'a, WordsMessage> = if word.meaning_ids.is_empty() {
        Text::new("○").into()
    } else if is_partial {
        svg_checkbox(
            CheckboxState::Indeterminate,
            WordsMessage::Selection(SelectionMessage::ToggleWord(word.id)),
        )
    } else {
        svg_checkbox(
            is_selected,
            WordsMessage::Selection(SelectionMessage::ToggleWord(word.id)),
        )
    };

    // Word content (display only - not editable)
    let word_content: Element<'a, WordsMessage> = Button::new(Text::new(&word.content).size(16))
        .style(button::secondary)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Detail(DetailMessage::SelectWord(word.id)))
        .into();

    // Meaning count
    let meaning_count = Text::new(format!("{} meanings", word.meaning_ids.len())).size(12);

    // Word header row
    let word_header = Row::new()
        .push(expand_icon)
        .push(checkbox)
        .push(word_content)
        .push(Text::new(" ").width(iced::Length::Fill))
        .push(meaning_count)
        .push(build_word_actions(word.id.into()))
        .spacing(8)
        .align_y(iced::Alignment::Center);

    // Build expanded content if needed
    if is_expanded {
        let mut content = Column::new()
            .push(word_header)
            .push(iced::widget::rule::horizontal(1))
            .spacing(5);

        // Add meaning button (opens detail panel)
        content = content.push(
            Button::new(Text::new("+ Add Meaning"))
                .style(button::primary)
                .padding(ButtonSize::Medium.to_iced_padding())
                .on_press(WordsMessage::Detail(DetailMessage::StartAddMeaning(
                    word.id,
                ))),
        );

        // Meaning nodes
        for meaning_id in &word.meaning_ids {
            if let Some(meaning) = model.meaning_registry.get(*meaning_id) {
                content = content.push(build_meaning_node(state, model, meaning));
            }
        }

        Container::new(content)
            .padding(10)
            .style(move |_| container::Style {
                background: Some(colors.surface_elevated.into()),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
    } else {
        Container::new(word_header)
            .padding(10)
            .style(move |_| container::Style {
                background: Some(colors.surface.into()),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}

/// Build word action buttons.
fn build_word_actions<'a>(word_id: WordId) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

    // Delete icon
    let delete_icon_handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let delete_icon = svg(delete_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    Button::new(delete_icon)
        .style(button::danger)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Word(WordMessage::Delete {
            id: word_id.into(),
        }))
        .into()
}

/// Build a meaning node with its clozes.
fn build_meaning_node<'a>(
    state: &'a MainWindowState,
    model: &'a Model,
    meaning: &'a crate::models::Meaning,
) -> Element<'a, WordsMessage> {
    let words_state = &state.words;
    let is_selected = words_state.selection.meanings.contains(&meaning.id);
    let cloze_count = model.cloze_registry.iter_by_meaning_id(meaning.id).count();

    // Get theme colors
    let colors = AppTheme::default().colors();

    // Checkbox
    let checkbox = svg_checkbox(
        is_selected,
        WordsMessage::Selection(SelectionMessage::ToggleMeaning(meaning.id)),
    );

    // POS badge
    let pos_badge = Container::new(Text::new(format!("[{}]", meaning.pos)).size(12))
        .padding([2, 6])
        .style(move |_| container::Style {
            background: Some(colors.pos_badge_bg.into()),
            text_color: Some(colors.pos_badge_text),
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    // CEFR level badge (if set)
    let cefr_badge = if let Some(cefr) = meaning.cefr_level {
        Container::new(Text::new(cefr.to_string()).size(12))
            .padding([2, 6])
            .style(move |_| container::Style {
                background: Some(colors.surface_elevated.into()),
                text_color: Some(colors.text_secondary),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
    } else {
        Container::new(Text::new(""))
    };

    // Definition - clickable to toggle detail panel
    let definition: Element<'a, WordsMessage> =
        Button::new(Text::new(&meaning.definition).size(14))
            .style(button::secondary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::Detail(DetailMessage::SelectMeaning(
                meaning.id,
            )))
            .into();

    // Cloze status indicator
    let cloze_status = if cloze_count > 0 {
        Text::new(format!("{} clozes ✓", cloze_count)).size(12)
    } else {
        Text::new("0 clozes ○".to_string()).size(12)
    };

    // Meaning header
    let meaning_header = Row::new()
        .push(checkbox)
        .push(pos_badge)
        .push(cefr_badge)
        .push(definition)
        .push(Text::new(" ").width(iced::Length::Fill))
        .push(cloze_status)
        .push(build_meaning_actions(meaning.id.into()))
        .spacing(8)
        .align_y(iced::Alignment::Center);

    // Tags row
    let tags_row = build_tags_row(state, model, meaning);

    // Collect cloze preview elements (owned) - clickable to toggle detail panel
    let cloze_preview_items: Vec<Element<'a, WordsMessage>> = model
        .cloze_registry
        .iter_by_meaning_id(meaning.id)
        .take(2)
        .map(|(cloze_id, cloze)| {
            let text = cloze.render_blanks();
            Button::new(Text::new(text).size(11))
                .style(button::secondary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(WordsMessage::Detail(DetailMessage::SelectCloze(*cloze_id)))
                .into()
        })
        .collect();

    // Build column
    let mut column = Column::new()
        .push(meaning_header)
        .push(tags_row)
        .spacing(5)
        .padding([5, 10]);

    // Add cloze previews if any
    if !cloze_preview_items.is_empty() {
        column = column.push(
            Column::with_children(cloze_preview_items)
                .spacing(2)
                .padding([2, 10]),
        );
    }

    column.into()
}

/// Build meaning action buttons.
fn build_meaning_actions<'a>(meaning_id: MeaningId) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

    // Delete icon
    let delete_icon_handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let delete_icon = svg(delete_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    Button::new(delete_icon)
        .style(button::danger)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::Meaning(MeaningMessage::Delete {
            id: meaning_id.into(),
        }))
        .into()
}

/// Build the tags row for a meaning.
fn build_tags_row<'a>(
    state: &'a MainWindowState,
    model: &'a Model,
    meaning: &'a crate::models::Meaning,
) -> Element<'a, WordsMessage> {
    let words_state = &state.words;

    // Get theme colors
    let colors = AppTheme::default().colors();

    // Tag chips
    let mut tag_chips: Vec<Element<'a, WordsMessage>> = meaning
        .tag_ids
        .iter()
        .filter_map(|tag_id| model.tag_registry.get(*tag_id))
        .map(|tag| {
            Button::new(Text::new(&tag.name).size(11))
                .style(button::secondary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(WordsMessage::Tag(TagMessage::RemoveFromMeaning {
                    meaning_id: meaning.id,
                    tag_id: tag.id,
                }))
                .into()
        })
        .collect();

    // Add tag button
    tag_chips.push(
        Button::new(Text::new("+").size(11))
            .style(button::secondary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::Tag(TagMessage::ShowDropdown {
                meaning_id: meaning.id,
            }))
            .into(),
    );

    // Tag dropdown if active
    let tag_dropdown: Option<Element<'a, WordsMessage>> =
        if let Some(ref dropdown) = words_state.tag_dropdown {
            match dropdown.target {
                TagDropdownTarget::SingleMeaning(mid) if mid == meaning.id => {
                    Some(build_tag_dropdown(dropdown, model))
                }
                _ => None,
            }
        } else {
            None
        };

    let mut row = Row::new()
        .push(Text::new("Tags:").size(11))
        .extend(tag_chips)
        .spacing(4);

    if let Some(dropdown) = tag_dropdown {
        row = row.push(dropdown);
    }

    row.into()
}

/// Build tag dropdown.
fn build_tag_dropdown<'a>(
    dropdown: &'a TagDropdownState,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

    let search = TextInput::new("Search or create...", &dropdown.search)
        .on_input(|s| WordsMessage::Tag(TagMessage::Search { query: s }))
        .width(iced::Length::Fixed(150.0))
        .padding(4);

    // Filter tags by search
    let filtered_tags: Vec<_> = model
        .tag_registry
        .iter()
        .filter(|(_, tag)| {
            dropdown.search.is_empty()
                || tag
                    .name
                    .to_lowercase()
                    .contains(&dropdown.search.to_lowercase())
        })
        .map(|(_, tag)| tag)
        .take(5)
        .collect();

    let mut tag_items: Vec<Element<'a, WordsMessage>> = filtered_tags
        .iter()
        .map(|tag| {
            Button::new(Text::new(&tag.name).size(12))
                .width(iced::Length::Fill)
                .on_press(WordsMessage::Tag(TagMessage::AddToMeaning {
                    meaning_id: if let TagDropdownTarget::SingleMeaning(mid) = dropdown.target {
                        mid
                    } else {
                        MeaningId(Uuid::nil())
                    },
                    tag_id: tag.id.into(),
                }))
                .into()
        })
        .collect();

    // Quick create option if search doesn't match any tag
    if !dropdown.search.is_empty()
        && !model
            .tag_registry
            .iter()
            .any(|(_, t)| t.name.to_lowercase() == dropdown.search.to_lowercase())
    {
        let search = dropdown.search.clone();
        tag_items.push(
            Button::new(Text::new(format!("Create \"{}\"", search)).size(12))
                .width(iced::Length::Fill)
                .on_press(WordsMessage::Tag(TagMessage::QuickCreate {
                    meaning_id: if let TagDropdownTarget::SingleMeaning(mid) = dropdown.target {
                        mid
                    } else {
                        MeaningId(Uuid::nil())
                    },
                    name: search,
                }))
                .into(),
        );
    }

    Container::new(
        Column::new()
            .push(search)
            .extend(tag_items)
            .spacing(4)
            .padding(5),
    )
    .width(iced::Length::Fixed(170.0))
    .style(move |_| container::Style {
        background: Some(colors.surface.into()),
        border: iced::Border {
            color: colors.border,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
    .into()
}

/// Build the contextual action bar (shows when items selected).
fn build_action_bar<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

    let meaning_selected_count = words_state.selection.meaning_count();
    let cloze_selected_count = words_state.selection.cloze_count();

    // Check for cloze selection first
    if cloze_selected_count > 0 {
        let selection_info = Text::new(format!("☑ {} clozes selected", cloze_selected_count));

        // Export dropdown
        let export_options = vec!["Plaintext", "TypstPdf"];
        let export_dropdown = PickList::new(export_options.to_vec(), None::<&str>, |_| {
            WordsMessage::Export(ExportMessage::ToPlaintext)
        })
        .width(iced::Length::Fixed(140.0))
        .placeholder("Export");

        let delete_btn = Button::new(Text::new("Delete Clozes"))
            .style(button::danger)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::Batch(BatchMessage::DeleteSelectedClozes));

        return Row::new()
            .push(selection_info)
            .push(Text::new(" ").width(iced::Length::Fill))
            .push(export_dropdown)
            .push(delete_btn)
            .spacing(10)
            .align_y(iced::Alignment::Center)
            .into();
    }

    // No cloze selection - check for meaning selection
    if meaning_selected_count == 0 {
        // Show search input and add button
        let search_input = TextInput::new("Search words...", &words_state.query)
            .on_input(|s| WordsMessage::Search(SearchMessage::QueryChanged(s)))
            .width(iced::Length::Fill)
            .padding(8);

        let add_btn = Button::new(Text::new("+ Add"))
            .style(button::primary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::Detail(DetailMessage::StartNewWord));

        return Row::new()
            .push(search_input)
            .push(add_btn)
            .spacing(10)
            .into();
    }

    // Selection info for meanings
    let selection_info = Text::new(format!("☑ {} meanings selected", meaning_selected_count));

    // Batch tag dropdown if active
    let tag_btn: Element<'a, WordsMessage> = if let Some(ref dropdown) = words_state.tag_dropdown {
        match dropdown.target {
            TagDropdownTarget::SelectedMeanings => Row::new()
                .push(
                    Button::new(Text::new("Add Tag ▾"))
                        .style(button::primary)
                        .padding(ButtonSize::Standard.to_iced_padding()),
                )
                .push(build_batch_tag_dropdown(dropdown, model))
                .spacing(2)
                .into(),
            _ => Button::new(Text::new("Add Tag"))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(WordsMessage::Tag(TagMessage::ShowBatchDropdown))
                .into(),
        }
    } else {
        Button::new(Text::new("Add Tag"))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::Tag(TagMessage::ShowBatchDropdown))
            .into()
    };

    let queue_btn = Button::new(Text::new("Queue"))
        .style(button::primary)
        .padding(ButtonSize::Standard.to_iced_padding())
        .on_press(WordsMessage::Batch(BatchMessage::QueueSelected));

    let delete_btn = Button::new(Text::new("Delete"))
        .style(button::danger)
        .padding(ButtonSize::Standard.to_iced_padding())
        .on_press(WordsMessage::Batch(BatchMessage::DeleteSelected));

    Row::new()
        .push(selection_info)
        .push(Text::new(" ").width(iced::Length::Fill))
        .push(tag_btn)
        .push(queue_btn)
        .push(delete_btn)
        .spacing(10)
        .align_y(iced::Alignment::Center)
        .into()
}

/// Build batch tag dropdown for selected meanings.
fn build_batch_tag_dropdown<'a>(
    dropdown: &'a TagDropdownState,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Get theme colors
    let colors = AppTheme::default().colors();

    let search = TextInput::new("Search...", &dropdown.search)
        .on_input(|s| WordsMessage::Tag(TagMessage::Search { query: s }))
        .width(iced::Length::Fixed(150.0))
        .padding(4);

    // Filter tags by search
    let filtered_tags: Vec<_> = model
        .tag_registry
        .iter()
        .filter(|(_, tag)| {
            dropdown.search.is_empty()
                || tag
                    .name
                    .to_lowercase()
                    .contains(&dropdown.search.to_lowercase())
        })
        .map(|(_, tag)| tag)
        .take(5)
        .collect();

    let tag_items: Vec<Element<'a, WordsMessage>> = filtered_tags
        .iter()
        .map(|tag| {
            Button::new(Text::new(&tag.name).size(12))
                .width(iced::Length::Fill)
                .on_press(WordsMessage::Tag(TagMessage::AddToSelected {
                    tag_id: tag.id,
                }))
                .into()
        })
        .collect();

    Container::new(
        Column::new()
            .push(search)
            .extend(tag_items)
            .spacing(4)
            .padding(5),
    )
    .width(iced::Length::Fixed(170.0))
    .style(move |_| container::Style {
        background: Some(colors.surface.into()),
        border: iced::Border {
            color: colors.border,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    })
    .into()
}
