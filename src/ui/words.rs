use crate::Message;
use crate::registry::{ClozeRegistry, MeaningRegistry, TagRegistry, WordRegistry};
use crate::state::ui::MeaningInputState;
use iced::Element;
use iced::widget::{Button, Column, Row, Text, TextInput, button};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub fn view<'state>(
    word_registry: &'state WordRegistry,
    meaning_registry: &'state MeaningRegistry,
    cloze_registry: &'state ClozeRegistry,
    tag_registry: &'state TagRegistry,
    input: &str,
    tag_filter: &str,
    selected_word_ids: &'state HashSet<Uuid>,
    selected_meaning_ids: &'state HashSet<Uuid>,
    expanded_word_ids: &'state HashSet<Uuid>,
    meaning_inputs: &'state HashMap<Uuid, MeaningInputState>,
    cloze_inputs: &'state HashMap<Uuid, String>,
    active_tag_dropdown: &'state Option<Uuid>,
    tag_search_input: &str,
) -> Element<'state, crate::Message> {
    let all_words: Vec<_> = word_registry.iter().map(|(_, w)| w).collect();
    let all_tags: Vec<_> = tag_registry.iter().map(|(_, t)| t).collect();
    let selected_count = selected_word_ids.len();

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
                        .get_by_id(*meaning_id)
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

    let queue_btn = if selected_count > 0 {
        Button::new(Text::new(format!("Add to Queue ({})", selected_count)))
            .style(button::primary)
            .padding([8, 16])
            .on_press(Message::QueueSelected)
    } else {
        Button::new(Text::new(format!("Add to Queue ({})", selected_count)))
            .style(button::secondary)
            .padding([8, 16])
    };

    let delete_btn = if selected_count > 0 {
        Button::new(Text::new(format!("Delete ({})", selected_count)))
            .style(button::danger)
            .padding([8, 16])
            .on_press(Message::DeleteSelected)
    } else {
        Button::new(Text::new(format!("Delete ({})", selected_count)))
            .style(button::secondary)
            .padding([8, 16])
    };

    let selection_bar = Row::new()
        .push(select_all_btn)
        .push(deselect_all_btn)
        .push(Text::new("").width(iced::Length::Fill))
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
        .filter_map(|word_id| word_registry.get_by_id(*word_id))
        .map(|word| {
            let is_selected = selected_word_ids.contains(&word.id);
            let is_expanded = expanded_word_ids.contains(&word.id);
            let meaning_count = word.meaning_ids.len();

            let select_checkbox = Button::new(Text::new(if is_selected { "[x]" } else { "[ ]" }))
                .style(button::secondary)
                .padding([2, 6])
                .on_press(Message::ToggleWord(word.id))
                .width(iced::Length::Fixed(50.0));

            let expand_icon = if is_expanded { "▼" } else { "▶" };
            let expand_btn = Button::new(Text::new(expand_icon))
                .style(button::secondary)
                .padding([2, 6])
                .on_press(Message::ToggleWordExpand(word.id))
                .width(iced::Length::Fixed(30.0));

            let delete_word_btn = Button::new(Text::new("Delete"))
                .style(button::danger)
                .padding([2, 6])
                .on_press(Message::DeleteWord(word.id));

            let word_row = Row::new()
                .push(select_checkbox)
                .push(expand_btn)
                .push(Text::new(word.content.clone()).size(18))
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
                    let pos_input = TextInput::new("POS...", &input_state.pos)
                        .on_input(move |v| Message::MeaningPosInputChanged(word.id, v))
                        .width(iced::Length::Fixed(80.0));

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
                                .push(pos_input)
                                .push(def_input)
                                .push(save_btn)
                                .push(cancel_btn)
                                .spacing(5),
                        )
                        .push(iced::widget::rule::horizontal(1));
                }

                // List meanings for this word
                for meaning_id in &word.meaning_ids {
                    if let Some(meaning) = meaning_registry.get_by_id(*meaning_id) {
                        let cloze_vec: Vec<_> =
                            cloze_registry.iter_by_meaning_id(meaning.id).collect();

                        let is_meaning_selected = selected_meaning_ids.contains(&meaning.id);
                        let meaning_checkbox =
                            Button::new(Text::new(if is_meaning_selected { "[x]" } else { "[ ]" }))
                                .style(button::secondary)
                                .padding([2, 6])
                                .on_press(Message::ToggleMeaning(meaning.id))
                                .width(iced::Length::Fixed(30.0));

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
                                .filter_map(|tag_id| tag_registry.get_by_id(*tag_id))
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
                            cloze_vec.clone().into_iter().map(|(id, cloze)| {
                                Row::new()
                                    .push(Text::new(cloze.render_blanks()))
                                    .push(
                                        Button::new(Text::new("×"))
                                            .style(button::secondary)
                                            .padding([2, 6])
                                            .on_press(Message::DeleteCloze(*id)),
                                    )
                                    .spacing(5)
                                    .into()
                            }),
                        )
                        .spacing(5)
                        .into();

                        // Cloze input
                        let cloze_input_text =
                            cloze_inputs.get(&meaning.id).cloned().unwrap_or_default();
                        let add_cloze_row = Row::new()
                            .push(
                                TextInput::new("Add cloze...", &cloze_input_text)
                                    .on_input(move |v| Message::ClozeInputChanged(meaning.id, v))
                                    .width(iced::Length::Fill),
                            )
                            .push(
                                Button::new(Text::new("Add"))
                                    .style(button::primary)
                                    .padding([4, 8])
                                    .on_press(Message::CreateCloze(
                                        meaning.id,
                                        cloze_input_text.clone(),
                                    )),
                            );

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
                            let search_input =
                                TextInput::new("Search or create tag...", tag_search_input)
                                    .on_input(move |v| {
                                        Message::AddTagToMeaningSearch(meaning.id, v)
                                    })
                                    .width(iced::Length::Fill);

                            let filtered_tags: Vec<_> = all_tags
                                .iter()
                                .filter(|tag| {
                                    !meaning.tag_ids.contains(&tag.id)
                                        && tag
                                            .name
                                            .to_lowercase()
                                            .contains(&tag_search_input.to_lowercase())
                                })
                                .collect();

                            let tag_items: Vec<Element<_>> = filtered_tags
                                .iter()
                                .map(|tag| {
                                    Button::new(Text::new(&tag.name))
                                        .on_press(Message::AddTagToMeaning(meaning.id, tag.id))
                                        .width(iced::Length::Fill)
                                        .into()
                                })
                                .collect();

                            let create_info = if !tag_search_input.trim().is_empty() {
                                Text::new(format!("[+] Create: \"{}\"", tag_search_input.trim()))
                                    .size(12)
                            } else {
                                Text::new("Enter a tag name").size(12)
                            };

                            Some(
                                Column::new()
                                    .push(search_input)
                                    .push(iced::widget::rule::horizontal(1))
                                    .push(create_info)
                                    .push(Column::with_children(tag_items).spacing(5))
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
                                            .push(add_cloze_row)
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
            TextInput::new("Enter word...", input)
                .on_input(Message::WordsInputChanged)
                .on_submit(Message::CreateWord(input.to_string()))
                .width(iced::Length::Fill),
        )
        .push(
            Button::new(Text::new("Add Word"))
                .style(button::primary)
                .padding([8, 16])
                .on_press(Message::CreateWord(input.to_string())),
        );

    let main_column = Column::new()
        .push(Text::new("Words").size(24))
        .push(selection_bar)
        .push(filter_section)
        .push(input_section)
        .push(iced::widget::rule::horizontal(1))
        .push(
            iced::widget::scrollable(Column::with_children(word_items).spacing(10))
                .height(iced::Length::Fill),
        )
        .spacing(10);

    main_column.into()
}
