//! Words panel view function.

use super::message::{ExportKind, WordsMessage};
use super::state::{ClozeFilter, TagDropdownState, TagDropdownTarget};
use crate::assets;
use crate::models::PartOfSpeech;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::components::{CheckboxState, svg_checkbox};
use crate::ui::main_window::state::MainWindowState;
use iced::Element;
use iced::widget::{
    Button, Column, Container, PickList, Row, Text, TextInput, button, container, svg,
};
use strum::VariantArray;
use uuid::Uuid;

/// Renders the words panel.
pub fn view<'a>(state: &'a MainWindowState, model: &'a Model) -> Element<'a, WordsMessage> {
    let words_ui = &state.words_ui;

    // Search and filter bar
    let search_bar = build_search_bar(words_ui, model);

    // Word tree
    let word_tree = build_word_tree(state, model);

    // Action bar (contextual, shows when items selected)
    let action_bar = build_action_bar(words_ui, model);

    Column::new()
        .push(search_bar)
        .push(iced::widget::rule::horizontal(1))
        .push(iced::widget::scrollable(word_tree).height(iced::Length::Fill))
        .push(action_bar)
        .spacing(10)
        .padding(10)
        .height(iced::Length::Fill)
        .into()
}

/// Build the search and filter bar.
fn build_search_bar<'a>(
    words_ui: &'a super::state::WordsUiState,
    _model: &'a Model,
) -> Element<'a, WordsMessage> {
    // Search input
    let search_input = TextInput::new("Search words or definitions...", &words_ui.search_query)
        .on_input(WordsMessage::SearchChanged)
        .width(iced::Length::Fill)
        .padding(8);

    // Cloze filter dropdown
    let cloze_filter = PickList::new(
        vec![
            ClozeFilter::All,
            ClozeFilter::HasClozes,
            ClozeFilter::Pending,
            ClozeFilter::Failed,
        ],
        Some(words_ui.filter.cloze_status),
        WordsMessage::FilterByClozeStatus,
    )
    .width(iced::Length::Fixed(120.0))
    .placeholder("Filter");

    // Clear filter button
    let clear_btn = if !words_ui.search_query.is_empty()
        || words_ui.filter.cloze_status != ClozeFilter::All
        || words_ui.filter.tag_id.is_some()
    {
        Button::new(Text::new("Clear"))
            .style(button::secondary)
            .padding([8, 16])
            .on_press(WordsMessage::ClearFilter)
    } else {
        Button::new(Text::new("Clear"))
            .style(button::secondary)
            .padding([8, 16])
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
    let words_ui = &state.words_ui;

    // Filter words based on search and filter state
    let filtered_word_ids: Vec<Uuid> = model
        .word_registry
        .iter()
        .filter(|(_, word)| {
            // Search filter
            let matches_search = if words_ui.search_query.is_empty() {
                true
            } else {
                let query = words_ui.search_query.to_lowercase();
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
            let matches_cloze_filter = match words_ui.filter.cloze_status {
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
            let matches_tag_filter = match words_ui.filter.tag_id {
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
    let words_ui = &state.words_ui;
    let is_expanded = words_ui.expanded_word_ids.contains(&word.id);
    let is_selected = words_ui.is_word_selected(word);
    let is_partial = words_ui.is_word_partial(word);

    // Expand/collapse icon
    let expand_icon_name = if is_expanded {
        "keyboard_arrow_down_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    } else {
        "keyboard_arrow_right_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    };
    let expand_icon_handle = assets::get_svg(expand_icon_name)
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let expand_icon: Element<'a, WordsMessage> = svg(expand_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0))
        .into();

    // Checkbox state
    let checkbox: Element<'a, WordsMessage> = if word.meaning_ids.is_empty() {
        Text::new("○").into()
    } else if is_partial {
        svg_checkbox(
            CheckboxState::Indeterminate,
            WordsMessage::ToggleWordSelection(word.id),
        )
    } else {
        svg_checkbox(is_selected, WordsMessage::ToggleWordSelection(word.id))
    };

    // Word content (editable or display)
    let word_content: Element<'a, WordsMessage> = if words_ui.editing_word_id == Some(word.id) {
        TextInput::new("", &words_ui.editing_word_text)
            .on_input(WordsMessage::EditWordInput)
            .on_submit(WordsMessage::EditWordSave(word.id))
            .width(iced::Length::Fill)
            .padding([2, 4])
            .into()
    } else {
        Button::new(Text::new(&word.content).size(16))
            .style(button::secondary)
            .padding([2, 6])
            .on_press(WordsMessage::ToggleWordExpand(word.id))
            .into()
    };

    // Meaning count
    let meaning_count = Text::new(format!("{} meanings", word.meaning_ids.len())).size(12);

    // Word header row
    let word_header = Row::new()
        .push(expand_icon)
        .push(checkbox)
        .push(word_content)
        .push(Text::new(" ").width(iced::Length::Fill))
        .push(meaning_count)
        .push(build_word_actions(word.id))
        .spacing(8)
        .align_y(iced::Alignment::Center);

    // Get theme colors
    let colors = AppTheme::default().colors();

    // Build expanded content if needed
    if is_expanded {
        let mut content = Column::new()
            .push(word_header)
            .push(iced::widget::rule::horizontal(1))
            .spacing(5);

        // Add meaning form if active
        if words_ui.adding_meaning_to_word == Some(word.id) {
            content = content.push(build_add_meaning_form(words_ui));
        } else {
            // Add meaning button
            content = content.push(
                Button::new(Text::new("+ Add Meaning"))
                    .style(button::secondary)
                    .padding([4, 8])
                    .on_press(WordsMessage::AddMeaningStart(word.id)),
            );
        }

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
fn build_word_actions<'a>(word_id: Uuid) -> Element<'a, WordsMessage> {
    // Edit icon
    let edit_icon_handle = assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let edit_icon = svg(edit_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    // Delete icon
    let delete_icon_handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let delete_icon = svg(delete_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    Row::new()
        .push(
            Button::new(edit_icon)
                .style(button::secondary)
                .padding([2, 6])
                .on_press(WordsMessage::EditWordStart(word_id)),
        )
        .push(
            Button::new(delete_icon)
                .style(button::danger)
                .padding([2, 6])
                .on_press(WordsMessage::DeleteWord(word_id)),
        )
        .spacing(2)
        .into()
}

/// Build the add meaning inline form.
fn build_add_meaning_form<'a>(
    words_ui: &'a super::state::WordsUiState,
) -> Element<'a, WordsMessage> {
    let pos_options = PartOfSpeech::VARIANTS;
    let pos_pick_list = PickList::new(
        pos_options.to_vec(),
        Some(words_ui.meaning_input.pos),
        WordsMessage::AddMeaningPosSelected,
    )
    .width(iced::Length::Fixed(100.0));

    let def_input = TextInput::new("Definition...", &words_ui.meaning_input.definition)
        .on_input(WordsMessage::AddMeaningInput)
        .width(iced::Length::Fill)
        .padding(4);

    let save_btn = Button::new(Text::new("Save"))
        .style(button::primary)
        .padding([4, 8])
        .on_press(WordsMessage::AddMeaningSave);

    let cancel_btn = Button::new(Text::new("Cancel"))
        .style(button::secondary)
        .padding([4, 8])
        .on_press(WordsMessage::AddMeaningCancel);

    Row::new()
        .push(pos_pick_list)
        .push(def_input)
        .push(save_btn)
        .push(cancel_btn)
        .spacing(5)
        .padding([5, 0])
        .into()
}

/// Build a meaning node with its clozes.
fn build_meaning_node<'a>(
    state: &'a MainWindowState,
    model: &'a Model,
    meaning: &'a crate::models::Meaning,
) -> Element<'a, WordsMessage> {
    let words_ui = &state.words_ui;
    let is_selected = words_ui.selected_meaning_ids.contains(&meaning.id);
    let cloze_count = model.cloze_registry.iter_by_meaning_id(meaning.id).count();

    // Get theme colors
    let colors = AppTheme::default().colors();

    // Checkbox
    let checkbox = svg_checkbox(
        is_selected,
        WordsMessage::ToggleMeaningSelection(meaning.id),
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

    // Definition (editable or display)
    let definition: Element<'a, WordsMessage> = if words_ui.editing_meaning_id == Some(meaning.id) {
        TextInput::new("", &words_ui.editing_meaning_text)
            .on_input(WordsMessage::EditMeaningInput)
            .on_submit(WordsMessage::EditMeaningSave(meaning.id))
            .width(iced::Length::Fill)
            .padding([2, 4])
            .into()
    } else {
        Text::new(&meaning.definition).into()
    };

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
        .push(definition)
        .push(Text::new(" ").width(iced::Length::Fill))
        .push(cloze_status)
        .push(build_meaning_actions(meaning.id))
        .spacing(8)
        .align_y(iced::Alignment::Center);

    // Tags row
    let tags_row = build_tags_row(state, model, meaning);

    // Collect cloze preview elements (owned)
    let cloze_preview_items: Vec<Element<'a, WordsMessage>> = model
        .cloze_registry
        .iter_by_meaning_id(meaning.id)
        .take(2)
        .map(|(cloze_id, cloze)| {
            let is_selected = words_ui.is_cloze_selected(*cloze_id);
            let text = cloze.render_blanks();
            Row::new()
                .push(svg_checkbox(
                    is_selected,
                    WordsMessage::ToggleClozeSelection(*cloze_id),
                ))
                .push(Text::new(text).size(11))
                .spacing(2)
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
fn build_meaning_actions<'a>(meaning_id: Uuid) -> Element<'a, WordsMessage> {
    // Edit icon
    let edit_icon_handle = assets::get_svg("edit_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let edit_icon = svg(edit_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    // Delete icon
    let delete_icon_handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let delete_icon = svg(delete_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    Row::new()
        .push(
            Button::new(edit_icon)
                .style(button::secondary)
                .padding([2, 6])
                .on_press(WordsMessage::EditMeaningStart(meaning_id)),
        )
        .push(
            Button::new(delete_icon)
                .style(button::danger)
                .padding([2, 6])
                .on_press(WordsMessage::DeleteMeaning(meaning_id)),
        )
        .spacing(2)
        .into()
}

/// Build the tags row for a meaning.
fn build_tags_row<'a>(
    state: &'a MainWindowState,
    model: &'a Model,
    meaning: &'a crate::models::Meaning,
) -> Element<'a, WordsMessage> {
    let words_ui = &state.words_ui;

    // Tag chips
    let mut tag_chips: Vec<Element<'a, WordsMessage>> = meaning
        .tag_ids
        .iter()
        .filter_map(|tag_id| model.tag_registry.get(*tag_id))
        .map(|tag| {
            Button::new(Text::new(&tag.name).size(11))
                .style(button::secondary)
                .padding([2, 6])
                .on_press(WordsMessage::RemoveTagFromMeaning(meaning.id, tag.id))
                .into()
        })
        .collect();

    // Add tag button
    tag_chips.push(
        Button::new(Text::new("+").size(11))
            .style(button::secondary)
            .padding([2, 6])
            .on_press(WordsMessage::ShowTagDropdown(meaning.id))
            .into(),
    );

    // Tag dropdown if active
    let tag_dropdown: Option<Element<'a, WordsMessage>> =
        if let Some(ref dropdown) = words_ui.tag_dropdown {
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
        .on_input(WordsMessage::TagSearchChanged)
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
                .on_press(WordsMessage::AddTagToMeaning(
                    if let TagDropdownTarget::SingleMeaning(mid) = dropdown.target {
                        mid
                    } else {
                        Uuid::nil()
                    },
                    tag.id,
                ))
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
                .on_press(WordsMessage::QuickCreateTag(
                    if let TagDropdownTarget::SingleMeaning(mid) = dropdown.target {
                        mid
                    } else {
                        Uuid::nil()
                    },
                    search,
                ))
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
    words_ui: &'a super::state::WordsUiState,
    model: &'a Model,
) -> Element<'a, WordsMessage> {
    let meaning_selected_count = words_ui.selected_count();
    let cloze_selected_count = words_ui.selected_cloze_count();

    // Check for cloze selection first
    if cloze_selected_count > 0 {
        let selection_info = Text::new(format!("☑ {} clozes selected", cloze_selected_count));

        // Export dropdown
        let export_options = ExportKind::VARIANTS;
        let export_dropdown = PickList::new(export_options.to_vec(), None::<&ExportKind>, |kind| {
            WordsMessage::ExportSelected(kind)
        })
        .width(iced::Length::Fixed(140.0))
        .placeholder("Export");

        let delete_btn = Button::new(Text::new("Delete Clozes"))
            .style(button::danger)
            .padding([8, 16])
            .on_press(WordsMessage::DeleteSelectedClozes);

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
        // Show word input when nothing selected
        let word_input = TextInput::new("Add new word...", &words_ui.search_query)
            .on_input(WordsMessage::SearchChanged)
            .on_submit(WordsMessage::CreateWord(words_ui.search_query.clone()))
            .width(iced::Length::Fill)
            .padding(8);

        let add_btn = Button::new(Text::new("Add Word"))
            .style(button::primary)
            .padding([8, 16])
            .on_press(WordsMessage::CreateWord(words_ui.search_query.clone()));

        return Row::new().push(word_input).push(add_btn).spacing(10).into();
    }

    // Selection info for meanings
    let selection_info = Text::new(format!("☑ {} meanings selected", meaning_selected_count));

    // Batch tag dropdown if active
    let tag_btn: Element<'a, WordsMessage> = if let Some(ref dropdown) = words_ui.tag_dropdown {
        match dropdown.target {
            TagDropdownTarget::SelectedMeanings => Row::new()
                .push(
                    Button::new(Text::new("Add Tag ▾"))
                        .style(button::primary)
                        .padding([8, 16]),
                )
                .push(build_batch_tag_dropdown(dropdown, model))
                .spacing(2)
                .into(),
            _ => Button::new(Text::new("Add Tag"))
                .style(button::secondary)
                .padding([8, 16])
                .on_press(WordsMessage::ShowBatchTagDropdown)
                .into(),
        }
    } else {
        Button::new(Text::new("Add Tag"))
            .style(button::secondary)
            .padding([8, 16])
            .on_press(WordsMessage::ShowBatchTagDropdown)
            .into()
    };

    let queue_btn = Button::new(Text::new("Queue"))
        .style(button::primary)
        .padding([8, 16])
        .on_press(WordsMessage::QueueSelected);

    let delete_btn = Button::new(Text::new("Delete"))
        .style(button::danger)
        .padding([8, 16])
        .on_press(WordsMessage::DeleteSelected);

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
        .on_input(WordsMessage::TagSearchChanged)
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
                .on_press(WordsMessage::AddTagToSelected(tag.id))
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
