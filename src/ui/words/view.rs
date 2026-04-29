use crate::assets;
use crate::models::types::{MeaningId, WordId};
use crate::query::SortType;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::layout::breakpoint::Breakpoint;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::widgets::{CheckboxState, svg_checkbox};
use crate::ui::words::manager::{TagDropdownState, TagDropdownTarget};
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::Element;
use iced::widget::Space;
use iced::widget::{Button, Column, Container, PickList, Row, Text, TextInput, container, svg};

use uuid::Uuid;

// Renders the words panel.
pub fn view<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    theme: AppTheme,
    breakpoint: Breakpoint,
) -> Element<'a, WordsMessage, AppTheme> {
    let (left_ratio, right_ratio) = breakpoint.column_ratio();

    // Search and filter bar
    let search_bar = build_search_bar(words_state, model, breakpoint);

    // Get theme colors
    let colors = theme.colors();

    // Word tree (left panel)
    let word_tree = build_word_tree(words_state, model, theme);

    if breakpoint.is_single_column() {
        // Mobile: single column layout (word tree only, no detail panel)
        Column::new()
            .push(search_bar)
            .push(iced::widget::rule::horizontal(1))
            .push(iced::widget::scrollable(word_tree).height(iced::Length::Fill))
            .push(build_action_bar(words_state, model, theme))
            .spacing(Spacing::DEFAULT.s2)
            .padding(Spacing::DEFAULT.s2)
            .height(iced::Length::Fill)
            .into()
    } else {
        // Tablet/Desktop: two-column layout
        let left_panel = Column::new()
            .push(search_bar)
            .push(iced::widget::rule::horizontal(1))
            .push(iced::widget::scrollable(word_tree).height(iced::Length::Fill))
            .push(build_action_bar(words_state, model, theme))
            .spacing(Spacing::DEFAULT.s2)
            .padding(Spacing::DEFAULT.s2)
            .width(iced::Length::FillPortion((left_ratio * 10.0) as u16));

        // Detail panel (right panel)
        let right_panel = Container::new(crate::ui::words::detail_view::view(
            words_state.panel.state(),
            &words_state.panel.word_buffer,
            &words_state.panel.meaning_buffer,
            model,
            theme,
        ))
        .width(iced::Length::FillPortion((right_ratio * 10.0) as u16))
        .height(iced::Length::Fill)
        .style(move |_| container::Style {
            background: Some(colors.semantic.surface.raised.into()),
            ..Default::default()
        });

        // Two-column layout
        Row::new()
            .push(left_panel)
            .push(right_panel)
            .spacing(Spacing::DEFAULT.xs2)
            .height(iced::Length::Fill)
            .into()
    }
}

/// Build the search and filter bar.
fn build_search_bar<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    breakpoint: Breakpoint,
) -> Element<'a, WordsMessage, AppTheme> {
    let query = &words_state.search.query;
    let suggestion = if !query.is_empty() {
        words_state.search.get_suggestion(&model.word_registry)
    } else {
        None
    };

    let mut search_input =
        crate::ui::widgets::rich_text_input::RichTextInput::new("Search words or definitions...")
            .value(query)
            .on_input(WordsMessage::SearchQueryChanged)
            .on_submit(WordsMessage::SuggestionAccepted)
            .width(iced::Length::Fill)
            .padding(Spacing::DEFAULT.s);

    if let Some(sug) = suggestion {
        search_input = search_input.ghost_text(sug);
    }

    let search_with_ghost: Element<'a, WordsMessage, AppTheme> = Element::new(search_input);

    let sort_width = if breakpoint.is_single_column() {
        iced::Length::Fixed(90.0)
    } else {
        iced::Length::Fixed(110.0)
    };
    let sort_picker = PickList::new(
        SortType::variants(),
        Some(words_state.search.sort),
        WordsMessage::SortTypeChanged,
    )
    .width(sort_width)
    .placeholder("Sort");

    let clear_btn = if words_state.search.has_active_filters() {
        Button::new(Text::new("Clear"))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::FiltersCleared)
    } else {
        Button::new(Text::new("Clear"))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
    };

    Row::new()
        .push(search_with_ghost)
        .push(sort_picker)
        .push(clear_btn)
        .spacing(Spacing::DEFAULT.s2)
        .align_y(iced::Alignment::Center)
        .into()
}

