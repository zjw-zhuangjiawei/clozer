//! Words view UI component.

use crate::message::Message;
use crate::models::PartOfSpeech;
use crate::state::Model;
use crate::window::{TagDropdownState, WindowState};
use iced::Element;
use iced::widget::{Button, Column, Container, PickList, Row, Text, TextInput, button, svg};
use strum::VariantArray;
use uuid::Uuid;

pub fn view<'state>(model: &'state Model, window: &'state WindowState) -> Element<'state, Message> {
    let all_words: Vec<_> = model.word_registry.iter().map(|(_, w)| w).collect();
    let all_tags: Vec<_> = model.tag_registry.iter().map(|(_, t)| t).collect();
    let selected_meaning_count = window.selected_meaning_ids.len();

    // Aliases for cleaner code
    let word_registry = &model.word_registry;
    let meaning_registry = &model.meaning_registry;
    let tag_registry = &model.tag_registry;
    let cloze_registry = &model.cloze_registry;
    let tag_filter = &window.words_ui.tag_filter;
    let meaning_inputs = &window.words_ui.meaning_inputs;
    let active_tag_dropdown = &window.words_ui.active_tag_dropdown;
    let meanings_tag_dropdown_state = &window.words_ui.meanings_tag_dropdown_state;
    let meanings_tag_search_input = &window.words_ui.meanings_tag_search_input;
    let meanings_tag_remove_search_input = &window.words_ui.meanings_tag_remove_search_input;

    // Filter words by tag filter
    let filtered_words: Vec<Uuid> = if tag_filter.is_empty() {
        all_words.iter().map(|w| w.id).collect()
    } else {
        let matching_tag_ids: Vec<Uuid> = all_tags
            .iter()
            .filter(|t| t.name.to_lowercase().contains(&tag_filter.to_lowercase()))
            .map(|t| t.id)
            .collect();

        all_words
            .iter()
            .filter(|word| {
                word.meaning_ids.iter().any(|meaning_id| {
                    meaning_registry
                        .get(*meaning_id)
                        .map(|m| m.tag_ids.iter().any(|tid| matching_tag_ids.contains(tid)))
                        .unwrap_or(false)
                })
            })
            .map(|w| w.id)
            .collect()
    };

    // Selection bar
    let select_all_btn = Button::new(Text::new("Select All"))
        .style(button::secondary)
        .padding([8, 16])
        .on_press(Message::SelectAllWords);

    let deselect_all_btn = Button::new(Text::new("Select None"))
        .style(button::secondary)
        .padding([8, 16])
        .on_press(Message::DeselectAllWords);

    let queue_btn = if selected_meaning_count > 0 {
        Button::new(Text::new(format!(
            "Add to Queue ({})",
            selected_meaning_count
        )))
        .style(button::primary)
        .padding([8, 16])
        .on_press(Message::QueueSelected)
    } else {
        Button::new(Text::new(format!(
            "Add to Queue ({})",
            selected_meaning_count
        )))
        .style(button::secondary)
        .padding([8, 16])
    };

    let delete_btn = if selected_meaning_count > 0 {
        Button::new(Text::new(format!("Delete ({})", selected_meaning_count)))
            .style(button::danger)
            .padding([8, 16])
            .on_press(Message::DeleteSelected)
    } else {
        Button::new(Text::new(format!("Delete ({})", selected_meaning_count)))
            .style(button::secondary)
            .padding([8, 16])
    };

    // Add Tag and Remove Tag buttons for selected meanings
    let add_tag_btn = if selected_meaning_count > 0 {
        Button::new(Text::new("Add Tag"))
            .style(button::secondary)
            .padding([8, 16])
            .on_press(Message::ToggleMeaningsAddTagDropdown)
    } else {
        Button::new(Text::new("Add Tag"))
            .style(button::secondary)
            .padding([8, 16])
    };

    let remove_tag_btn = if selected_meaning_count > 0 {
        Button::new(Text::new("Remove Tag"))
            .style(button::secondary)
            .padding([8, 16])
            .on_press(Message::ToggleMeaningsRemoveTagDropdown)
    } else {
        Button::new(Text::new("Remove Tag"))
            .style(button::secondary)
            .padding([8, 16])
    };

    // Compute common tags for Remove Tag dropdown
    let common_tag_ids: Vec<Uuid> = if selected_meaning_count > 0 {
        let mut common_tags: Option<std::collections::BTreeSet<Uuid>> = None;
        for &meaning_id in &window.selected_meaning_ids {
            if let Some(meaning) = meaning_registry.get(meaning_id) {
                if let Some(ref mut tags) = common_tags {
                    tags.retain(|t| meaning.tag_ids.contains(t));
                } else {
                    common_tags = Some(meaning.tag_ids.clone());
                }
            }
        }
        common_tags.unwrap_or_default().into_iter().collect()
    } else {
        vec![]
    };

    // Filter tags for Add Tag dropdown (tags not on all selected meanings)
    let tags_for_add: Vec<_> = if selected_meaning_count > 0 {
        all_tags
            .iter()
            .filter(|tag| {
                let on_all_meanings = window.selected_meaning_ids.iter().all(|&mid| {
                    meaning_registry
                        .get(mid)
                        .map(|m| m.tag_ids.contains(&tag.id))
                        .unwrap_or(false)
                });
                !on_all_meanings
                    && tag
                        .name
                        .to_lowercase()
                        .contains(&meanings_tag_search_input.to_lowercase())
            })
            .collect()
    } else {
        vec![]
    };

    // Filter common tags for Remove Tag dropdown
    let tags_for_remove: Vec<_> = common_tag_ids
        .iter()
        .filter_map(|tag_id| tag_registry.get(*tag_id))
        .filter(|tag| {
            tag.name
                .to_lowercase()
                .contains(&meanings_tag_remove_search_input.to_lowercase())
        })
        .collect();

    // Batch tag dropdown
    let batch_tag_dropdown: Option<Element<_>> = match meanings_tag_dropdown_state {
        TagDropdownState::Add => {
            let search = TextInput::new("Search tags...", meanings_tag_search_input)
                .on_input(Message::MeaningsTagSearchChanged)
                .width(iced::Length::Fill);
            let tag_items: Vec<Element<_>> = tags_for_add
                .iter()
                .map(|tag| {
                    Button::new(Text::new(&tag.name))
                        .on_press(Message::BatchAddTagToSelectedMeanings(tag.id))
                        .width(iced::Length::Fill)
                        .into()
                })
                .collect();
            let empty_msg = if tags_for_add.is_empty() {
                Text::new("No tags available").size(12)
            } else {
                Text::new("")
            };
            Some(
                Container::new(
                    Column::new()
                        .push(Text::new("Add Tag to Selected Meanings").size(14))
                        .push(iced::widget::rule::horizontal(1))
                        .push(search)
                        .push(empty_msg)
                        .push(Column::with_children(tag_items).spacing(5))
                        .spacing(5)
                        .padding(10),
                )
                .width(iced::Length::Fixed(250.0))
                .into(),
            )
        }
        TagDropdownState::Remove => {
            let search = TextInput::new("Search tags...", meanings_tag_remove_search_input)
                .on_input(Message::MeaningsTagRemoveSearchChanged)
                .width(iced::Length::Fill);
            let tag_items: Vec<Element<_>> = tags_for_remove
                .iter()
                .map(|tag| {
                    Button::new(Text::new(&tag.name))
                        .on_press(Message::BatchRemoveTagFromSelectedMeanings(tag.id))
                        .width(iced::Length::Fill)
                        .into()
                })
                .collect();
            let empty_msg = if tags_for_remove.is_empty() {
                Text::new("No common tags").size(12)
            } else {
                Text::new("")
            };
            Some(
                Container::new(
                    Column::new()
                        .push(Text::new("Remove Tag from Selected Meanings").size(14))
                        .push(iced::widget::rule::horizontal(1))
                        .push(search)
                        .push(empty_msg)
                        .push(Column::with_children(tag_items).spacing(5))
                        .spacing(5)
                        .padding(10),
                )
                .width(iced::Length::Fixed(250.0))
                .into(),
            )
        }
        TagDropdownState::None => None,
    };

    let selection_bar = Row::new()
        .push(select_all_btn)
        .push(deselect_all_btn)
        .push(Text::new("").width(iced::Length::Fill))
        .push(add_tag_btn)
        .push(remove_tag_btn)
        .push(queue_btn)
        .push(delete_btn)
        .spacing(10);

    // Tag filter input
    let filter_section = Row::new()
        .push(
            TextInput::new("Filter by tag...", tag_filter)
                .on_input(Message::WordsTagFilterChanged)
                .width(iced::Length::Fill),
        )
        .push(
            Button::new(Text::new("Clear"))
                .style(button::secondary)
                .padding([8, 16])
                .on_press_maybe(if !tag_filter.is_empty() {
                    Some(Message::WordsClearTagFilter)
                } else {
                    None
                }),
        )
        .spacing(10);

    // Word items
    let word_items: Vec<Element<_>> = filtered_words
        .iter()
        .filter_map(|word_id| word_registry.get(*word_id))
        .map(|word| {
            let is_selected = window.selected_word_ids.contains(&word.id);
            let is_expanded = window.expanded_word_ids.contains(&word.id);
            let meaning_count = word.meaning_ids.len();

            let select_checkbox = if is_selected {
                Button::new(
                    svg("assets/icon/check_box_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
                        .width(iced::Length::Fixed(20.0))
                        .height(iced::Length::Fixed(20.0)),
                )
                .style(button::secondary)
                .padding([2, 6])
                .on_press(Message::ToggleWord(word.id))
                .width(iced::Length::Fixed(30.0))
            } else {
                Button::new(
                    svg("assets/icon/check_box_outline_blank_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
                        .width(iced::Length::Fixed(20.0))
                        .height(iced::Length::Fixed(20.0)),
                )
                .style(button::secondary)
                .padding([2, 6])
                .on_press(Message::ToggleWord(word.id))
                .width(iced::Length::Fixed(30.0))
            };

            let word_text_btn = Button::new(Text::new(word.content.clone()).size(18))
                .style(button::secondary)
                .padding([2, 6])
                .on_press(Message::ToggleWordExpand(word.id));

            let delete_word_btn = Button::new(Text::new("Delete"))
                .style(button::danger)
                .padding([2, 6])
                .on_press(Message::DeleteWord(word.id));

            let word_row = Row::new()
                .push(select_checkbox)
                .push(word_text_btn)
                .push(Text::new(format!("({} meanings)", meaning_count)).size(12))
                .push(Text::new("").width(iced::Length::Fill))
                .push(delete_word_btn)
                .spacing(10)
                .align_y(iced::Alignment::Center);

            let mut word_column = Column::new().push(word_row).spacing(5).padding(10);

            // Expanded section with meanings
            if is_expanded {
                // Add Meaning button
                let add_meaning_btn = Button::new(Text::new("+ Add Meaning"))
                    .style(button::secondary)
                    .padding([4, 8])
                    .on_press(Message::ToggleMeaningInput(word.id));

                word_column = word_column.push(add_meaning_btn);

                // Meaning input form
                if let Some(input_state) = meaning_inputs.get(&word.id)
                    && input_state.visible
                {
                    let pos_options = PartOfSpeech::VARIANTS;
                    let pos_pick_list =
                        PickList::new(pos_options, Some(input_state.pos), move |pos| {
                            Message::MeaningPosSelected(word.id, pos)
                        })
                        .width(iced::Length::Fixed(120.0));

                    let def_input = TextInput::new("Definition...", &input_state.definition)
                        .on_input(move |v| Message::MeaningDefInputChanged(word.id, v))
                        .width(iced::Length::Fill);

                    let save_btn = Button::new(Text::new("Save"))
                        .style(button::primary)
                        .padding([4, 8])
                        .on_press(Message::SaveMeaning(word.id));

                    let cancel_btn = Button::new(Text::new("Cancel"))
                        .style(button::secondary)
                        .padding([4, 8])
                        .on_press(Message::CancelMeaningInput(word.id));

                    word_column = word_column
                        .push(
                            Row::new()
                                .push(pos_pick_list)
                                .push(def_input)
                                .push(save_btn)
                                .push(cancel_btn)
                                .spacing(5),
                        )
                        .push(iced::widget::rule::horizontal(1));
                }

                // List meanings for this word
                for meaning_id in &word.meaning_ids {
                    if let Some(meaning) = meaning_registry.get(*meaning_id) {
                        let cloze_vec: Vec<_> =
                            cloze_registry.iter_by_meaning_id(meaning.id).collect();

                        let is_meaning_selected = window.selected_meaning_ids.contains(&meaning.id);
                        let meaning_checkbox = if is_meaning_selected {
                            Button::new(
                                svg("assets/icon/check_box_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
                                    .width(iced::Length::Fixed(20.0))
                                    .height(iced::Length::Fixed(20.0)),
                            )
                            .style(button::secondary)
                            .padding([2, 6])
                            .on_press(Message::ToggleMeaning(meaning.id))
                            .width(iced::Length::Fixed(30.0))
                        } else {
                            Button::new(
                                svg("assets/icon/check_box_outline_blank_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
                                    .width(iced::Length::Fixed(20.0))
                                    .height(iced::Length::Fixed(20.0)),
                            )
                            .style(button::secondary)
                            .padding([2, 6])
                            .on_press(Message::ToggleMeaning(meaning.id))
                            .width(iced::Length::Fixed(30.0))
                        };

                        // Meaning header
                        let meaning_header = Row::new()
                            .push(meaning_checkbox)
                            .push(Text::new(format!(
                                "• {}: {}",
                                meaning.pos, meaning.definition
                            )))
                            .spacing(10);

                        // Tag chips
                        let tag_chips: Element<_> = Row::with_children(
                            meaning
                                .tag_ids
                                .iter()
                                .filter_map(|tag_id| tag_registry.get(*tag_id))
                                .map(|tag| {
                                    let tag_name = tag.name.clone();
                                    Button::new(Text::new(format!("[{}]", tag_name)))
                                        .style(button::secondary)
                                        .padding([2, 6])
                                        .on_press(Message::RemoveTagFromMeaning(meaning.id, tag.id))
                                        .into()
                                }),
                        )
                        .spacing(5)
                        .into();

                        // Cloze list
                        let cloze_list: Element<_> = Column::with_children(
                            cloze_vec.clone().into_iter().map(|(_id, cloze)| {
                                Row::new()
                                    .push(Text::new(cloze.render_blanks()))
                                    .spacing(5)
                                    .into()
                            }),
                        )
                        .spacing(5)
                        .into();

                        // Actions
                        let delete_meaning_btn = Button::new(Text::new("Delete"))
                            .style(button::danger)
                            .padding([2, 6])
                            .on_press(Message::DeleteMeaning(meaning.id));

                        let add_tag_btn = Button::new(Text::new("+ Tag"))
                            .style(button::secondary)
                            .padding([2, 6])
                            .on_press(Message::WordsMeaningToggleTagDropdown(meaning.id));

                        // Tag dropdown
                        let tag_dropdown: Option<Element<_>> = if *active_tag_dropdown
                            == Some(meaning.id)
                        {
                            let search_input = TextInput::new("Search or create tag...", "")
                                .on_input(move |v| Message::AddTagToMeaningSearch(meaning.id, v))
                                .width(iced::Length::Fill);

                            let create_info = Text::new("Enter a tag name").size(12);

                            Some(
                                Column::new()
                                    .push(search_input)
                                    .push(iced::widget::rule::horizontal(1))
                                    .push(create_info)
                                    .spacing(5)
                                    .padding(10)
                                    .into(),
                            )
                        } else {
                            None
                        };

                        word_column = word_column
                            .push(
                                Row::new()
                                    .push(Text::new(" ").width(iced::Length::Fixed(20.0)))
                                    .push(
                                        Column::new()
                                            .push(meaning_header)
                                            .push(
                                                Row::new()
                                                    .push(Text::new("Tags:").size(12))
                                                    .push(tag_chips)
                                                    .push(add_tag_btn)
                                                    .spacing(5),
                                            )
                                            .push(
                                                Text::new(format!("Clozes ({})", cloze_vec.len()))
                                                    .size(12),
                                            )
                                            .push(cloze_list)
                                            .push(delete_meaning_btn)
                                            .spacing(5),
                                    ),
                            )
                            .push(iced::widget::rule::horizontal(1));

                        if let Some(dropdown) = tag_dropdown {
                            word_column = word_column.push(dropdown);
                        }
                    }
                }
            }

            word_column.into()
        })
        .collect();

    // Input section
    let input_section = Row::new()
        .push(
            TextInput::new("Enter word...", &window.words_ui.word_input)
                .on_input(Message::WordsInputChanged)
                .on_submit(Message::CreateWord(window.words_ui.word_input.to_string()))
                .width(iced::Length::Fill),
        )
        .push(
            Button::new(Text::new("Add Word"))
                .style(button::primary)
                .padding([8, 16])
                .on_press(Message::CreateWord(window.words_ui.word_input.to_string())),
        );

    let main_column = Column::new()
        .push(Text::new("Words").size(24))
        .push(selection_bar)
        .push(filter_section)
        .push(input_section)
        .push(iced::widget::rule::horizontal(1));

    let main_column = if let Some(dropdown) = batch_tag_dropdown {
        main_column.push(dropdown)
    } else {
        main_column
    };

    let main_column = main_column
        .push(
            iced::widget::scrollable(Column::with_children(word_items).spacing(10))
                .height(iced::Length::Fill),
        )
        .spacing(10);

    main_column.into()
}