/// Build the word tree.
fn build_word_tree<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    // Execute the query using the new SearchManager
    // Note: We need to clone the search state because we can't borrow mutably in a view function
    // The actual caching happens in the update loop
    let results = words_state.search.get_results();

    let word_nodes: Vec<Element<'a, WordsMessage, AppTheme>> = match results {
        Some(results) => results
            .iter()
            .filter_map(|(word_id, _)| model.word_registry.get(*word_id))
            .map(|word| build_word_node(words_state, model, word, theme))
            .collect(),
        None => {
            // No results cached yet, show all words
            model
                .word_registry
                .iter()
                .map(|(_, word)| build_word_node(words_state, model, word, theme))
                .collect()
        }
    };

    if word_nodes.is_empty() {
        Column::new()
            .push(
                Container::new(Text::new("No words found. Add a word to get started."))
                    .center_x(iced::Length::Fill)
                    .padding(Spacing::DEFAULT.l2),
            )
            .into()
    } else {
        Column::with_children(word_nodes)
            .spacing(Spacing::DEFAULT.xs2)
            .into()
    }
}

/// Build a single word node with its meanings.
fn build_word_node<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    word: &'a crate::models::Word,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let is_expanded = words_state.expansion.is_expanded(word.id);
    let is_selected = words_state.selection.is_word_selected(word);
    let is_partial = words_state.selection.is_word_partial(word);

    // Get theme colors
    let colors = theme.colors();

    // Expand/collapse icon (as button)
    let expand_icon_name = if is_expanded {
        "keyboard_arrow_down_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    } else {
        "keyboard_arrow_right_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg"
    };
    let expand_icon_handle = assets::get_svg(expand_icon_name)
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let expand_icon: Element<'a, WordsMessage, AppTheme> = Button::new(
        svg(expand_icon_handle)
            .width(iced::Length::Fixed(16.0))
            .height(iced::Length::Fixed(16.0)),
    )
    .style(button::secondary)
    .padding(ButtonSize::Small.to_iced_padding())
    .on_press(if is_expanded {
        WordsMessage::WordCollapsed(word.id)
    } else {
        WordsMessage::WordExpanded(word.id)
    })
    .into();

    // Checkbox state
    let checkbox: Element<'a, WordsMessage, AppTheme> = if word.meaning_ids.is_empty() {
        let radio_handle =
            assets::get_svg("radio_button_unchecked_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
                .map(svg::Handle::from_memory)
                .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
        Button::new(
            svg(radio_handle)
                .width(iced::Length::Fixed(20.0))
                .height(iced::Length::Fixed(20.0)),
        )
        .style(button::tertiary)
        .padding([2, 6])
        .width(iced::Length::Fixed(30.0))
        .into()
    } else if is_partial {
        svg_checkbox(
            CheckboxState::Indeterminate,
            WordsMessage::WordToggled(word.id),
            theme,
        )
    } else {
        svg_checkbox(is_selected, WordsMessage::WordToggled(word.id), theme)
    };

    // Word content (display only - not editable)
    let word_content: Element<'a, WordsMessage, AppTheme> =
        Button::new(Text::new(&word.content).size(FontSize::Subtitle.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::WordSelected(word.id))
            .into();

    // Meaning count badge
    // let meaning_count_badge = count_badge::<WordsMessage>(word.meaning_ids.len(), theme);

    // Word header row
    let word_header = Row::new()
        .push(expand_icon)
        .push(checkbox)
        .push(word_content)
        // .push(meaning_count_badge)
        .push(Space::new())
        .push(build_word_actions(word.id))
        .spacing(Spacing::DEFAULT.s)
        .align_y(iced::Alignment::Center);

    // Build expanded content if needed
    if is_expanded {
        let mut content = Column::new()
            .push(word_header)
            .push(iced::widget::rule::horizontal(1))
            .spacing(Spacing::DEFAULT.xs2);

        // Add meaning button (opens detail panel)
        content = content.push(
            Button::new(Text::new("+ Add Meaning"))
                .style(button::primary)
                .padding(ButtonSize::Medium.to_iced_padding())
                .on_press(WordsMessage::MeaningAddStarted { word_id: word.id }),
        );

        // Meaning nodes
        for meaning_id in &word.meaning_ids {
            if let Some(meaning) = model.meaning_registry.get(*meaning_id) {
                content = content.push(build_meaning_node(words_state, model, meaning, theme));
            }
        }

        Container::new(content)
            .padding(Spacing::DEFAULT.s)
            .style(move |_| container::Style {
                background: Some(colors.semantic.surface.raised.into()),
                border: iced::Border {
                    color: colors.semantic.border.default,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
            .into()
    } else {
        Container::new(word_header)
            .padding(Spacing::DEFAULT.s)
            .style(move |_| container::Style {
                background: Some(colors.semantic.surface.raised.into()),
                border: iced::Border {
                    color: colors.semantic.border.default,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}

/// Build word action buttons.
fn build_word_actions<'a>(word_id: WordId) -> Element<'a, WordsMessage, AppTheme> {
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
        .on_press(WordsMessage::WordDeleted(word_id))
        .into()
}

/// Build a meaning node with its clozes.
fn build_meaning_node<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    meaning: &'a crate::models::Meaning,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let is_selected = words_state.selection.is_meaning_selected(meaning.id);
    let cloze_count = model.cloze_registry.iter_by_meaning_id(meaning.id).count();
    let colors = theme.colors();

    // Checkbox
    let checkbox = svg_checkbox(is_selected, WordsMessage::MeaningToggled(meaning.id), theme);

    // POS badge
    // let pos_badge = pos_badge::<WordsMessage>(meaning.pos, theme);

    // CEFR level badge (if set)
    let cefr_badge =
    // if let Some(cefr) = meaning.cefr_level {
    //     cefr_badge::<WordsMessage>(cefr, theme)
    // } else
     {
        Container::new(Text::new(""))
    };

    // Definition - clickable to toggle detail panel
    let definition: Element<'a, WordsMessage, AppTheme> =
        Button::new(Text::new(&meaning.definition).size(FontSize::Body.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::MeaningSelected(meaning.id))
            .into();

    // Cloze status indicator
    let cloze_status_text = if cloze_count > 0 {
        format!("{} clozes", cloze_count)
    } else {
        "no clozes".to_string()
    };
    let cloze_status = Text::new(cloze_status_text)
        .size(FontSize::Caption.px())
        .color(colors.semantic.text.tertiary);

    // Meaning header
    let meaning_header = Row::new()
        .push(checkbox)
        // .push(pos_badge)
        .push(cefr_badge)
        .push(definition)
        .push(Space::new())
        .push(cloze_status)
        .push(build_meaning_actions(meaning.id))
        .spacing(Spacing::DEFAULT.xs)
        .align_y(iced::Alignment::Center);

    // Tags row
    let tags_row = build_tags_row(words_state, model, meaning, theme);

    // Collect cloze preview elements (owned) - clickable to toggle detail panel
    let cloze_preview_items: Vec<Element<'a, WordsMessage, AppTheme>> = model
        .cloze_registry
        .iter_by_meaning_id(meaning.id)
        .take(2)
        .map(|(cloze_id, cloze)| {
            let text = cloze.render_blanks();
            Button::new(Text::new(text).size(FontSize::Caption.px()))
                .style(button::tertiary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(WordsMessage::ClozeSelected(*cloze_id))
                .into()
        })
        .collect();

    // Build column
    let mut column = Column::new()
        .push(meaning_header)
        .push(tags_row)
        .spacing(Spacing::DEFAULT.xs2)
        .padding([Spacing::DEFAULT.xs2, Spacing::DEFAULT.s2]);

    // Add cloze previews if any
    if !cloze_preview_items.is_empty() {
        column = column.push(
            Column::with_children(cloze_preview_items)
                .spacing(Spacing::DEFAULT.xxs)
                .padding([Spacing::DEFAULT.xxs, Spacing::DEFAULT.s2]),
        );
    }

    column.into()
}

/// Build meaning action buttons.
fn build_meaning_actions<'a>(meaning_id: MeaningId) -> Element<'a, WordsMessage, AppTheme> {
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
        .on_press(WordsMessage::MeaningDeleted(meaning_id))
        .into()
}

/// Build the tags row for a meaning.
fn build_tags_row<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    meaning: &'a crate::models::Meaning,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    // Tag chips
    let mut tag_chips: Vec<Element<'a, WordsMessage, AppTheme>> = meaning
        .tag_ids
        .iter()
        .filter_map(|tag_id| model.tag_registry.get(*tag_id))
        .map(|tag| Button::new(Text::new(tag.name.clone())).into())
        .collect();

    // Add tag button
    tag_chips.push(
        Button::new(Text::new("+").size(FontSize::Caption.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::TagDropdownOpened {
                for_meaning: meaning.id,
            })
            .into(),
    );

    // Tag dropdown if active
    let tag_dropdown: Option<Element<'a, WordsMessage, AppTheme>> =
        if let Some(ref dropdown) = words_state.panel.tag_dropdown() {
            match dropdown.target {
                TagDropdownTarget::SingleMeaning(mid) if mid == meaning.id => {
                    Some(build_tag_dropdown(dropdown, model, theme))
                }
                _ => None,
            }
        } else {
            None
        };

    let mut row = Row::new().extend(tag_chips).spacing(Spacing::DEFAULT.xxs);

    if let Some(dropdown) = tag_dropdown {
        row = row.push(dropdown);
    }

    row.into()
}

/// Build tag dropdown.
fn build_tag_dropdown<'a>(
    dropdown: &'a TagDropdownState,
    model: &'a Model,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    // Get theme colors
    let colors = theme.colors();

    let search = TextInput::new("Search or create...", &dropdown.search)
        .on_input(WordsMessage::TagSearchChanged)
        .width(iced::Length::Fixed(150.0))
        .padding(Spacing::DEFAULT.xs);

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

    let mut tag_items: Vec<Element<'a, WordsMessage, AppTheme>> = filtered_tags
        .iter()
        .map(|tag| {
            let meaning_id = if let TagDropdownTarget::SingleMeaning(mid) = dropdown.target {
                mid
            } else {
                MeaningId(Uuid::nil())
            };
            Button::new(Text::new(&tag.name).size(FontSize::Footnote.px()))
                .width(iced::Length::Fill)
                .on_press(WordsMessage::TagAddedToMeaning {
                    meaning_id,
                    tag_id: tag.id,
                })
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
        let meaning_id = if let TagDropdownTarget::SingleMeaning(mid) = dropdown.target {
            mid
        } else {
            MeaningId(Uuid::nil())
        };
        tag_items.push(
            Button::new(Text::new(format!("Create \"{}\"", search)).size(FontSize::Footnote.px()))
                .width(iced::Length::Fill)
                .on_press(WordsMessage::TagQuickCreated {
                    meaning_id,
                    name: search,
                })
                .into(),
        );
    }

    Container::new(
        Column::new()
            .push(search)
            .extend(tag_items)
            .spacing(Spacing::DEFAULT.xs)
            .padding(Spacing::DEFAULT.xs2),
    )
    .width(iced::Length::Fixed(170.0))
    .style(move |_| container::Style {
        background: Some(colors.semantic.surface.raised.into()),
        border: iced::Border {
            color: colors.semantic.border.default,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
    .into()
}

/// Build batch tag dropdown for selected meanings.
fn build_batch_tag_dropdown<'a>(
    dropdown: &'a TagDropdownState,
    model: &'a Model,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    // Get theme colors
    let colors = theme.colors();

    let search = TextInput::new("Search...", &dropdown.search)
        .on_input(WordsMessage::TagSearchChanged)
        .width(iced::Length::Fixed(150.0))
        .padding(Spacing::DEFAULT.xs);

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

    let tag_items: Vec<Element<'a, WordsMessage, AppTheme>> = filtered_tags
        .iter()
        .map(|tag| {
            Button::new(Text::new(&tag.name).size(FontSize::Footnote.px()))
                .width(iced::Length::Fill)
                .on_press(WordsMessage::TagAddedToSelected { tag_id: tag.id })
                .into()
        })
        .collect();

    Container::new(
        Column::new()
            .push(search)
            .extend(tag_items)
            .spacing(Spacing::DEFAULT.xs)
            .padding(Spacing::DEFAULT.xs2),
    )
    .width(iced::Length::Fixed(170.0))
    .style(move |_| container::Style {
        background: Some(colors.semantic.surface.raised.into()),
        border: iced::Border {
            color: colors.semantic.border.default,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    })
    .into()
}

/// Build the contextual action bar (shows when items selected).
fn build_action_bar<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    theme: AppTheme,
) -> Element<'a, WordsMessage, AppTheme> {
    let meaning_selected_count = words_state.selection.meaning_count();
    let cloze_selected_count = words_state.selection.cloze_count();

    if cloze_selected_count > 0 {
        let selection_info = Text::new(format!("{} clozes selected", cloze_selected_count))
            .size(FontSize::Body.px());

        let export_btn = Button::new(Text::new("Export").size(FontSize::Body.px()))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::ExportPlaintext);

        let delete_btn = Button::new(Text::new("Delete Clozes").size(FontSize::Body.px()))
            .style(button::danger)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::ClozesDeleted);

        return Row::new()
            .push(selection_info)
            .push(Space::new())
            .push(export_btn)
            .push(delete_btn)
            .spacing(Spacing::DEFAULT.s2)
            .align_y(iced::Alignment::Center)
            .into();
    }

    if meaning_selected_count == 0 {
        let add_btn = Button::new(Text::new("+ Add Word").size(FontSize::Body.px()))
            .style(button::primary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(WordsMessage::NewWordStarted);

        return Row::new().push(add_btn).spacing(Spacing::DEFAULT.s2).into();
    }

    let selection_info = Text::new(format!("{} meanings selected", meaning_selected_count))
        .size(FontSize::Body.px());

    let tag_btn: Element<'a, WordsMessage, AppTheme> =
        if let Some(dropdown) = words_state.panel.tag_dropdown() {
            match dropdown.target {
                TagDropdownTarget::SelectedMeanings => Row::new()
                    .push(
                        Button::new(Text::new("Add Tag ▾").size(FontSize::Body.px()))
                            .style(button::primary)
                            .padding(ButtonSize::Standard.to_iced_padding()),
                    )
                    .push(build_batch_tag_dropdown(dropdown, model, theme))
                    .spacing(Spacing::DEFAULT.xxs)
                    .into(),
                _ => Button::new(Text::new("Add Tag").size(FontSize::Body.px()))
                    .style(button::secondary)
                    .padding(ButtonSize::Standard.to_iced_padding())
                    .on_press(WordsMessage::TagBatchDropdownOpened)
                    .into(),
            }
        } else {
            Button::new(Text::new("Add Tag").size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(WordsMessage::TagBatchDropdownOpened)
                .into()
        };

    let queue_btn = Button::new(Text::new("Queue").size(FontSize::Body.px()))
        .style(button::primary)
        .padding(ButtonSize::Standard.to_iced_padding())
        .on_press(WordsMessage::MeaningsQueuedForGeneration);

    let delete_btn = Button::new(Text::new("Delete").size(FontSize::Body.px()))
        .style(button::danger)
        .padding(ButtonSize::Standard.to_iced_padding())
        .on_press(WordsMessage::MeaningsDeleted);

    Row::new()
        .push(selection_info)
        .push(Space::new())
        .push(tag_btn)
        .push(queue_btn)
        .push(delete_btn)
        .spacing(Spacing::DEFAULT.s2)
        .align_y(iced::Alignment::Center)
        .into()
}
